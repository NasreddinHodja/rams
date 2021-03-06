use std::io;
use std::io::prelude::*;
use std::io::{Read, BufReader};
use std::fs;
use std::net::TcpStream;
use std::path::Path;
use ssh2::{Session, Error, Sftp};
use glob::glob;

use serde::{Serialize, Deserialize};
use confy;

#[derive(Debug)]
pub struct Sender {
    host: String,
    port: String,
    username: String,
    password: String,
    source: String,
    destination: String,
}

impl Sender {
    pub fn new() -> Sender {
        let cfg: SenderConfig = confy::load("rams").unwrap();

        Sender {
            host: cfg.host,
            port: cfg.port,
            username: cfg.username,
            password: cfg.password,
            source: cfg.source,
            destination: cfg.destination,
        }
    }

    pub fn update_config(&mut self) {
        let cfg = SenderConfig::from_input();

        self.host = cfg.host;
        self.port = cfg.port;
        self.username = cfg.username;
        self.password = cfg.password;
        self.source = cfg.source;
        self.destination = cfg.destination;
    }

    pub fn send_chapter(
        &self,
        local_path: &str,
        remote_path: &str,
        chapter: u32,
        sftp: &Sftp,
    ) -> Result<(), Error> {
        let path = format!("{}chapter_{:04}*",
                            local_path, chapter);
        let chapter_paths = glob(&path).unwrap();

        for chapter_path in chapter_paths {
            let remote_chapter_path =
                chapter_path.as_ref().unwrap().to_string_lossy();
            let split_path: Vec<&str> =
                remote_chapter_path.split("/").collect();
            let remote_chapter_path =
                format!("{}{}/", remote_path,
                                 &split_path[split_path.len()-1]);
            sftp.mkdir(&Path::new(&remote_chapter_path), 0o644)
                .expect(&format!("failed to create directory {}",
                                 remote_chapter_path));

            println!("{}", remote_chapter_path);
            for path in fs::read_dir(chapter_path.unwrap()).unwrap() {
                let local_path =
                    String::from(path.unwrap().path().to_string_lossy());
                let split_path: Vec<&str> = local_path.split("/").collect();
                let chapter_path = &split_path[split_path.len()-3..].join("/");
                let remote_path = format!("{}{}", self.destination,
                                          chapter_path);

                // read local file
                let local_file = fs::File::open(local_path).unwrap();
                let mut reader = BufReader::new(local_file);
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer)
                      .expect(&format!("couldn't read local {} file to buffer",
                                      chapter_path));

                // write remote file
                let mut remote_file =
                    sftp.create(&Path::new(&remote_path)).unwrap();
                remote_file.write(&buffer).unwrap();
            }
        }

        Ok(())
    }

    pub fn send(
        self,
        manga_name: &str,
        start_chapter: u32,
        end_chapter: u32,
    ) -> Result<(), Error> {
        // connect to address
        let address = format!("{}:{}", self.host, self.port);
        let tcp = TcpStream::connect(address).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_password(&self.username, &self.password).unwrap();

        // open sftp
        let sftp = sess.sftp().unwrap();

        let local_path = format!("{}{}/",
                                 self.source, manga_name);
        let remote_path = format!("{}{}/",
                                  self.destination, manga_name);
        // make remote dir for manga if non existant
        match sftp.mkdir(&Path::new(&remote_path), 0o644) {
            Ok(_) => {},
            Err(_) => {},
        };

        if start_chapter == 0 && end_chapter == 0 {
            // send whole manga
            let chapters_glob = String::from(&local_path) + "chapter_*/".into();
            for chap in glob(&chapters_glob).unwrap() {
                let chap_str = chap.unwrap()
                    .into_os_string().into_string().unwrap();
                let chap_str = chap_str .split("-").collect::<Vec<&str>>()[0];
                let chap_num: u32 = chap_str .split("_").last().unwrap()
                    .parse().unwrap();
                self.send_chapter(&local_path, &remote_path, chap_num, &sftp)
                    .unwrap()
            }
            return Ok(());
        } else {
            // send chapters
            for i in start_chapter..(end_chapter+1) {
                self.send_chapter(&local_path, &remote_path, i,&sftp).unwrap();
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderConfig {
    host: String,
    port: String,
    username: String,
    password: String,
    source: String,
    destination: String,
}

impl ::std::default::Default for SenderConfig {
    fn default() -> Self {
        Self::from_input()
    }
}

impl SenderConfig {
    pub fn from_input() -> Self {
        let mut host = String::new();
        let mut port = String::new();
        let mut username = String::new();
        let mut password = String::new();
        let mut source = String::new();
        let mut destination = String::new();

        println!("host:");
        io::stdin()
            .read_line(&mut host)
            .expect("Error reading input");
        println!("port:");
        io::stdin()
            .read_line(&mut port)
            .expect("Error reading input");
        println!("username");
        io::stdin()
            .read_line(&mut username)
            .expect("Error reading input");
        println!("password");
        io::stdin()
            .read_line(&mut password)
            .expect("Error reading input");
        println!("source");
        io::stdin()
            .read_line(&mut source)
            .expect("Error reading input");
        println!("destination");
        io::stdin()
            .read_line(&mut destination)
            .expect("Error reading input");

        let new_cfg = SenderConfig {
            host: host.trim().into(),
            port: port.trim().into(),
            username: username.trim().into(),
            password: password.trim().into(),
            source: source.trim().into(),
            destination: destination.trim().into(),
        };

        confy::store("rams", &new_cfg,).unwrap();
        new_cfg
    }
}
