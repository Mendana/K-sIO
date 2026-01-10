use matheval::{evaluator::Evaluator, lexer::{Lexer}, parser::Parser};
use std::{io::{self, Write}}; 

fn main() {
    println!("=== Calculus Engine REPL ===");
    println!("Type 'exit' or 'quit' to leave the REPL.");
    println!("Type 'help' for available functions\n");

    let mut evaluator = Evaluator::new();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let trimmed = input.trim();

        match trimmed {
            "exit" | "quit" => break,
            "help" => print_help(),
            "vars" => list_vars(&evaluator),
            "" => continue,
            _ => process_input(trimmed, &mut evaluator),
        }
    }

    println!("Goodbye");
}

fn process_input(input: &str, evaluator: &mut Evaluator) {
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(toks) => toks,
        Err(e) => {
            eprintln!("Lexing error: {}", e);
            return;
        },
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parsing error: {}", e);
            return;
        },
    };

    match evaluator.evaluate(&ast) {
        Ok(result) => println!("= {}", result),
        Err(e) => eprintln!("Evaluation error: {}", e),
    }
}

fn list_vars(evaluator: &Evaluator) {
    let vars = evaluator.get_context().get_variables();

    if vars.is_empty() {
        println!("No variables defined.");
        return;
    }

    println!("Defined variables:");
    for (name, value) in vars {
        println!("  {} = {}", name, value);
    }
}

fn print_help() {
    println!("Available functions:");
    println!("  sin(x), cos(x), tan(x), asin(x), acos(x), atan(x) - Trigonometric functions");
    println!("  sqrt(x), abs(x), exp(x), pow(x, y), max(x, y), min(x, y), ceil(x), floor(x), round(x) - Common mathematical functions");
    println!("  ln(x), log(x) - Natural and base-10 logarithm");
    println!("  x! - Factorial");
    println!("\nConstants: pi, e");
    println!("Operators: +, -, *, /, ^");
    println!("You can assign variables using the '=' operator, e.g., x = 5");
    println!("Type 'vars' to see all defined variables.");
    println!("Type 'exit' or 'quit' to leave the REPL.\n");
}