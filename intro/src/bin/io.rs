use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut f = OpenOptions::new()
        .truncate(true)
        .read(true)
        .create(true)
        .write(true)
        .open("./src/bin/tmpfile")
        .unwrap();
    f.write_all(b"Hello, world\n").unwrap();
    f.sync_all().unwrap();
}
