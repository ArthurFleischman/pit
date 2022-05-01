use std::{fs, io::Read};
use serde_json;

use crate::args::project_args::ProjectArgs;

pub fn handle_project(args: ProjectArgs){
    create_root(args.name, args.path);
    get_lang_structure(args.lang);
}

fn create_root(name: String, path:String){
    let path_name= path+"/"+&name;
    match fs::create_dir_all(&path_name){
        Err(e)=> println!("{}",e),
        Ok(_)=>println!("projects root dir created successfully at:\n{}",path_name)
    };
}
fn get_lang_structure(lang: String) -> serde_json::Value{
    let config_path =format!("./config/{}.json",lang);
    let mut file = fs::File::open(config_path).expect("[FS] could not open file");
    let mut string_data = String::new();
    file.read_to_string(&mut string_data).expect("[FILE READER] could not read data");
    serde_json::from_str(string_data.as_str()).expect("[SERDE] could not parse file to json")
}

