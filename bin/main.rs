extern crate marte_lang;

use std::{env, fs, process};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub marte);

fn main() {
    let mut args = env::args().skip(1);
    if let Some(path) = args.next() {
        let text = fs::read_to_string(&path)
            .map_err(|err| {
                eprintln!("ERROR {path}: {err}");
                process::exit(1);
            })
            .unwrap();
        let chunk = marte::ChunkParser::new()
            .parse(&text)
            .map_err(|err| {
                eprintln!("ERROR {path}: {err}");
                process::exit(1);
            })
            .unwrap();
        dbg!(chunk);
    } else {
        println!("{USAGE}");
    }
}
const USAGE: &str = r#"USAGE:
    marte <file> -- runs file
"#;
