use std::io;
use std::fs;
use std::path::Path;
use clap::{Parser, Subcommand};


/// Simple program for cli utilities
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Echo command
    Echo { name: Option<String> },
    /// Cat command
    Cat { path: Option<String> },
    /// LS command
    LS { name: Option<String> },
    /// Find command
    Find { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();
    let mut exit: bool = false;
    let mut count: i32  = 0;
    match &cli.command {
        Some(Commands::Echo { name }) => {
            println!("'myapp add' was used, name is: {:?}", name)
        }
        Some(Commands::Cat { path }) => {
            cat(path.clone());
        }
        Some(Commands::LS { name }) => {
            println!("'myapp add' was used, name is: {:?}", name)
        }
        Some(Commands::Find { name }) => {
            println!("'myapp add' was used, name is: {:?}", name)
        }
        None => {
            println!("Default subcommand");
        }
    }

    println!("Hello, world!");
    while exit == false && count < 5 {
        let mut input: String = String::new();
        println!("   - To repeat  like echo : Echo ");
        println!("   - To cat a file : Cat ");
        println!("   - To list a directory : LS ");
        println!("   - To find a file in a folder : Find ");
        println!("   - exit : Exit ");
        println!("Please make a choice :");
        io::stdin().read_line(&mut input)
        .expect("Failed to read line");
        interpret_input(&input.trim(),&mut exit);
        count+=1;
    };
}

fn interpret_input (input:&str,exit:&mut bool, ){


    if input.to_lowercase().eq("echo"){
        echo();
        return;
    }
    if input.to_lowercase().eq("cat"){
        cat(None);
        return;
    }
    if input.to_lowercase().eq("ls"){
        list_repetory();
        return;
    }
    if input.to_lowercase().eq("exit"){
        *exit=true;
        return;
    }
    println!("Votre input n'est pas correct, recommencez svp !");
}

fn interpret_input_from_args (func:&str,input:&str,exit:&mut bool){


    if func.to_lowercase().eq("echo"){
        println!("______________________\n{}\n______________________",input.trim());
        return;
    }
    if func.to_lowercase().eq("cat"){
        cat(Some(input.to_string()));
        return;
    }
    if func.to_lowercase().eq("exit"){
        *exit=true;
        return;
    }
    println!("Votre input n'est pas correct, recommencez svp ");
}

fn cat(path: Option<String>){
    let mut cat_path: String = String::new();
    match path {
        Some(p) =>  cat_path = p,
        None => {
            println!("Please enter path to text : ");
            io::stdin().read_line(&mut cat_path)
                .expect("Failed to read line");
        }
    };
    let contents = fs::read_to_string(cat_path.trim())
        .expect("Should have been able to read the file");
    println!("Please, take your file to read :\n{contents}");
}

fn echo() {
    let mut string_echo: String = String::new();
    println!("Please enter your msg : ");
    io::stdin().read_line(&mut string_echo)
    .expect("Failed to read line");
    println!("______________________\n{}\n______________________",string_echo.trim());
}

fn list_repetory (){
    let mut string_path: String = String::new();
    println!("Please enter your path to list : ");
    io::stdin().read_line(&mut string_path)
    .expect("Failed to read line");
    let path = Path::new(string_path.trim());
    if !path.exists() || !path.is_dir(){
        println!("it's not a repertory");
        return;
    };
    for file in fs::read_dir(path).unwrap(){
        println!("{}", file.unwrap().path().display());
    }
}