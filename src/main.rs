use std::{
    fs::File,
    io::{Read, Write},
};

use rustyline::Editor;

fn main() {
    test(1);
    test(2);
    test(3);
    test(4);
}

fn test(initial_count: usize) {
    write_history(initial_count);

    let mut f = File::open("history.txt").unwrap();

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    println!("---------\ncount={}\n{}", initial_count, s);
}

fn write_initial(count: usize) {
    let mut f = File::create("history.txt").unwrap();

    for _ in 0..count {
        f.write_all(b"A --- long line\n").unwrap();
    }
    f.flush().unwrap();
}

fn write_history(initial_count: usize) {
    write_initial(initial_count);

    let mut e = Editor::<()>::new();

    e.add_history_entry("B --- long line");
    e.append_history("history.txt").unwrap();
}
