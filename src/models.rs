use serde::Deserialize;


#[derive(Deserialize)]
pub struct ProjectInfo {
    name: String,
    ascii_art: Vec<String>,
    about: Vec<String>,
    url: String,
}
