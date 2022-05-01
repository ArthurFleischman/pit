use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
struct FileData {
    pub initial_commands: Vec<String>,
    pub default_structure: Vec<String>,
    pub default_files: Vec<String>
}
pub fn new(initial_commands: Vec<String>,default_structure: Vec<String>,default_files: Vec<String>)-> FileData{
    FileData { initial_commands: initial_commands, default_structure: default_structure, default_files: default_files }
}
