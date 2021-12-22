extern crate clap;

use clap::{Arg, App};

mod lib;
use lib::{read_kotus_xml, spoonerism};

fn main() {
    let matches = App::new("sananmuunnos")
        .version("0.1")
        .author("antti.hayrynen@gmail.com")
        .arg(Arg::with_name("listen")
            .short("l")
            .long("listen")
            .value_name("PORT")
            .help("Listen to port")
            .takes_value(true))
        .arg(Arg::with_name("WORD")
            .help("Dirty word")
            .required(false)
            .index(1))
        .get_matches();
    
    if matches.is_present("WORD") {
        let word  = matches.value_of("WORD").unwrap();
    
        let (prefixmap, suffixmap) = read_kotus_xml("sanat.xml");
        spoonerism(word, &prefixmap, &suffixmap);
    }
}
