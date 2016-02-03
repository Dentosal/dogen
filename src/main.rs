#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env;

fn print_usage() {
    println!("Dogen - Documentation Generator for Lazy students");
    println!("Copyright (c) 2016 Hannes Karppila Holder All Rights Reserved.");
    println!("\n\tUsage: dogen infile\n");
}


fn read_file(filename: &str, buffer: &mut String) -> Result<(), io::Error<>> {
    let file = try!(File::open(filename));
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = try!(line);
        buffer.push_str(&format!("{}\n", line));
    }
    Ok(())
}
fn write_file(filename: &str, buffer: &String) -> Result<(), io::Error<>> {
    let mut file = try!(File::create(filename));
    try!(file.write_all(&buffer.as_bytes()));
    Ok(())
}


fn py_parse_args(range_str: &String) -> Vec<String> {
    let mut ret = Vec::new();
    let mut buf = String::new();
    let mut level = 0;
    for c in range_str.chars() {
        if c == '(' || c == '[' || c == '{' {
            level += 1;
            continue;
        }
        if c == ')' || c == ']' || c == '}' {
            level -= 1;
            continue;
        }
        if c == ',' {
            ret.push(buf.clone());
            buf.clear();
            continue;
        }
        if c == ' ' {
            continue
        }
        if level == 0 {
            buf.push(c);
        }
    }
    ret.push(buf.clone());
    return ret;
}


fn py_range_to_string(range_str: &mut String) {
    let range_args = py_parse_args(&range_str);
    if !range_args.is_empty() {
        let mut start = "0".to_string();
        let mut end = "".to_string();
        let mut step = "1".to_string();
        let mut index = 0;
        for cap in range_args {
            index += 1;
            if index == 1 {
                end = cap.clone();
            }
            else if index == 2 {
                start = end.clone();
                end = cap.clone();
            }
            else if index == 3 {
                step = cap.clone();
            }
            else {  // invalid format
                return; // just return, no modifications made
            }
        }
        range_str.clear();
        if index == 0 {  // invalid format
            return; // just return, no modifications made
        }
        range_str.push_str(&format!("from {} to {}", start, end));
        if index == 3 {
            range_str.push_str(&format!(" with step {}", step));
        }
    }
}

fn get_group(regex: &regex::Regex, subject: &String) -> String {    // get first group (fast)
    let mut ret = String::new();
    if regex.is_match(subject) {
        ret.push_str(match regex.captures(subject) {
            Some(g) => match g.at(1) {Some(x) => x, None => ""},
            None    => "",
        });
    }
    return ret;
}
fn get_groups(regex: &regex::Regex, subject: &String) -> Vec<String> {  // get all groups
    let mut ret = Vec::new();
    if regex.is_match(subject) {

        let cap = regex.captures(subject).unwrap();
        for (i,c) in cap.iter().enumerate() {
            if i > 0 {
                let mut ns = String::new();
                ns.push_str(c.unwrap());
                ret.push(ns.clone());
            }
        }
    }
    return ret;
}

