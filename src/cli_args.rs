use std::env;

use crate::cli_utils::info; 
///A structure to abstract over primitive rust std function for parsing arguments of a CLI
///application
pub struct ParseArguments {
    project_name: String,
    args: Vec<String>,
    arg1: String,
    arg2: String,
    arg3:String,
    usage: String,
    options: Vec<String>,

}
impl ParseArguments {
    pub fn new() -> Self { Self
        {
        project_name: String::new(),
        args: Vec::new(),
        usage: String::new(),
        options: Vec::new(),
        arg1: String::new(),
        arg2: String::new(),
        arg3: String::new(),
    }
    }
fn print_args(&mut self) -> Vec<String>{
    info(format!("{:?}",self.args).as_str());
    return self.args.clone()

}

fn collect_args(&mut self){
    self.args = env::args().collect();
    self.print_args();
    self.project_name = self.args[0].clone();
    self.arg1 = self.args[1].clone();
    self.arg2 = self.args[2].clone();
    self.arg3 = self.args[3].clone();

}



}
