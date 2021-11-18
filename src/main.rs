use std::{
    env,
    fs::read_to_string,
    io::{self, Write},
    path::Path,
    process::exit,
    sync::atomic::AtomicBool,
};

use scanner::Scanner;

mod scanner;
mod token;

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), io::Error> {
    let mut args = env::args();
    let bin_name = args.next().expect("no bin_name in args");
    let file_name = args.next();

    // args is mutable iterator, so it should be empty, after using all possible args
    if args.len() > 0 {
        println!("Usage: {} filename", bin_name);
        exit(64);
    }

    match file_name {
        Some(file_name) => run_file(file_name)?,
        None => run_prompt()?,
    }

    Ok(())
}

fn run_file<P: AsRef<Path>>(file_name: P) -> Result<(), io::Error> {
    let code = read_to_string(file_name)?;

    run(&code);

    if HAD_ERROR.load(std::sync::atomic::Ordering::Relaxed) {
        exit(65);
    }

    Ok(())
}

fn run_prompt() -> Result<(), io::Error> {
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let line = {
            let mut line = String::new();
            match stdin.read_line(&mut line)? {
                0 => break,
                _ => line,
            }
        };

        run(&line);
        HAD_ERROR.store(false, std::sync::atomic::Ordering::Relaxed);
    }

    Ok(())
}

fn run(source: &str) {
    let scanner = Scanner::new(source);

    for token in scanner.tokens() {
        println!("{:?}", token);
    }
}

pub fn error<S: AsRef<str>>(line: usize, message: S) {
    report(line, None, message.as_ref());
}

fn report(line: usize, location: Option<&str>, message: &str) {
    eprintln!(
        "[line {}] Error{}: {}",
        line,
        if let Some(loc) = location {
            format!(" {}", loc)
        } else {
            "".to_owned()
        },
        message
    );

    HAD_ERROR.store(true, std::sync::atomic::Ordering::Relaxed);
}
