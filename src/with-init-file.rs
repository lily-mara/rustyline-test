use std::{
    fs::File,
    io::{Read, Write},
};

use rustyline::Editor;

fn main() {
    test(1);
    // test(2);
    // test(3);
    // test(4);
}

fn test(count: usize) {
    let path = format!("history/init-{}.txt", count);

    let mut f = File::create(&path).unwrap();

    // for _ in 0..count {
    f.write_all(b"A --- long line\nA --- long line\n").unwrap();
    // }

    f.flush().unwrap();
    drop(f);

    let mut e = Editor::<()>::new();

    e.add_history_entry(format!("B"));
    e.append_history(&path).unwrap();

    drop(e);

    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    println!("---------\ncount={}\n{}", count, s);
}