fn generate_comment(mut result: &mut String) {
    // print!("{} // ", result);

    let inp = result.clone();
    result.clear();

    if inp.contains("#") {   // possible comment. TODO: I'm too lazy to parse this, so...
        return;
    }

    let re_def = regex!(r"^def\s*?([a-zA-Z_][a-zA-Z0-9_]*)\s*.+?$");
    let re_assign = regex!(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.+?)$");
    let re_while_true = regex!(r"^while\s+True\s*:$");
    let re_while = regex!(r"^while\s+(.+)\s*:$");
    let re_for_range = regex!(r"^for.+in\s+range\((.+)\):$");
    let re_for = regex!(r"^for.+in\s+(.+):$");
    let re_print_str = regex!(r#"^print\s*\(?"(.+)"\)?\s*$"#);
    let re_print_var = regex!(r"^print\s*\(?([a-zA-Z_][a-zA-Z0-9_]*)\)?$");
    let re_print_expr1 = regex!(r"^print\s*(.+)\s*$");
    let re_print_expr2 = regex!(r"^print\s*\((.+)\)\s*$");
    let re_increase_one = regex!(r"^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\+=\s*1\s*$");
    let re_decrease_one = regex!(r"^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*-=\s*1\s*$");
    let re_self_add = regex!(r"^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\+=\s*(.+)\s*$");
    let re_self_sub = regex!(r"^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*-=\s*(.+)\s*$");

    let rr_def = get_group(&re_def, &inp);

    if !rr_def.is_empty() {
        result.push_str(&format!("# Define function {}", rr_def));
        return;
    }

    let rr_assign = get_groups(&re_assign, &inp);
    if !rr_assign.is_empty() {
        result.push_str(&format!("# Set {} to {}", rr_assign[0], rr_assign[1]));
        return;
    }

    if re_while_true.is_match(&inp) {
        result.push_str(&"# Loop forever");
        return;
    }

    let rr_while = get_group(&re_while, &inp);
    if !rr_while.is_empty() {
        result.push_str(&format!("# Repeat while {} is true", rr_while));
        return;
    }

    let mut rr_for_range = get_group(&re_for_range, &inp);
    if !rr_for_range.is_empty() {
        py_range_to_string(&mut rr_for_range);
        result.push_str(&format!("# Loop {}", rr_for_range));
        return;
    }
    let rr_for = get_group(&re_for, &inp);
    if !rr_for.is_empty() {
        result.push_str(&format!("# Loop through {}", rr_for));
        return;
    }

    let rr_print_str = get_group(&re_print_str, &inp);
    if !rr_print_str.is_empty() {
        // printing string does not need a comment
        return;
    }

    let rr_print_var = get_group(&re_print_var, &inp);
    if !rr_print_var.is_empty() {
        result.push_str(&format!("# Print value of {}", rr_print_var));
        return;
    }

    let rr_print_expr1 = get_group(&re_print_expr1, &inp);
    if !rr_print_expr1.is_empty() {
        result.push_str(&format!("# Print result of {}", rr_print_expr1));
        return;
    }
    let rr_print_expr2 = get_group(&re_print_expr2, &inp);
    if !rr_print_expr2.is_empty() {
        result.push_str(&format!("# Print result of {}", rr_print_expr2));
        return;
    }

    let rr_increase_one = get_group(&re_increase_one, &inp);
    if !rr_increase_one.is_empty() {
        result.push_str(&format!("# Increase {} by one", rr_increase_one));
        return;
    }
    let rr_decrease_one = get_group(&re_decrease_one, &inp);
    if !rr_decrease_one.is_empty() {
        result.push_str(&format!("# Decrease {} by one", rr_decrease_one));
        return;
    }

    let rr_self_add = get_groups(&re_self_add, &inp);
    if !rr_self_add.is_empty() {
        result.push_str(&format!("# Add {} to {}", rr_self_add[1], rr_self_add[0]));
        return;
    }

    let rr_self_sub = get_groups(&re_self_sub, &inp);
    if !rr_self_sub.is_empty() {
        result.push_str(&format!("# Subtract {} from {}", rr_self_sub[1], rr_self_sub[0]));
        return;
    }
}
fn process(input: &String, output: &mut String) {
    for line in input.lines() {
        let mut result_comment = line.clone().trim().to_string();
        if line.is_empty() {
            output.push_str("\n");
        }
        else {
            generate_comment(&mut result_comment);
            output.push_str(&format!("{} {}\n", line, result_comment));
        }
    }
}


fn main() {

    let args: Vec<_> = env::args().collect();
    let mut flags: Vec<char> = Vec::new();
    let mut infile = String::new();
    let mut outfile = String::new();

    let mut input = String::new();
    let mut output = String::new();

    for arg in &args[1..] {
        if arg.starts_with("-") {
            for c in arg[1..].chars() {
                flags.push(c);
            }
        }
        else {
            if infile.is_empty() {
                infile = arg.clone();
            }
            else if outfile.is_empty() {
                outfile = arg.clone();
            }
            else {
                print_usage();
            }
        }
    }
    flags.sort();
    flags.dedup();

    if !infile.is_empty() {

        // if flags.contains(&'q') {
        //     println!("q flag!!");
        // }
        // if args.contains(&"-q".to_string()) {
        //     println!("q flag?!");
        // }
        //
        // println!("infile  '{}'", infile);
        // println!("outfile '{}'", outfile);

        match read_file(&infile, &mut input) {
            Ok(_)  => (),
            Err(e) => println!("{}", e.to_string()),
        }
        // println!("{}", input);

        process(&input, &mut output);
        if outfile.is_empty() { // stdout
            println!("{}", output);
        }
        else {
            match write_file(&outfile, &output) {
                Ok(_)  => (),
                Err(e) => println!("Error: Could not write to '{}': {}", outfile, e.to_string()),
            }
        }
    }
    else {
        print_usage();
    }

}
