use std::{fs, io::Read};
use serde_json::Value;

use crate::args::project_args::ProjectArgs;

use super::file_handler;

pub fn handle_project(args: ProjectArgs){
    create_root(args.name, args.path);
    let raw_json_data = get_lang_structure(args.lang);
    let parsed_data = parse_by_app_type(args.typ, raw_json_data);

}

fn create_root(name: String, path:String){
    let path_name= path+"/"+&name;
    match fs::create_dir_all(&path_name){
        Err(e)=> println!("{}",e),
        Ok(_)=>println!("projects root dir created successfully at:\n{}",path_name)
    };
}
fn get_lang_structure(lang: String) -> Value{
    let config_path =format!("./config/{}.json",lang);
    let mut file = fs::File::open(config_path).expect("[FS] could not open file");
    let mut string_data = String::new();
    file.read_to_string(&mut string_data).expect("[FILE READER] could not read data");
    serde_json::from_str(string_data.as_str()).expect("[SERDE] could not parse file to json")
}
fn parse_by_app_type(app_type: String, raw_json_data: Value) -> FileData{
    let app_data = raw_json_data[app_type].as_object().expect("[SERDE} could not parse json app to map");
    // file_handler::new(app_data["initial_commands"].as_array_mut(), default_structure, default_files)
}

