extern crate clap;

use clap::{Arg, App};
use sananmuunnos::SpoonMaps;

fn load_maps(filename: &str, ftype: &str) -> SpoonMaps {
    match ftype {
        "kotusxml" => SpoonMaps::from_kotus_xml(filename),
        "text" => SpoonMaps::from_text(filename),
        _ => panic!("Unknown match type")
    }
}

fn main() {
    let matches = App::new("sananmuunnos")
        .version("0.1")
        .author("antti.hayrynen@gmail.com")
        .arg(Arg::with_name("wordlist")
            .short('w')
            .long("wordlist")
            .value_name("FILE")
            .default_value("sanat.xml")
            .help("word list")
            .takes_value(true))
        .arg(Arg::with_name("format")
            .short('f')
            .long("format")
            .value_name("FORMAT")
            .default_value("kotusxml")
            .help("format")
            .takes_value(true))
        .arg(Arg::with_name("WORD")
            .help("Dirty word")
            .required(false)
            .index(1))
        .get_matches();
    


    if matches.is_present("WORD") {
        let word  = matches.value_of("WORD").unwrap();
    
        let spoonmaps = load_maps(matches.value_of("wordlist").unwrap(), matches.value_of("format").unwrap());
        for wordresult in spoonmaps.spoonerism(word).iter() {
            println!("{}", wordresult.rootword);
            for ending in wordresult.endings.iter() {
                println!("      {}", ending);
            }
        };
    }
}
