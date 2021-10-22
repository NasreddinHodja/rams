mod sender;
use sender::*;

fn main() {
    let s = Sender::new();
    // println!("{:#?}", s);
    s.send("one_piece", 1, 1);
}
