use std::fs::remove_file;
use std::{fs::File, io::Read};

use rustyline::{config::Configurer, Editor};

fn main() {
    test(1);
    test(100);
}

fn test(initial_count: usize) {
    write_history(initial_count);

    let mut f = File::open("history.txt").unwrap();

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    println!("---------\ncount={}\n{}", initial_count, s);
    remove_file("history.txt").unwrap();
}

fn write_history(count: usize) {
    let mut f = Editor::<()>::new();
    f.set_history_ignore_dups(false);
    for _ in 0..count {
        f.add_history_entry("A --- long line");
    }
    f.append_history("history.txt").unwrap();

    let mut e = Editor::<()>::new();

    e.add_history_entry("B --- long line");
    e.append_history("history.txt").unwrap();

    f.add_history_entry("C --- long line");
    f.append_history("history.txt").unwrap();

    f.add_history_entry("C --- long line");
    f.append_history("history.txt").unwrap();

    e.add_history_entry("D --- long line");
    e.append_history("history.txt").unwrap();
}
