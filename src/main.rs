use colored::*;
use std::io::{self, Write};

fn main() {
    println!("{}", "🔐 Advanced Password Strength Checker".bold());
    println!("Type 'exit' to quit.\n");

    loop {
        print!("Enter password: ");
        io::stdout().flush().unwrap();

        let mut password = String::new();
        if io::stdin().read_line(&mut password).is_err() {
            eprintln!("{}", "Failed to read input.".red());
            continue;
        }
        let password = password.trim();

        if password.eq_ignore_ascii_case("exit") {
            println!("{}", "Goodbye! Stay secure!".green());
            break;
        }
        if password.is_empty() {
            println!("{}", "Password cannot be empty. Please try again.".yellow());
            continue;
        }

        // Print simple checks
        print_basic_checks(password);

        // Use zxcvbn
        match zxcvbn::zxcvbn(password, &[]) {
            Ok(estimate) => {
                println!("\nPassword Checks:");
                let score_color = match estimate.score() {
                    0 | 1 => "red",
                    2 => "yellow",
                    3 => "cyan",
                    _ => "green",
                };
                println!(
                    "{}",
                    format!("Score: {}/4", estimate.score()).color(score_color)
                );
                // Convert log10 guesses to bits: log2(x) = log10(x) / log10(2)
                let entropy_bits = estimate.guesses_log10() / std::f64::consts::LOG10_2;
                println!(
                    "{}",
                    format!("Entropy: {:.2} bits", entropy_bits).cyan()
                );
                println!(
                    "{}",
                    format!(
                        "Estimated crack time (offline, 10B/s): {}",
                        estimate.crack_times().offline_fast_hashing_1e10_per_second()
                    )
                    .magenta()
                );
                println!(
                    "{}",
                    format!(
                        "Estimated crack time (online, 10/s): {}",
                        estimate.crack_times().online_no_throttling_10_per_second()
                    )
                    .magenta()
                );

                if let Some(feedback) = estimate.feedback() {
                    if let Some(warning) = feedback.warning() {
                        println!("{}", format!("Warning: {}", warning).yellow());
                    }
                    for suggestion in feedback.suggestions() {
                        println!("- {}", suggestion);
                    }
                }
                println!("\n{}", "=".repeat(50));
            }
            Err(e) => println!("{}", format!("Error: {e}").red()),
        }
    }
}

fn print_basic_checks(password: &str) {
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
