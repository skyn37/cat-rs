use std::env;
use std::env::Args;

use std::fs::File;

use std::io;
use std::io::Stdin;
use std::io::IsTerminal;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;

use std::iter::Skip;

#[derive(Debug)]
struct Options {

    number_to_blank: bool,
    number_to_nonblank: bool,
    lines_empty_trunk: bool,
    files_to_open: Vec<String>
}


fn main() {

    let  args: Skip<Args> = env::args().skip(1);
    let options = from_args_options(args);
    handle_files(options);
}


fn handle_files(options: Options) {
    println!("*********************");
    println!("{options:?}");
    println!("*********************\n\n");

    let mut number_for_lines = 0;


    for path in options.files_to_open {
        let f = File::open(path.clone()).expect("Unable to open file");
        let f = BufReader::new(f);
        let mut prev_line = String::from("!!!"); // just a placeholder string
        for line in f.lines().peekable() {
            let line = line.expect("Unable to read line");

            if options.lines_empty_trunk {
                if prev_line == "" && line == "" {
                    continue 
                } else {
                    prev_line = String::from("!!!"); // just a placeholder string 
                }

                if line == "" {
                    prev_line = line.clone();
                }
            }

            if options.number_to_blank {
                number_for_lines += 1;
                if line.is_empty() && options.number_to_nonblank {
                    println!("{}", line)
                } else {
                    println!("     {} {}", number_for_lines, line);

                }
            } else {
                println!("{:X}", line)
            }
            //  println!("Line: {}", line);
        }       
        println!("{path:?}");
    }

}



fn from_args_options(mut args: Skip<Args>) -> Options {
    let mut options = Options {
        number_to_nonblank : false,
        number_to_blank: false,
        lines_empty_trunk: false,
        files_to_open: vec![],
    };


    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-b" => { 
                options.number_to_nonblank = true;
            },
            "-n" => {
                options.number_to_blank = true;
            },
            "-s" => {
                options.lines_empty_trunk = true;
            },
            _ => {
                options.files_to_open.push(arg);
            }
        }
    }

    options
}



