use std::fs;

use minijinja::{Environment, context};
use cached::proc_macro::cached;

const BASE_PROMPT_FILE_PATH: &str = "./prompts/base.jinja";

#[cached]
fn load_prompt(query: &str) -> &str {
    let mut env = Environment::new();
    env.add_template("memer", load_file(BASE_PROMPT_FILE_PATH)).unwrap();
    let template = env.get_template("memer");

    ""
}

#[cached]
fn load_file<'a>(path: &str) -> &'a str {
    let file_content = fs::read_to_string(path).unwrap();
    &file_content
}
