use std::fs;
use std::io::Read;
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
fn get_lang_structure(lang: String){
    let config_path =format!("./config/{}.json",lang);
    let mut file = fs::File::open(config_path).expect("could not open file");
    let mut  data = String::new();
    file.read_to_string(&mut data).expect("could not read data");
    
}

