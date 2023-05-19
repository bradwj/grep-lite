use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex, ctx_lines: usize) {
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    let lines = reader
        .lines()
        .take_while(|line| line.as_ref().ok().is_some())
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    for (i, line) in lines.iter().enumerate() {
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

    for (i, line) in lines.iter().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
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
                .required(false),
        )
        .args_from_usage(
            "-C --context=[NUM] 'Prints searched line and n lines before and after the result.'",
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let ctx_lines: usize = args
        .value_of("context")
        .and_then(|x| x.parse().ok())
        .unwrap_or(0);

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re, ctx_lines);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re, ctx_lines);
    }
}
