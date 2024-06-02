use std::env;
use std::fs;
use std::process;

fn config(
    mut args: impl Iterator<Item = String>,
) -> Result<String, &'static str> {
    args.next();

    match args.next() {
        Some(arg) => {
            match fs::read_to_string(arg) {
                Ok(contents) => Ok(contents),
                _ => Err("Could not open file"),
            }
        },
        None => Err("Didn't get a file path"),
    }
}

fn main() {
    let file_contents = config(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    dbg!(file_contents);
}
