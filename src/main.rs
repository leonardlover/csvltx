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

    let mut lines = file_contents.lines();

    let header = lines.next();

    if let Some(header) = header {
        let headers: Vec<_> = header.split(',').collect();
        let c = "|c".repeat(headers.len());

        println!("\\begin{{tabular}}{{{c}|}}\n    \\hline");

        let header = headers.join(" & ");

        println!("    {header} \\\\\n    \\hline");

        for line in lines {
            let line: Vec<_> = line.split(',').collect();
            let row = line.join(" & ");

            println!("    {row} \\\\\n    \\hline");
        }

        println!("\\end{{tabular}}");
    }
}
