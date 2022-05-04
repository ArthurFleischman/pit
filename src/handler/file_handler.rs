use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct FileData {
    pub typ: String,
    pub initial_commands: Vec<String>,
    pub default_structure: Vec<String>,
    pub default_files: Vec<String>
}
// fn new(typ:String, initial_commands:Vec<String>,default_structure:Vec<String>,default_files:Vec<String>)-> FileData{
//     FileData {typ: typ,initial_commands:initial_commands, default_structure: default_structure, default_files: default_files}
// }


