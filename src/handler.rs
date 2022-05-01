mod project_handler;
mod file_handler;
use crate::Action;

pub fn handle_data(data: Action){
    match data{
        Action::New(values) =>  project_handler::handle_project(values),
        Action::Create(values) => println!("{:?}",values)
    }
}