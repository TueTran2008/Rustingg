use std::{io::Read, str::Bytes};
#[derive(Debug)]
struct Foo {
    a: bool,
    b: u32,
}
use tracing::{error, info, span, trace, warn, Level};
fn main() {
    tracing_subscriber::fmt::init();
    let span = span!(Level::INFO, "main");
    let _guard = span.enter();
    for file in std::env::args() {
        let span = span!(tracing::Level::INFO, "file", hehe = %file);
        let _guard = span.enter();

        info!("opening a file");
        // let mut fd = std::fs::File::open(file).unwrap();
        // info!("this happen");
        // info!("that happen");
        // let mut bytes = Vec::new();
        warn!("reading the file");
        // fd.read(&mut bytes).unwrap();
        let foo = Foo { a: true, b: 10 };
        info!(parsing = ?foo, "Parsing file");
        //
        //
        info!("done with file");
    }
}
