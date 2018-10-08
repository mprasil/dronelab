use std::collections::HashMap;
use std::env::VarError;
use clap::Values;
use serde_json::{self, Value};
use serde_yaml;

pub type PluginParams = HashMap<String, String>;

pub fn get_env_hashmap(var: Result<String,VarError>) -> Result<PluginParams, String> {
    match var {
        Ok(yaml) => {
            let mut plugin_params = HashMap::new();
            let params: serde_json::Value = if let Ok(params) = serde_yaml::from_str(&yaml) { 
                params
            } else {
                return Err(String::from("Failed to parse provided PLUGIN yaml"))
            };
             
            if let Value::Object(params) = params {
                for (key, value) in params {
                    plugin_params.insert(
                        format!("PLUGIN_{}",key.to_uppercase()),
                        match value {
                            Value::String(string) => string,
                            Value::Number(number) => number.to_string(),
                            Value::Object(_) => value.to_string(),
                            Value::Array(ref values) => {
                                if values.iter().all(|ref x| { 
                                    x.is_number() || x.is_string() 
                                }) {
                                    values.iter()
                                    .map(|ref x| {
                                        if let Value::String(string) = x {
                                            string.clone()
                                        } else {
                                            x.to_string()
                                        }
                                    })
                                    .collect::<Vec<String>>().join(",")
                                } else { value.to_string() }
                            }
                            _ => return Err(String::from("Failed to parse the value"))
                        }
                    );
                }
            } else {
                return Err(String::from("Failed to parse plugin parameters"));
            };
            Ok(plugin_params)
        },
        Err(_) => Ok(PluginParams::new())
    }
}

pub fn get_cmdline_hashmap(cmdline: Option<Values>) -> Result<PluginParams, String> {
    match cmdline {
        Some(values) => {
            let mut plugin_params = HashMap::new();
            for param in values.collect::<Vec<&str>>().chunks(2) {
                if param.len() == 2 {
                    plugin_params.insert(
                        format!("PLUGIN_{}", param[0].to_uppercase()),
                        String::from(param[1])
                    );
                } else { // huh? clap should never allow this
                    return Err(String::from("Failed to parse commandline plugin parameters"));
                }
            };
            Ok(plugin_params)
        },
        None => Ok(PluginParams::new())
    }
}
