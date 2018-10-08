extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;


mod envsource;
mod mappings;
mod plugin_params;

use std::process::{Command,exit};
use clap::{Arg, App};
use mappings::drone_envs;
use plugin_params::*;
use std::env::var;



fn main() {
    let cmdline = App::new("dronelab")
       .version("0.1.0")
       .about("Drone plugin wrapper for Gitlab CI")
       .author("Miroslav Prasil")
       .arg(Arg::with_name("plugin_params")
            .short("p")
            .help("Plugin parameter provided as commandline option")
            .takes_value(true)
            .number_of_values(2)
            .value_names(&["key", "value"])
            .multiple(true)
       )
       .get_matches();

    let status = Command::new(var("DRONELAB").unwrap())
        .envs(drone_envs())
        .envs(get_env_hashmap(var("plugin")).unwrap())
        .envs(get_cmdline_hashmap(cmdline.values_of("plugin_params")).unwrap())
        .status()
        .expect("Failed to run binary");

    exit(status.code().unwrap_or(0));
}