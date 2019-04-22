
use std::io::{stdout, BufWriter};
use ferris_says::say;

fn main() {
    let stdout = stdout();
    let out = b"I+D Team Baby!";
    let width = 44;
    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).ok();
}
