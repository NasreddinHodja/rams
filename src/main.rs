mod sender;
use sender::Sender;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Rust manga sender")
        .version("1.0")
        .author("NasreddinHodja")
        .about("sends manga chapters throught sftp")
        .arg(
            Arg::with_name("MANGA")
                .help("manga folder")
                // .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("start")
                .short("s")
                .value_name("START")
                .takes_value(true)
                .help("start manga chapter"),
        )
        .arg(
            Arg::with_name("end")
                .short("e")
                .value_name("END")
                .takes_value(true)
                .help("end manga chapter"),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(false)
                // .conflicts_with_all(&["MANGA", "start", "end"])
                .help("configure sftp"),
        )
        .get_matches();

    let mut s = Sender::new();

    if matches.is_present("config") {
        s.update_config();
        return;
    }

    if let Some(manga) = matches.value_of("MANGA") {
        if !s.has_manga(manga) {
            println!("Error: {} manga was not found in source", manga);
            return;
        }

        match matches.value_of("start") {
            Some(start) => match matches.value_of("end") {
                Some(end) => {
                    println!(
                        "Sending {} -- chapters {} through {}",
                        manga, start, end
                    );
                    s.send(
                        manga,
                        start.parse::<u32>().unwrap(),
                        end.parse::<u32>().unwrap(),
                    )
                    .unwrap();
                }
                _ => {
                    println!("Sending {} -- chapter {}", manga, start);
                    s.send(
                        manga,
                        start.parse::<u32>().unwrap(),
                        start.parse::<u32>().unwrap(),
                    )
                    .unwrap();
                }
            },
            _ => {
                println!("Sending {} all chapters", manga);
                s.send(manga, 0, 0).unwrap();
            }
        }
    }
}
