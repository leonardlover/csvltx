use std::env;
use std::process;

fn config(
    mut args: impl Iterator<Item = String>,
) -> Result<String, &'static str> {
    args.next();

    match args.next() {
        Some(arg) => Ok(arg),
        None => Err("Didn't get a file path"),
    }
}

fn main() {
    let file_path = config(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
}
