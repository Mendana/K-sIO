use matheval::{
    context::AngleMode,
    error::{LexError, ParseError},
    evaluator::Evaluator,
    lexer::Lexer,
    parser::Parser,
    repl::CalcHelper,
};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    println!("=== Calculus Engine REPL ===");
    println!("Type 'exit' or 'quit' to leave the REPL.");
    println!("Type 'help' for available functions\n");
    println!("Use ↑/↓ to navigate through history.\n");
    println!("Use tab for autocompletion.\n");

    let mut evaluator = Evaluator::new();

    let mut helper = CalcHelper::new();
    helper.update_variables(evaluator.get_context().get_variables().keys().cloned().collect());
    let mut rl = Editor::new().expect("Failed to create REPL editor");
    rl.set_helper(Some(helper));

    let history_file = "calc_repl_history.txt";
    let _ = rl.load_history(history_file);


    loop {
        let readline = rl.readline("> ");
        
        match readline {
            Ok(line) => {
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(trimmed);
                
                match trimmed {
                    "exit" | "quit" => break,
                    "help" => print_help(),
                    "vars" => list_vars(&evaluator),
                    "deg" => set_angle_mode(&mut evaluator, AngleMode::Degrees),
                    "rad" => set_angle_mode(&mut evaluator, AngleMode::Radians),
                    "grad" => set_angle_mode(&mut evaluator, AngleMode::Gradians),
                    "mode" => show_mode(&evaluator),
                    "" => continue,
                    _ => {
                        process_input(trimmed, &mut evaluator);
                        if let Some(h) = rl.helper_mut() {
                            h.update_variables(evaluator.get_context().get_variables().keys().cloned().collect());
                        }
                    },
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Use 'exit' or 'quit' to leave the REPL.");
                continue;
            },
            Err(ReadlineError::Eof) => {
                break;
            },
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                break;
            }
        }
    }

    let _ = rl.save_history(history_file);
    println!("Goodbye");
}

fn process_input(input: &str, evaluator: &mut Evaluator) {
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(toks) => toks,
        Err(e) => {
            eprintln!("Lexing error: {}", e);
            if let Some(pos) = get_lexer_error_position(&e) {
                show_error_context(input, pos);
            }
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parsing error: {}", e);
            show_error_context(input, get_parse_error_position(&e));
            return;
        }
    };

    match evaluator.evaluate(&ast) {
        Ok(result) => println!("= {}", result),
        Err(e) => eprintln!("Evaluation error: {}", e),
    }
}

fn get_parse_error_position(error: &ParseError) -> usize {
    match error {
        ParseError::UnexpectedToken { position, .. } => *position,
        ParseError::InvalidExpression { position, .. } => *position,
        ParseError::UnexpectedEOF { position } => *position,
    }
}

fn get_lexer_error_position(error: &LexError) -> Option<usize> {
    match error {
        LexError::InvalidNumber(_) => None,
        LexError::UnexpectedCharacter(_, pos) => Some(*pos),
    }
}

fn show_error_context(input: &str, position: usize) {
    eprintln!("{}", input);
    eprintln!("{}^", " ".repeat(position));
}

fn list_vars(evaluator: &Evaluator) {
    let vars = evaluator.get_context().get_variables();

    if vars.is_empty() {
        println!("No variables defined.");
        return;
    }

    println!("Defined variables:");
    let mut vars: Vec<_> = vars.iter().collect();
    vars.sort_by(|a, b| a.0.cmp(b.0));

    for (name, value) in vars {
        println!("  {} = {}", name, value);
    }
}

fn set_angle_mode(evaluator: &mut Evaluator, mode: AngleMode) {
    evaluator.get_context_mut().set_angle_mode(mode);
    let mode_str = match mode {
        AngleMode::Degrees => "Degrees",
        AngleMode::Radians => "Radians",
        AngleMode::Gradians => "Gradians",
    };
    println!("Angle mode set to: {}", mode_str);
}

fn show_mode(evaluator: &Evaluator) {
    let mode_str = match evaluator.get_context().get_angle_mode() {
        AngleMode::Degrees => "Degrees (DEG)",
        AngleMode::Radians => "Radians (RAD)",
        AngleMode::Gradians => "Gradians (GRAD)",
    };
    println!("Current angle mode: {}", mode_str);
}

fn print_help() {
    println!("Available functions:");
    println!("  Trigonometric: sin(x), cos(x), tan(x), asin(x), acos(x), atan(x)");
    println!("  Logarithmic: ln(x), log(x)");
    println!("  Power & roots: sqrt(x), exp(x), pow(x,y)");
    println!("  Rounding: floor(x), ceil(x), round(x)");
    println!("  Other: abs(x), max(...), min(...)");
    println!("  Factorial: x!");
    println!("\nConstants: PI, E");
    println!("Operators: +, -, *, /, ^");
    println!("\nCommands:");
    println!("  deg   - Set angle mode to degrees (default)");
    println!("  rad   - Set angle mode to radians");
    println!("  grad  - Set angle mode to gradians");
    println!("  mode  - Show current angle mode");
    println!("  vars  - List all defined variables");
    println!("  help  - Show this help");
    println!("  exit  - Exit the REPL");
}
