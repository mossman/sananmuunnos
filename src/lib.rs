#[macro_use]
extern crate diesel;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use array_tool::vec::Intersect;
use serde::Serialize;
use unicode_segmentation::UnicodeSegmentation;
use xml::reader::{EventReader, XmlEvent};

pub mod models;
pub mod schema;

const VOWELS: &'static [&'static str] = &["a", "e", "i", "o", "u", "y", "å", "ä", "ö"];

#[derive(Serialize)]
pub struct SpoonMaps {
    prefixmap: HashMap<String, Vec<String>>,
    suffixmap: HashMap<String, Vec<String>>,
    altprefixmap: HashMap<String, Vec<String>>,
}

#[derive(Serialize)]
pub struct WordResult {
    pub rootword: String,
    pub endings: Vec<String>,
}

fn split_word(s: &str) -> (String, String, bool) {
    let mut prefix: String = String::new();
    let mut suffix: String = String::new();
    let mut prev: &str = "";
    let mut state = 0;
    let mut double_vowel = false;

    for grapheme in s.graphemes(true) {
        if state == 0 {
            // skip "-" at start
            if let &"-" = &grapheme {
                continue;
            }
            prefix.push_str(&grapheme);
            state = 1;
        } else if state == 1 {
            if VOWELS.contains(&prev) {
                if prev == grapheme {
                    double_vowel = true;
                    prefix.push_str(&grapheme);
                } else {
                    suffix.push_str(&grapheme);
                    state = 2
                }
            } else {
                prefix.push_str(&grapheme);
            }
        } else if state == 2 {
            suffix.push_str(&grapheme);
        }
        prev = grapheme;
    }
    (prefix, suffix, double_vowel)
}

fn check_double_vowel(prefix: &str) -> bool {
    let mut prev: &str = "";
    let mut same: bool = false;

    for grapheme in prefix.graphemes(true) {
        if grapheme == prev {
            same = true;
        } else {
            same = false;
        }
        prev = grapheme;
    }
    return same;
}

fn altlookup(word: &str) -> Option<String> {
    let mut altlookup = String::new();

    for grapheme in word.graphemes(true) {
        let translated = match grapheme {
            "ä" => &"a",
            "ö" => &"o",
            "a" => &"ä",
            "o" => &"ö",
            _ => grapheme,
        };
        altlookup.push_str(&translated);
    }
    if &altlookup != &word {
        Some(altlookup)
    } else {
        None
    }
}

fn prefix_candidates<'a>(
    suffix: &str,
    hashmap: &'a HashMap<String, Vec<String>>,
    double_vowel: bool,
) -> Vec<&'a String> {
    let mut matches: Vec<&String> = Vec::new();
    if hashmap.contains_key(suffix) {
        for prefix in hashmap.get(suffix).unwrap().iter() {
            let has_double_vowel = check_double_vowel(prefix);
            if double_vowel && has_double_vowel {
                matches.push(prefix);
            } else if !double_vowel && !has_double_vowel {
                matches.push(prefix);
            }
        }
    }
    matches
}

fn suffix_candidates<'a>(
    prefix: &str,
    hashmap: &'a HashMap<String, Vec<String>>,
) -> Vec<&'a String> {
    let mut matches: Vec<&String> = Vec::new();

    if hashmap.contains_key(prefix) {
        for suffix in hashmap.get(prefix).unwrap().iter() {
            matches.push(suffix)
        }
    }
    matches
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn squeeze(word: &str) -> String {
    word.graphemes(true)
        .collect::<Vec<&str>>()
        .split_last()
        .unwrap()
        .1
        .join("")
}

fn stretch(word: &str) -> String {
    format!("{}{}", word, word.graphemes(true).last().unwrap())
}

fn add_word_to_maps(
    word: &str, 
    prefixmap: &mut HashMap<String, Vec<String>>,
    suffixmap: &mut HashMap<String, Vec<String>>,
    altprefixmap: &mut HashMap<String, Vec<String>>
) {
    let (prefix, suffix, _double_vowel) = split_word(&word);
    suffixmap
        .entry(suffix.clone())
        .or_insert(Vec::new())
        .push(prefix.clone());

    let altsuffix = altlookup(&suffix);
    match altsuffix {
        Some(altsuffix) => {
            altprefixmap
                .entry(prefix.clone())
                .or_insert(Vec::new())
                .push(altsuffix);
        }
        None => altprefixmap
            .entry(prefix.clone())
            .or_insert(Vec::new())
            .push(suffix.clone()),
    }
    prefixmap.entry(prefix).or_insert(Vec::new()).push(suffix);
}

impl SpoonMaps {
    pub fn from_text(filename: &str) -> SpoonMaps {
        let mut prefixmap: HashMap<String, Vec<String>> = HashMap::new();
        let mut suffixmap: HashMap<String, Vec<String>> = HashMap::new();
        let mut altprefixmap: HashMap<String, Vec<String>> = HashMap::new();

        let file = match File::open(filename) {
            Err(why) => panic!("Could not open file {}", why),
            Ok(file) => file,
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(word) = line {
                add_word_to_maps(&word, &mut prefixmap, &mut suffixmap, &mut altprefixmap);
            }
        }
        SpoonMaps {
            prefixmap: prefixmap,
            suffixmap: suffixmap,
            altprefixmap: altprefixmap,
        }
    }

