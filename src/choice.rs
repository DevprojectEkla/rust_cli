extern crate regex;
use regex::Regex;
use std::{io::{self, Write}, fs};
use crate::cli_utils;
use cli_utils::{print_c_no_nl, text, info, warn, error, success, WHITE};

/// The public Choice Structure allows developper to manage user input in CLI applications without
/// creating loop by themselves. 
/// the different loop_methods provides a convenient way of handling input from users. For example
/// the yes_or_no_loop can be used every time a user must answer by yes or no and the simple
/// Choice::new().yes_or_no_loop() call will check if the answer is 'yes' and return true or 'no'
/// and return false.
pub struct Choice {
    ///you tipically need to provide a string to ask a question to the user (ex:
    ///let mut choice = Choice::new()
    ///choice.question="Do you want to go fishing?".to_string())
    pub question: String, 
    ///you can provide a vector of possible response ['yes', 'no']
    pub answer_prop: Vec<String>,
    ///should be private but needed to be public for testing
    pub user_input: String,
    ///you might want to choose a custom prompt symbol for the user, Default is '=>'
    pub prompt: String,
    ///choose wether or not you want to print the answer_prop vector
    pub print: bool,

}
impl Choice {
    pub fn new() -> Choice {
        Choice { question: String::new(),
        answer_prop: vec!["yes".to_string(), "no".to_string()],
        user_input: String::new(),
        prompt: "=> ".to_string(),
        print: true }
    }
    ///this method is private and should not be use directly
    fn print_answer_prop(&mut self){
        if self.print {
            info(format!("{:?}", self.answer_prop).as_str());
            
        };
    }
    fn check_input(&mut self, input:String) -> bool {
    match input.as_str()  {"yes" | "y" | "no" | "n" => true,
                            _ => false,
        
}
    }
///this private method is used for the public method yes_or_no_loop()
fn check_yes(&mut self, input:String) -> bool {
    match input.as_str() {
        "yes" | "y" => true,
        _ => false,
        
    }
    
}

fn check_exit(&mut self, input:String) -> bool {
    match input.as_str() {
        "exit" | "cancel" => true,
        _ => false
    }

    
}

fn display_question(&mut self) {
print_c_no_nl(format!("{}\r\n",self.question).as_str(),WHITE);
    print_c_no_nl(format!("{}", self.prompt).as_str(),WHITE);
    io::stdout().flush().expect("failed to flush")
}
fn get_user_input(&mut self) -> String {
    // we need to clean this variable first
    self.user_input = String::new();
    //then we print the question
    self.display_question();
    //and we read from stdin
    io::stdin().read_line(&mut self.user_input).expect("failed to read line");
    self.user_input.clone().trim().to_string()
 

}
///this methods provides the typical use of the Choice structure by starting a loop waiting for a
///user input and checking if the user answer correctly, looping if he is not and returning a
///boolean if he gives a correct answer (yes -> true, no -> false)
    pub fn yes_or_no_loop(&mut self) -> bool 
    {
        loop 
        {
           let input = self.get_user_input();
           if self.check_exit(input) {
                println!("bye bye !");
                std::process::exit(0)
            }

            if self.check_input(self.user_input.clone().trim().to_string()) {
                if self.check_yes(self.user_input.clone().trim().to_string()) {
                    return true
                }
                else {
                    return false
                    
                }}
                else {
                    text("the answer must be chosen among\n");
                    self.print_answer_prop();

                }


            
        }

    } 
/// you might need this one to check the input to match a given pattern
pub fn check_pattern_loop(&mut self, pattern: &str) -> String {
    
    loop {
        let input = self.get_user_input();

        let test = self.check_pattern(pattern);
        match test {
            true => {
                success(format!("the input {} is correct",input).as_str());
return input
            },
            false => {
                warn(format!("the pattern {} is required for this input", pattern).as_str());
                info(format!("input:{}",input).as_str());
                continue

            },
        }
        

    }




    
}

fn check_pattern(&mut self, pattern: &str) -> bool
{
    let regexp = Regex::new(pattern).unwrap();
    let input = self.user_input.as_str().trim();
    if regexp.is_match(input) {
            success("correct input");
            info(format!("you chose {}",self.user_input).as_str());
            return true             
        }
        else {
        error(format!("the pattern does not match {}",pattern).as_str());
        return false
        }
}

fn check_file(&mut self) -> bool 
    {
        let input = self.user_input.trim().to_string();
    if fs::metadata(input.clone()).is_ok()
        {
            success(format!("the file {} exist", input).as_str());
            return true
        }
    else 
        {
            return false
        }

    }
// this one is a bit tricky, first we try to get the metadata from the path given by user_input and
// if it goes well we return the .is_dir result which is true or false
fn check_dir(&mut self) -> Result<bool, std::io::Error>
    {
        match fs::metadata(self.user_input.clone().trim()) 
        {
        
            Ok(meta) => Ok(meta.is_dir()),
            Err(err) => Err(err),
        }

    }
///the typical loop to ask the user for the path to a file
pub fn choose_file_loop(&mut self) 
    {
        loop
        {
            self.get_user_input();
            let path = self.check_file();

            match path {
                true =>   {
                            success("correct path to a file"); 
                            break
                          },
                false => {error("Incorrect path, choose an existing file");
                continue},

                
            }
            
        }


    }
/// the typical loop to ask the user for a path to a directory
pub fn choose_dir_loop(&mut self) 
    {
        loop
        {
            let input = self.get_user_input();
            let path = self.check_dir();

            match path {
                Ok(_) =>  {
                            success(format!("{} is a correct path to a directory",input).as_str()); 
                            break
                          },
                Err(err) => {error(format!("Error: {}. Incorrect path, choose an existing directory", err).as_str());
                continue},

                
            }
            
        }


    }


}//Impl

#[test]
fn check_pattern_test(){
let mut choix = Choice::new();
choix.question = "taper un nom commençant par 3 chiffres:".to_string();
let test = choix.check_pattern_loop(r"^\d{3}");
let input = choix.user_input;
assert_eq!(test,input);
}

#[test]
fn yes_or_no_loop_test_no() {
let mut choix = Choice::new();

choix.question = "Paris est la capital du Brésil ?".to_string();
let test = choix.yes_or_no_loop();
assert_eq!(test, false)
}
#[test]
fn yes_or_no_loop_test_yes() {
let mut choix = Choice::new();

choix.question = "Paris est la capital de la France ?".to_string();
let test = choix.yes_or_no_loop();
assert_eq!(test, true)
}
