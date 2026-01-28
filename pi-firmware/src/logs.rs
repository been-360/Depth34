pub fn red(text: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", text)
}

pub fn green(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}

pub fn yellow(text: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", text)
}

pub fn cyan(text: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", text)
}
