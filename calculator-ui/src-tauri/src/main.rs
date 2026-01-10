// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use matheval::{
    evaluator::Evaluator,
    lexer::Lexer,
    parser::Parser,
    context::AngleMode,
};
use std::sync::Mutex;
use tauri::State;

struct AppState {
    evaluator: Mutex<Evaluator>,
}

#[derive(serde::Serialize)]
struct EvalResult {
    success: bool,
    result: Option<f64>,
    error: Option<String>,
}

#[tauri::command]
fn evaluate(expression: String, state: State<AppState>) -> EvalResult {
    let mut evaluator = state.evaluator.lock().unwrap();
    
    let mut lexer = Lexer::new(&expression);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => return EvalResult {
            success: false,
            result: None,
            error: Some(format!("Lexing error: {}", e)),
        },
    };
    
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => return EvalResult {
            success: false,
            result: None,
            error: Some(format!("Parsing error: {}", e)),
        },
    };
    
    match evaluator.evaluate(&ast) {
        Ok(result) => EvalResult {
            success: true,
            result: Some(result),
            error: None,
        },
        Err(e) => EvalResult {
            success: false,
            result: None,
            error: Some(format!("Evaluation error: {}", e)),
        },
    }
}

#[tauri::command]
fn get_variables(state: State<AppState>) -> Vec<(String, f64)> {
    let evaluator = state.evaluator.lock().unwrap();
    let vars = evaluator.get_context().get_variables();
    vars.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

#[tauri::command]
fn set_angle_mode(mode: String, state: State<AppState>) -> Result<(), String> {
    let mut evaluator = state.evaluator.lock().unwrap();
    let angle_mode = match mode.as_str() {
        "deg" => AngleMode::Degrees,
        "rad" => AngleMode::Radians,
        "grad" => AngleMode::Gradians,
        _ => return Err("Invalid angle mode".to_string()),
    };
    evaluator.get_context_mut().set_angle_mode(angle_mode);
    Ok(())
}

#[tauri::command]
fn get_angle_mode(state: State<AppState>) -> String {
    let evaluator = state.evaluator.lock().unwrap();
    match evaluator.get_context().get_angle_mode() {
        AngleMode::Degrees => "deg".to_string(),
        AngleMode::Radians => "rad".to_string(),
        AngleMode::Gradians => "grad".to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            evaluator: Mutex::new(Evaluator::new()),
        })
        .invoke_handler(tauri::generate_handler![
            evaluate,
            get_variables,
            set_angle_mode,
            get_angle_mode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}