use std::{env, fs};

pub fn set_env() {
    let file = fs::read_to_string(".env").unwrap();

    for var in file.split('\n') {
        let key_value : Vec<&str> = var.split('=').collect();

        env::set_var(key_value[0], key_value[1]);
    }
}