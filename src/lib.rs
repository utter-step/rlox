use std::sync::atomic::{self, AtomicBool};

pub use scanner::Scanner;
pub use token::Token;

mod scanner;
mod token;

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub fn error<S: AsRef<str>>(line: usize, message: S) {
    report(line, None, message.as_ref());
}

pub fn reset_error() {
    HAD_ERROR.store(false, atomic::Ordering::Relaxed);
}

pub fn had_error() -> bool {
    HAD_ERROR.load(atomic::Ordering::Relaxed)
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

    HAD_ERROR.store(true, atomic::Ordering::Relaxed);
}
