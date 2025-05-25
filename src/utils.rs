use colored::*;

/// Prints detailed checks similar to the original (length, uppercase, etc.)
pub fn print_detailed_checks(password: &str) {
    let checks = [
        ("Length >= 12", password.chars().count() >= 12),
        ("Has uppercase", password.chars().any(|c| c.is_uppercase())),
        ("Has lowercase", password.chars().any(|c| c.is_lowercase())),
        ("Has number", password.chars().any(|c| c.is_ascii_digit())),
        ("Has symbol", password.chars().any(|c| !c.is_alphanumeric())),
    ];

    println!();
    for (label, passed) in &checks {
        let status = if *passed { "✔" } else { "✘" };
        let color = if *passed { "green" } else { "red" };
        println!("{}", format!("{label}: {status}").color(color));
    }
}
