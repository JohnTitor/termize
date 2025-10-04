fn main() {
    if let Some((w, h)) = termize::dimensions() {
        println!("nice! your terminal is of {w}x{h} dimensions");
    } else {
        eprintln!("couldn't read terminal size :(");
    }
}
