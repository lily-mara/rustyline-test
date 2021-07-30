use std::{fs::File, io::Read};

use rustyline::Editor;

fn main() {
    test(1);
    test(2);
    test(3);
    test(4);
}

fn test(editor_count: usize) {
    let mut editors = Vec::new();

    let path = format!("history/{}.txt", editor_count);

    for _ in 0..editor_count {
        editors.push(Editor::<()>::new());
    }

    for e in &mut editors {
        e.add_history_entry(format!("A --- long line"));
    }

    for e in &mut editors {
        e.add_history_entry(format!("B --- long line"));
        e.append_history(&path).unwrap();
    }

    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    println!("---------\ncount={}\n{}", editor_count, s);
}
