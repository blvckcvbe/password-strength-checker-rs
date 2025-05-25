# Advanced Password Strength Checker (Rust CLI)

A simple, interactive, and modern CLI tool to check password strength using the [`zxcvbn`](https://github.com/dropbox/zxcvbn) algorithm in Rust. While making this tool I realized that there were too many performance issues in using the rockyou.txt.gz so that's why I switched to zxcvbn. This is a bit larger than my python tool but definitely has better performance.

## Features

- **Advanced strength estimation:** Uses [zxcvbn](https://github.com/dropbox/zxcvbn) for realistic password scoring and crack time.
- **Entropy & crack time display:** See how strong your password is in bits, and get an estimate for how long it would take to crack it.
- **Basic checks:** Immediate feedback on length, uppercase, lowercase, numbers, and symbols.
- **Actionable feedback:** Suggestions and warnings for improving your password.
- **Colorful output:** Easy-to-read, color-coded terminal interface.

## Usage

### 1. Clone and build

```bash
git clone https://github.com/YOUR_USERNAME/password_strength_checker.git
cd password_strength_checker
cargo run
```

### 3. Example

```
🔐 Advanced Password Strength Checker
Type 'exit' to quit.

Enter password: Helloworld!

Length >= 12: ✘
Has uppercase: ✔
Has lowercase: ✔
Has number: ✘
Has symbol: ✔

Password Checks:
Score: 1/4
Entropy: 18.99 bits
Estimated crack time (offline, 10B/s): less than a second
Estimated crack time (online, 10/s): 14 hours
Warning: This is similar to a commonly used password.
- Add another word or two. Uncommon words are better.
- Capitalization doesn't help very much.

==================================================
```

Type `exit` to quit the program.
*note*: The actual CLI interface will show this color coded

## How it works

- Prompts the user for a password (input is visible for simplicity).
- Runs a series of basic checks (length, character types).
- Uses the `zxcvbn` crate to analyze the password's strength, entropy, and estimated crack times.
- Prints feedback and improvement suggestions from the zxcvbn algorithm.

## Dependencies

- [zxcvbn](https://crates.io/crates/zxcvbn) - Password strength estimator.
- [colored](https://crates.io/crates/colored) - Colored terminal output.

## License

MIT

## Credits

- Built upon [Dropbox's zxcvbn algorithm](https://github.com/dropbox/zxcvbn). Huge thanks to this dude for making my life easier and improving the performance of my tool!
