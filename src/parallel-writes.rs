use std::{
    fs::{remove_file, File},
    io::Read,
};

use rustyline::Editor;

const PATH: &str = "history/parallel.txt";

fn main() {
    let _ = remove_file(PATH);

    let t = std::thread::spawn(|| {
        run();
    });

    run();

    t.join().unwrap();

    println!("After parallel writes");
    print_contents();

    run();

    println!("After final writes");
    print_contents();
}

fn print_contents() {
    let mut f = File::open(PATH).unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();
    println!("{}", s);
}

fn run() {
    let mut e = Editor::<()>::new();
    dump_stuff(&mut e);
    write(&mut e, "ls");
    dump_stuff(&mut e);
}

fn dump_stuff(e: &mut Editor<()>) {
    for i in 0..2 {
        write(e, &format!(".......... {}", i));
    }
}

fn write(e: &mut Editor<()>, data: &str) {
    e.add_history_entry(data);
    e.append_history(PATH).unwrap();
}
