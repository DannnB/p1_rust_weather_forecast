mod custom_error;
use reqwest;

pub fn fetch_todos(url: &str) -> Result<Vec<Todo>, custom_error::CustomError> {
    let response = reqwest::blocking::get(url)?.text()?;
    let todos: Vec<Todo> = serde_json::from_str(&response)?;
    Ok(todos)
}