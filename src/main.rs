use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

fn look(s: &str, q: &str) -> bool {
    s.starts_with(q)
}

fn process<R: BufRead>(reader: R) { //}, re: &Regex) {
    let mut in_comment: bool = false;
    let mut in_quote: bool = false;
    let mut quote_start = "";
    let mut quote_end = "";
    let mut comment_start = "";
    let mut comment_end = "\n";
    let comment_delimiters = vec![("//", "\n"), ("/*", "*/")];
    let quote_delimiters = vec![("\"", "\""), ("'", "'"), 
            ("r#\"", "\"#"), ("r##\"", "\"##")];

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.chars().all(|c| c.is_whitespace()) {
                println!("");
                continue;
            }
            let line = line + "\n";
            let chars: Vec<(usize, char)> = line.char_indices().collect();
            // println!("{:?}", chars);
            if !in_comment & (chars.len() == 0) {
                println!("");
                continue;
            }
            let mut to_print = String::new();
            // let mut modified: String = "".to_string();
            let mut index = 0;

            'chars: loop {
                if index >= chars.len() {
                    break;
                }
                // println!("# {}", index);
                let rest = &line[chars[index].0..];
                if in_comment {
                    if look(rest, comment_end) {
                        in_comment = false;
                        index += comment_end.chars().count();
                        continue;
                    }
                    index += 1;
                    continue;

                } else if in_quote {
                    if look(rest, quote_end) {
                        in_quote = false;
                        index += quote_end.chars().count();
                        to_print += &quote_end;
                    }
                } else {
                    for cstart in comment_delimiters.iter() {
                        if look(&rest, &cstart.0.to_string()) {
                            in_comment = true;
                            comment_start = cstart.0;
                            comment_end = cstart.1;
                            index += cstart.0.chars().count();
                            continue 'chars;
                        }
                    }
                    for sstart in quote_delimiters.iter() {
                        if look(&rest, &sstart.0.to_string()) {
                            in_quote = true;
                            quote_start = sstart.0;
                            quote_end = sstart.1;
                            index += sstart.0.chars().count();
                            to_print = to_print + sstart.0;
                            continue 'chars;
                        }
                    }
                }
                if index < chars.len() -1 {
                    to_print = to_print + &line[chars[index].0..chars[index+1].0];
                } else {
                    to_print = to_print + &line[chars[index].0..];
                }
                index += 1;
            }
            if to_print.ends_with('\n') {
                to_print.pop();
            }
            if to_print.len() == 0 {
                continue;
            }
            if to_print.chars().all(|c| c.is_whitespace()) {
                continue;
            }
            println!("{}", to_print);
        }
    }
}

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    // let re = Regex::new(r###"(?<![\"'])(?<!\\[\"'])#(?![^\"']*['\"][^'\"]*$)"###).unwrap();
    // let re = regex::Regex::new(r##""(\\.|[^"\\])*"|r"(?:#*".*?"#*)""##).unwrap();
    // let re = regex::Regex::new(r##""(\\.|[^"\\])*"|r"(?:#*".*?"#*)""##).unwrap();
    // let ll = "hello".to_string().len();
    // println!("{ll} <- ");

    if args.len() > 1 {
        for path in &args[1..] {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            process(reader); // , &re);
        }
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process(reader);//, &re);
    }
    let _ = r#"not" //funny"#;
    Ok(())
}
/* 
    latex \% is a tricky one
    needs to be a regex
*/
