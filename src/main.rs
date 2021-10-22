mod sender;
use sender::Sender;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Rust manga sender")
        .version("1.0")
        .author("NasreddinHodja")
        .about("sends manga chapters throught sftp")
        .arg(Arg::with_name("MANGA")
             .help("manga folder")
             .required(true)
             .index(1))
        .arg(Arg::with_name("start")
             .short("s")
             .value_name("START")
             .takes_value(true)
             .help("start manga chapter"))
        .arg(Arg::with_name("end")
             .short("e")
             .value_name("END")
             .takes_value(true)
             .help("start manga chapter"))
        .get_matches();

    let manga = matches.value_of("MANGA").unwrap();
    let start = matches.value_of("start").unwrap();
    let end = matches.value_of("end").unwrap();

    println!("Sending {} chapters {} through {}",
             manga, start, end);

    // let s = Sender::new();
    // s.send("chainsaw_man", 1, 97);
}
