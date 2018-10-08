use std::env::var;

pub trait EnvValue {
    fn get(&self) -> Option<String>;
}


// Use ${name} variable value or provided default value
pub struct EnvDefault {
    name: String,
    default: String,
}

impl EnvDefault {
    pub fn define(name: &str, default: &str) -> Self {
        Self {
            name: name.to_string(),
            default: default.to_string(),
        }
    }
}

impl EnvValue for EnvDefault {
    fn get(&self) -> Option<String> {
        Some(var(&self.name).unwrap_or(self.default.clone()))
    }
}

// Use ${name} variable or skip the variable (return None)
pub struct EnvSkip {
    name: String,
}

impl EnvValue for EnvSkip {
    fn get(&self) -> Option<String> {
        var(&self.name).ok()
    }
}

impl EnvSkip {
    pub fn define(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}


