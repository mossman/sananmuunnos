extern crate clap;

use clap::{Arg, App};

mod lib;
//mod webapp;

use lib::{SpoonMaps};
//use webapp::{spawn};

#[async_std::main]
async fn main() {
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
    

//    if let Some(port) = matches.value_of("listen") {
//       let port: u16 = port.parse().unwrap_or_else(|error| {
//            panic!("Unable to parse port: {:?}", error);
//        });
//        let spoonmaps = SpoonMaps::from_kotus_xml("sanat.xml");

//        spawn(port, &prefixmap, &suffixmap).await;
//    }

    if matches.is_present("WORD") {
        let word  = matches.value_of("WORD").unwrap();
    
        let spoonmaps = SpoonMaps::from_kotus_xml("sanat.xml");
        for wordresult in spoonmaps.spoonerism(word).iter() {
            println!("{}", wordresult.rootword);
            for ending in wordresult.endings.iter() {
                println!("      {}", ending);
            }
        };
    }
}
