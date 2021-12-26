use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use array_tool::vec::Intersect;
use unicode_segmentation::UnicodeSegmentation;
use xml::reader::{EventReader, XmlEvent};
use serde::Serialize;

const VOWELS: &'static [&'static str] = &["a", "e", "i", "o", "u", "y", "å", "ä", "ö"];

fn split_word(s: &str) -> (String, String, bool) {
    let mut prefix: String = String::new();
    let mut suffix: String = String::new();
    let mut prev: &str = "";
    let mut state = 0;
    let mut double_vowel = false;

    for grapheme in s.graphemes(true).collect::<Vec<&str>>().iter() {
        if state == 0 {
            // skip "-" at start
            if let &"-" = &*grapheme {
                continue;
            }
            prefix.push_str(&grapheme);
            state = 1;
        } else if state == 1 {
            if VOWELS.contains(&prev) {
                if &prev == grapheme {
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
    let graphemes = prefix.graphemes(true).collect::<Vec<&str>>();

    for grapheme in graphemes.iter() {
        if grapheme == &prev {
            same = true;
        } else {
            same = false;
        }
        prev = grapheme;
    }
    return same;
}

fn prefix_candidates<'a>(
    suffix: &str,
    hashmap: &'a HashMap<String, Vec<String>>,
    double_vowel: bool,
) -> Vec<&'a String> {
    let mut matches: Vec<&String> = Vec::new();

    for prefix in hashmap.get(suffix).unwrap().iter() {
        let has_double_vowel = check_double_vowel(prefix);
        if double_vowel && has_double_vowel {
            matches.push(prefix);
        } else if !double_vowel && !has_double_vowel {
            matches.push(prefix);
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

#[derive(Serialize)]
pub struct SpoonMaps {
    prefixmap: HashMap<String, Vec<String>>,
    suffixmap: HashMap<String, Vec<String>>
}

#[derive(Serialize)]
pub struct WordResult {
    pub rootword: String,
    pub endings: Vec<String>
}

impl SpoonMaps {
    pub fn from_kotus_xml(filename: &str) -> SpoonMaps {
        let mut prefixmap: HashMap<String, Vec<String>> = HashMap::new();
        let mut suffixmap: HashMap<String, Vec<String>> = HashMap::new();
    
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
                        let (prefix, suffix, _double_vowel) = split_word(&text);
                        suffixmap
                            .entry(suffix.clone())
                            .or_insert(Vec::new())
                            .push(prefix.clone());
                        prefixmap.entry(prefix).or_insert(Vec::new()).push(suffix);
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
            suffixmap: suffixmap
        }
    }

    pub fn spoonerism(
        &self,
        dirty: &str,
    ) -> Vec<WordResult> {
        let (dirty_prefix, dirty_suffix, double_vowel) = split_word(&dirty.to_lowercase());
    
        let dirty_suffix_candidates = suffix_candidates(&dirty_prefix, &self.prefixmap);
    
        let mut results = Vec::new();

        for prefix in prefix_candidates(&dirty_suffix, &self.suffixmap, double_vowel) {
            if &format!("{}{}", prefix, dirty_suffix) == dirty {
                continue;
            }

            let mut wordresult = WordResult {
                rootword: format!("{}{}", prefix, dirty_suffix),
                endings: Vec::new()
            };

            let candidates = suffix_candidates(&prefix, &self.prefixmap);
    
            for suffix in dirty_suffix_candidates.intersect(candidates) {
                if &format!("{}{}", dirty_prefix, suffix) == dirty {
                    continue;
                }
                wordresult.endings.push(format!("{}{}", dirty_prefix, suffix));
            }
    
            if double_vowel && check_double_vowel(prefix) {
                let new_lookup = prefix
                    .graphemes(true)
                    .collect::<Vec<&str>>()
                    .split_last()
                    .unwrap()
                    .1
                    .join("");
    
                let candidates = suffix_candidates(&new_lookup, &self.prefixmap);
    
                let short_prefix = dirty_prefix
                    .graphemes(true)
                    .collect::<Vec<&str>>()
                    .split_last()
                    .unwrap()
                    .1
                    .join("");
                let short_suffix_candidates = suffix_candidates(&short_prefix, &self.prefixmap);
    
                for suffix in short_suffix_candidates.intersect(candidates) {
                    if &format!("{}{}", short_prefix, suffix) == dirty {
                        continue;
                    }
                    wordresult.endings.push(format!("{}{}", short_prefix, suffix));
                }
            } else if !double_vowel {
                let new_lookup = format!("{}{}", prefix, prefix.graphemes(true).last().unwrap());
    
                let candidates = suffix_candidates(&new_lookup, &self.prefixmap);
    
                let long_prefix = format!(
                    "{}{}",
                    dirty_prefix,
                    dirty_prefix.graphemes(true).last().unwrap()
                );
                let long_suffix_candidates = suffix_candidates(&long_prefix, &self.prefixmap);
    
                for suffix in long_suffix_candidates.intersect(candidates) {
                    if &format!("{}{}", long_prefix, suffix) == dirty {
                        continue;
                    }
                    wordresult.endings.push(format!("{}{}", long_prefix, suffix));
                }
            }
            if wordresult.endings.len() > 0 {
                results.push(wordresult)
            }
        }
        results
    }
    
}
