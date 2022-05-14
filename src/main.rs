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

    let s = Sender::new();

    let manga = matches.value_of("MANGA").unwrap();
    if let Some(start) = matches.value_of("start") {
        if let Some(end) = matches.value_of("end") {
            println!("Sending {} -- chapters {} through {}",
                    manga, start, end);
            s.send(manga,
                   start.parse::<u32>().unwrap(),
                   end.parse::<u32>().unwrap())
             .unwrap();
        } else {
            println!("Sending {} -- chapter {}", manga, start);
            s.send(manga,
                   start.parse::<u32>().unwrap(),
                   start.parse::<u32>().unwrap())
             .unwrap();
        };
    } else {
        println!("Sending {} all chapters", manga);
        s.send(manga, 0, 0).unwrap();
    };
}
