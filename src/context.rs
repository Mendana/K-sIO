use std::collections::HashMap;

pub struct Context {
    variables: HashMap<String, f64>,
}

impl Context {
    pub fn new() -> Self {
        let mut variables = HashMap::new();

        variables.insert("pi".to_string(), std::f64::consts::PI);
        variables.insert("e".to_string(), std::f64::consts::E);

        Context { variables }
    }

    pub fn get(&self, name: &str) -> Option<f64>{
        self.variables.get(name).copied()
    }

    pub fn set_ans(&mut self, value: f64) {
        self.set("ans".to_string(), value);
    }

    pub fn set(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }
}