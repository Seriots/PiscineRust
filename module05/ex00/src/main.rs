use std::writeln;
use std::io::stdout;
use std::io::Write;


fn main() {
    for i in 1..=10 {

        match writeln!(stdout(), "{i}") {
            Err(_) => return,
            Ok(_) => continue,
        }
    }
}
