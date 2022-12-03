use std::path::PathBuf;

use sledge_config::{get_config, Configuration};

pub fn parse(dir: &str) -> &Configuration {
    let config_dir = PathBuf::from(&format!("tests/{}", dir));
    let config_dir = config_dir.canonicalize().unwrap();
    println!("Setting environment variable to load from {:?}", config_dir);
    std::env::set_var(
        "SLEDGE_CONFIG_ROOT",
        &config_dir.to_string_lossy().to_string(),
    );

    match get_config() {
        Ok(c) => c,
        Err(e) => panic!("Failed to parse configuration; error: {}", e),
    }
}