    pub fn from_kotus_xml(filename: &str) -> SpoonMaps {
        let mut prefixmap: HashMap<String, Vec<String>> = HashMap::new();
        let mut suffixmap: HashMap<String, Vec<String>> = HashMap::new();
        let mut altprefixmap: HashMap<String, Vec<String>> = HashMap::new();

        let file = match File::open(filename) {
            Err(why) => panic!("Could not open file {}", why),
            Ok(file) => file,
        };

        let file = BufReader::new(file);

        let parser = EventReader::new(file);
        let mut capture = false;
        let mut prev = 0;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    if name.local_name == "s" {
                        capture = true;
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.local_name == "s" {
                        capture = false;
                    }
                }
                Ok(XmlEvent::Characters(text)) => {
                    // Avoid dupes
                    if prev == hash(&text) {
                        continue;
                    }
                    if capture {
                        add_word_to_maps(&text, &mut prefixmap, &mut suffixmap, &mut altprefixmap);

                        prev = hash(&text);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        SpoonMaps {
            prefixmap: prefixmap,
            suffixmap: suffixmap,
            altprefixmap: altprefixmap,
        }
    }

    fn match_endings(
        &self,
        prefix: &str,
        dirty_prefix: &str,
        dirty_suffix_candidates: &Vec<&String>,
        dirty: &str,
    ) -> Vec<String> {
        let mut res = Vec::new();
        let candidates = suffix_candidates(&prefix, &self.prefixmap);

        for suffix in dirty_suffix_candidates.intersect(candidates) {
            if &format!("{}{}", dirty_prefix, suffix) == dirty {
                continue;
            }
            res.push(format!("{}{}", dirty_prefix, suffix));
        }
        res
    }

    fn match_alt_endings(
        &self,
        prefix: &str,
        dirty_prefix: &str,
        dirty_suffix_candidates: &Vec<&String>,
        dirty: &str,
    ) -> Vec<String> {
        let mut res = Vec::new();
        let candidates = suffix_candidates(&prefix, &self.prefixmap);

        for suffix in dirty_suffix_candidates.intersect(candidates) {
            // Switch suffix to alt form if needed
            let new_suffix = altlookup(&suffix);
            match new_suffix {
                Some(a) => {
                    if &format!("{}{}", dirty_prefix, a) == dirty {
                        continue;
                    }
                    res.push(format!("{}{}", dirty_prefix, a));
                }
                _ => {
                    res.push(format!("{}{}", dirty_prefix, suffix));
                }
            }
        }
        res
    }

    pub fn spoonerism(&self, dirty: &str) -> Vec<WordResult> {
        let dirty = &dirty.to_lowercase();

        let (dirty_prefix, dirty_suffix, double_vowel) = split_word(&dirty);

        let dirty_suffix_candidates = suffix_candidates(&dirty_prefix, &self.prefixmap);

        let mut results = Vec::new();

        let short_prefix = squeeze(&dirty_prefix);
        let long_prefix = stretch(&dirty_prefix);

        for prefix in prefix_candidates(&dirty_suffix, &self.suffixmap, double_vowel) {
            if &format!("{}{}", prefix, dirty_suffix) == dirty {
                continue;
            }

            let mut wordresult = WordResult {
                rootword: format!("{}{}", prefix, dirty_suffix),
                endings: Vec::new(),
            };

            wordresult.endings.extend(self.match_endings(
                &prefix,
                &dirty_prefix,
                &dirty_suffix_candidates,
                &dirty,
            ));

            if double_vowel && check_double_vowel(prefix) {
                let new_lookup = squeeze(&prefix);
                let short_suffix_candidates = suffix_candidates(&short_prefix, &self.prefixmap);

                wordresult.endings.extend(self.match_endings(
                    &new_lookup,
                    &short_prefix,
                    &short_suffix_candidates,
                    &dirty,
                ))
            } else if !double_vowel {
                let new_lookup = stretch(&prefix);
                let long_suffix_candidates = suffix_candidates(&long_prefix, &self.prefixmap);

                wordresult.endings.extend(self.match_endings(
                    &new_lookup,
                    &long_prefix,
                    &long_suffix_candidates,
                    &dirty,
                ))
            }
            if wordresult.endings.len() > 0 {
                results.push(wordresult);
            }
        }
        // Wonderful world of finnish phonetics
        if !altlookup(&dirty_suffix).is_none() && altlookup(&dirty_prefix).is_none() {
            let altdirty = altlookup(&dirty).unwrap();
            let (dirty_prefix, dirty_suffix, double_vowel) = split_word(&altdirty);

            let dirty_suffix_candidates = suffix_candidates(&dirty_prefix, &self.altprefixmap);

            for prefix in prefix_candidates(&dirty_suffix, &self.suffixmap, double_vowel) {
                let mut wordresult = WordResult {
                    rootword: format!("{}{}", prefix, dirty_suffix),
                    endings: Vec::new(),
                };

                wordresult.endings.extend(self.match_alt_endings(
                    &prefix,
                    &dirty_prefix,
                    &dirty_suffix_candidates,
                    &dirty,
                ));
                if wordresult.endings.len() > 0 {
                    results.push(wordresult);
                }
            }
            // Todo long tail. ää öö things.
        }
        results
    }

    pub fn check(&self, first: &str, second: &str) -> bool {
        // Simple check for now
        let (first_prefix, first_suffix, _) = split_word(&first.to_lowercase());
        let (second_prefix, second_suffix, _) = split_word(&second.to_lowercase());
        (self.prefixmap.contains_key(&first_prefix)
            || self.altprefixmap.contains_key(&first_prefix))
            && (self.prefixmap.contains_key(&second_prefix)
                || self.altprefixmap.contains_key(&second_prefix))
            && self.suffixmap.contains_key(&first_suffix)
            && self.suffixmap.contains_key(&second_suffix)
    }
}
