use std::fs;
use std::io::Read;
use serde_json::Value;

use crate::args::project_args::ProjectArgs;
use file_handler::FileData;
use super::file_handler;


pub fn handle_project(args: ProjectArgs){
    create_root(args.name, args.path);
    let raw_json_data = get_lang_structure(args.lang);
    let app = get_app_type(args.typ, raw_json_data).expect("[SERDE] could not find app type");
    println!("{:?}",app);
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
fn get_app_type(app_type: String, raw_json_data: Value) -> Option<FileData>{
    let typ_list = raw_json_data.as_object().unwrap().get("types").expect("[SERDE] could not fetch app types");
    let typs: Vec<FileData> = serde_json::from_value(typ_list.clone()).unwrap();
    for val in typs{
        if val.typ == app_type{
            
            return Option::Some(val);
        }
        else{
            return Option::None;
        }
    }
    return Option::None;

}


