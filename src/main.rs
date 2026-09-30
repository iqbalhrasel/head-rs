use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.is_empty() || args.len() < 3 {
        eprintln!("Usage: head -n 5 file.txt");
        exit(1);
    }

    let line_count = match args[1].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Usage: head -n 5 file.txt");
            eprintln!(r#"Usage: Put number after "-n". Example: -n 5"#);
            exit(1);
        }
    };

    let files = &args[2..];

    for f in files {
        match File::open(f) {
            Ok(file) => {
                let reader = BufReader::new(file);

                for line_result in reader.lines().take(line_count) {
                    match line_result {
                        Ok(line) => println!("{line}"),
                        Err(_) => eprintln!("Unable to read line"),
                    }
                }
                println!()
            }
            Err(_) => eprintln!("Unable to open file"),
        }
    }
}
