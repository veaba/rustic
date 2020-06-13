use std::fs;

fn main() {
    let _ = write_file()();
}

fn write_file() -> std::io::Result<()> {
    fs::write("foo.txt", b"foo-->")?;
    fs::write("bar.txt", "bar ==?")?;
    Ok(())
}
/*
use std::fs;

fn main() {
    let _ = write_file()();
}

fn write_file() -> std::io::Result<()> {
    fs::write("foo.txt", b"foo-->")?;
    fs::write("bar.txt", "bar ==?")?;
    Ok(())
}
*/
