use std::{
    fs::File,
    io::{Read, Write},
};

use rustyline::Editor;

const PATH: &str = "history/init.txt";

fn main() {
    let mut f = File::create(PATH).unwrap();

    f.write_all(b"#V2\nA --- long line\nA --- long line\n")
        .unwrap();

    f.flush().unwrap();
    drop(f);

    let mut e = Editor::<()>::new();

    e.add_history_entry(format!("B"));
    e.append_history(PATH).unwrap();

    drop(e);

    let mut f = File::open(PATH).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    println!("{}", s);
}
