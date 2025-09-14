//! why Box<str>? the options that we have is either String (which would mean it will be mutable
//! (which we do not indeded to do), owned, etc...) or a Arc or Rc, as they seem unrequired for this setup at the moment, i have
//! utilized a Box<str>

use std::fs;

use minijinja::{context, Environment};
use uuid::Uuid;

const BASE_PROMPT_FILE_PATH: &str = "src/prompts/base.jinja";

pub fn load_prompt(query: &str) -> Box<str> {
    let mut env = Environment::new();
    let loaded_prompt: &str = &load_file(None);
    env.add_template("memer", loaded_prompt).unwrap();
    let template = env.get_template("memer").unwrap();
    template
        .render(context!(query => query))
        .unwrap()
        .to_owned()
        .into_boxed_str()
}

fn load_file(path: Option<&str>) -> Box<str> {
    fs::read_to_string(path.unwrap_or(BASE_PROMPT_FILE_PATH))
        .unwrap()
        .to_owned()
        .into_boxed_str()
}

pub fn get_default_uuid_directory() -> String {
    Uuid::new_v4().to_string()
}
