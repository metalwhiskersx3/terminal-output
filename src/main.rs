use console::Term;
use std::io::stdout;
use std::thread;
use std::time::Duration;

fn main() {
    let term = Term::stdout();
    term.write_line("Hello World");
    thread::sleep(Duration::from_millis(2000));
    term.clear_line();
}
