
mod sender;
use sender::*;

fn main() {
    let s = Sender::new();
    // println!("{:#?}", s);
    s.send("one_piece", 1, 1);
    // Connect to the local SSH server
    // let tcp = TcpStream::connect("192.168.1.100:1234").unwrap();
    // let mut sess = Session::new().unwrap();
    // sess.handshake(&tcp).unwrap();
    // sess.userauth_password("nasreddin", "oceanboie").unwrap();

    // let (mut remote_file, stat) = sess.scp_recv(Path::new("Pictures/manga/chapter_0146-1/000.jpeg")).unwrap();
    // println!("remote file size: {}", stat.size());
    // let mut contents = Vec::new();
    // remote_file.read_to_end(&mut contents).unwrap();
    // ...
}
