use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FileData {
    pub initial_commands: Vec<String>,
    pub default_structure: Vec<String>,
    pub default_files: Vec<String>
}
