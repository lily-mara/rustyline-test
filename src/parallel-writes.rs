use std::fs::remove_file;

use rustyline::Editor;

const PATH: &str = "history/parallel.txt";

fn main() {
    remove_file(PATH).unwrap();

    let t = std::thread::spawn(|| {
        run();
    });

    run();

    t.join().unwrap();

    // at this point, there are duplicate lines in the history file. You can
    // commment out this final `run` call to verify. After we call `run` the
    // final time, the front of lines will be removed.
    run();
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
