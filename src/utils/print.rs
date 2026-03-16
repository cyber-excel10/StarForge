use colored::*;

pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "→".cyan(), msg);
}

pub fn warn(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}

pub fn header(msg: &str) {
    println!("\n{}", msg.bright_white().bold().underline());
}

pub fn kv(key: &str, value: &str) {
    println!("  {:<20} {}", key.dimmed(), value.bright_white());
}

pub fn kv_accent(key: &str, value: &str) {
    println!("  {:<20} {}", key.dimmed(), value.cyan().bold());
}

pub fn separator() {
    println!("{}", "─".repeat(60).dimmed());
}

pub fn step(n: usize, total: usize, msg: &str) {
    println!(
        "{} {}",
        format!("[{}/{}]", n, total).dimmed(),
        msg.bright_white()
    );
}
