use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("the pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("the file to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let ctx_lines = 2; // TODO: take in an arg for this

    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let mut reader = BufReader::new(f);

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line_) in reader.by_ref().lines().enumerate() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => {
                tags.push(i);

                let v = Vec::with_capacity(2 * ctx_lines + 1);
                ctx.push(v);
            }
            None => (),
        }
    }

    if tags.is_empty() {
        return;
    }

    reader.seek(std::io::SeekFrom::Start(0)).unwrap();

    let mut line: String;
    for (i, line_) in reader.lines().enumerate() {
        line = line_.unwrap();
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(&line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            println!("{}: {}", i + 1, line)
        }
    }
}
