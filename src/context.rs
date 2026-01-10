use std::collections::HashMap;

pub struct Context {
    variables: HashMap<String, f64>,
    angle_mode: AngleMode,
}

impl Context {
    pub fn new() -> Self {
        let mut variables = HashMap::new();

        variables.insert("PI".to_string(), std::f64::consts::PI);
        variables.insert("E".to_string(), std::f64::consts::E);

        Context { 
            variables,
            angle_mode: AngleMode::Degrees
        }
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

    pub fn get_variables(&self) -> &HashMap<String, f64> {
        &self.variables
    }

    pub fn get_angle_mode(&self) -> AngleMode {
        self.angle_mode
    }

    pub fn set_angle_mode(&mut self, mode: AngleMode) {
        self.angle_mode = mode;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleMode {
    Degrees,
    Radians,
    Gradians
}