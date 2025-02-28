use std::{env, fs, io::Write, process::Command};
use colored::Colorize;

/// Used for parsing
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Word,
    Comp,
    Args,
    Symbol,
    StringLiteral,
}

/// Used for parsing
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

impl Token {
    pub fn new(lexeme: String) -> Token {
        match lexeme.as_str() {
            "COMP" => Token { kind: TokenKind::Comp, lexeme },
            "COMPILER" => Token { kind: TokenKind::Comp, lexeme },
            "ARGS" => Token { kind: TokenKind::Args, lexeme },
            "ARGUMENTS" => Token { kind: TokenKind::Args, lexeme },
            _ => Token { kind: TokenKind::Word, lexeme },
        }
    }
}

/// Stores parsed input
pub struct UnifySetup {
    compiler: String,
    args: Vec<String>,
}

/// Parses Unify compiler-section
fn parse_unify_comp_section(tokens: &[Token], index: &mut usize) -> String {
    if *index + 2 < tokens.len() {
        if tokens[*index + 1].lexeme == ":" && tokens[*index + 2].kind == TokenKind::StringLiteral {
            *index += 2;
            return tokens[*index].lexeme.clone();
        } else {
            panic!(
                "[{}]: {}\n",
                "UNIFY".magenta().bold(),
                "Expected ':' followed by a string literal".red().bold()
            );
        }
    } else {
        panic!(
            "[{}]: {}",
            "UNIFY".magenta().bold(),
            "Unexpected end of file; expected ':' followed by a string literal".red().bold()
        );
    }
}

/// Parses Unify arguments-section
fn parse_unify_args_section(tokens: &[Token], index: &mut usize) -> Vec<String> {
    let mut args = Vec::new();
    if *index + 2 < tokens.len() {
        if tokens[*index + 1].lexeme == ":" && tokens[*index + 2].kind == TokenKind::StringLiteral {
            *index += 2;
            loop {
                if *index >= tokens.len() {
                    break;
                }
                if tokens[*index].kind == TokenKind::StringLiteral {
                    args.push(tokens[*index].lexeme.clone());
                    *index += 1;
                    if *index < tokens.len() && tokens[*index].lexeme == "," {
                        *index += 1;
                    }
                } else {
                    break;
                }
            }
        } else {
            panic!(
                "[{}]: {}",
                "UNIFY".magenta().bold(),
                "Expected ':' followed by at least one string literal for ARGS".red().bold()
            );
        }
    } else {
        panic!(
            "[{}]: {}",
            "UNIFY".magenta().bold(),
            "Unexpected end of file; expected ':' followed by a string literal".red().bold()
        );
    }
    args
}

impl UnifySetup {
    /// Creates a 'UnifySetup' unit
    pub fn new() -> Self {
        Self { compiler: String::new(), args: Vec::new() }
    }

    /// Parses given input to Unify-context
    pub fn parse(content: String) -> Self {
        let chars: Vec<char> = content.chars().collect();
        let mut tokens = Vec::new();
        let mut index = 0;

        while index < chars.len() {
            match chars[index] {
                '#' => {
                    while index < chars.len() && chars[index] != '\n' {
                        index += 1;
                    }
                }
                c if c.is_alphabetic() => {
                    let start = index;
                    while index < chars.len() && chars[index].is_alphabetic() {
                        index += 1;
                    }
                    let word: String = chars[start..index].iter().collect();
                    tokens.push(Token::new(word));
                    continue;
                }
                '\"' => {
                    index += 1;
                    if index < chars.len() && chars[index] == '\"' {
                        tokens.push(Token { kind: TokenKind::StringLiteral, lexeme: String::new() });
                        index += 1;
                        continue;
                    }
                    let mut str_lit = String::new();
                    while index < chars.len() && chars[index] != '\"' {
                        if chars[index] == '\n' {
                            panic!(
                                "[{}]: {}",
                                "UNIFY".magenta().bold(),
                                "Unterminated string literal".red().bold()
                            );
                        }
                        str_lit.push(chars[index]);
                        index += 1;
                    }
                    if index >= chars.len() || chars[index] != '\"' {
                        panic!(
                            "[{}]: {}",
                            "UNIFY".magenta().bold(),
                            "Unterminated string literal".red().bold()
                        );
                    }
                    tokens.push(Token { kind: TokenKind::StringLiteral, lexeme: str_lit });
                }
                c if c.is_whitespace() => {
                    // Skip whitespace.
                }
                c => {
                    tokens.push(Token { kind: TokenKind::Symbol, lexeme: c.to_string() });
                }
            }
            index += 1;
        }

        let mut setup = UnifySetup::new();
        let mut token_index = 0;
        while token_index < tokens.len() {
            match tokens[token_index].kind {
                TokenKind::Comp => {
                    setup.compiler = parse_unify_comp_section(&tokens, &mut token_index);
                }
                TokenKind::Args => {
                    setup.args = parse_unify_args_section(&tokens, &mut token_index);
                }
                _ => {
                    panic!(
                        "[{}]: {} {}\n",
                        "UNIFY".magenta().bold(),
                        "Unexpected token:".red().bold(),
                        format!("{:?}", tokens[token_index]).red().bold()
                    );
                }
            }
            token_index += 1;
        }

        setup
    }

    /// Returns the setup's compiler
    pub fn compiler(&self) -> &String {
        &self.compiler
    }

    /// Returns the setup's arguments
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}

/// Creates valid Unify file content for the given input
fn create_unify_file_content(args: &Vec<String>) -> String {
    let mut file_content = String::from("# Build script\n\nCOMP: ");
    file_content.push_str(&args[2]);
    file_content.push_str("\n\nARGS:\n");

    for i in 3..args.len() {
        file_content.push_str("  \"");
        file_content.push_str(&args[i]);
        file_content.push_str("\",\n");
    }

    file_content
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!(
            "[{}]: {}\nUsage: {} <path to .u file>\n{}: Additional arguments must follow the correct order for the compiler.\n",
            "UNIFY".magenta().bold(),
            "Missing file path argument".red().bold(),
            "unify".magenta().bold(),
            "Note".blue().bold()
        );
    }

    if args[1] == "--new" || args[1] == "--n" {
        let mut unify_file = fs::File::create("build.u").unwrap();
        let content = create_unify_file_content(&args);
        unify_file.write_all(content.as_bytes()).unwrap();
        println!("[{}] {}", "UNIFY".magenta().bold(), "Successfully created 'build.u'\n".green().bold());
    } else {
        let contents = fs::read_to_string(&args[1]).unwrap();

        let us = UnifySetup::parse(contents);

        print!(
            "[{}] {}\nCompiler: {}\nArguments: {}\nCommand-line output: {}",
            "UNIFY".magenta().bold(),
            "Config".magenta().bold(),
            us.compiler().yellow().bold(),
            us.args().len().to_string().green().bold(),
            us.compiler().yellow()
        );
        
        for arg in us.args() {
            print!(" {}", arg)
        }

        println!("\n");

        let output = Command::new(us.compiler())
            .args(us.args())
            .output()
            .expect(&format!(
                "[{}]: {}",
                "UNIFY".magenta().bold(),
                "Failed to execute command".red().bold()
            ));

        if output.status.success() {
            println!("[{}] {}\n", "UNIFY".magenta().bold(), "Finished building".green().bold());
        } else {
            eprintln!(
                "[{}]: {}\n{}",
                "UNIFY".magenta().bold(),
                "Building failed".red().bold(),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
