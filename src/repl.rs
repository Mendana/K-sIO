use rustyline::{Helper, completion::{Completer, Pair}, highlight::Highlighter, hint::Hinter, validate::Validator};

pub struct CalcHelper {
    functions: Vec<String>,
    commands: Vec<String>,
    variables: Vec<String>,
}

impl CalcHelper {
    pub fn new() -> Self {
        CalcHelper { 
            functions: vec![
                "sin".to_string(),
                "cos".to_string(),
                "tan".to_string(),
                "asin".to_string(),
                "acos".to_string(),
                "atan".to_string(),
                "sqrt".to_string(),
                "abs".to_string(),
                "ln".to_string(),
                "log".to_string(),
                "exp".to_string(),
                "floor".to_string(),
                "ceil".to_string(),
                "round".to_string(),
                "pow".to_string(),
                "max".to_string(),
                "min".to_string(),
            ],
            commands: vec![
                "help".to_string(),
                "exit".to_string(),
                "quit".to_string(),
                "vars".to_string(),
                "deg".to_string(),
                "rad".to_string(),
                "grad".to_string(),
                "mode".to_string(),
            ],
            variables: vec![],
        }
    }

    pub fn update_variables(&mut self, vars: Vec<String>) {
        self.variables = vars;
    }

    pub fn add_variable(&mut self, var: String) {
        if !self.variables.contains(&var) {
            self.variables.push(var);
        }
    }
}

impl Completer for CalcHelper {
    type Candidate = Pair;

    fn complete(
            &self, // FIXME should be `&mut self`
            line: &str,
            pos: usize,
            _ctx: &rustyline::Context<'_>,
        ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut candidates = Vec::new();

        let word_start = line[..pos]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);

        let word = &line[word_start..pos];

        if word.is_empty() {
            return Ok((pos, candidates));
        }

        for func in &self.functions {
            if func.starts_with(word) {
                candidates.push(Pair {
                    display: func.clone(),
                    replacement: func.clone(),
                });
            }
        }

        for var in &self.variables {
            if var.starts_with(word) {
                candidates.push(Pair {
                    display: var.clone(),
                    replacement: var.clone(),
                });
            }
        }

        if word_start == 0 {
            for cmd in &self.commands {
                if cmd.starts_with(word) {
                    candidates.push(Pair {
                        display: cmd.clone(),
                        replacement: cmd.clone(),
                    });
                }
            }
        }
        Ok((word_start, candidates))
    }
}

impl Hinter for CalcHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        if pos < line.len() {
            return None;
        }

        let word_start = line
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);

        let word = &line[word_start..pos];

        if word.is_empty() {
            return None;
        }

        for func in &self.functions {
            if func.starts_with(word) && func != word {
                return Some(func[word.len()..].to_string());
            }
        }

        for var in &self.variables {
            if var.starts_with(word) && var != word {
                return Some(var[word.len()..].to_string());
            }
        }

        if word_start == 0 {
            for cmd in &self.commands {
                if cmd.starts_with(word) && cmd != word {
                    return Some(cmd[word.len()..].to_string());
                }
            }
        }
        None
    }
}

impl Highlighter for CalcHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        format!("\x1b[90m{}\x1b[0m", hint).into()
    }
}
impl Validator for CalcHelper {}
impl Helper for CalcHelper {}