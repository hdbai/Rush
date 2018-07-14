extern crate libc;
use std::io::*;
use std::string::*;
use std::env;
use std::ffi::CString;
use std::process::exit;
use std::process::Command;



fn run(input : &str, history : &Vec<String>) -> () {
    let mut cmds : Vec<_> = input.split(" | ").collect();
    let mut is_background = false;
    for cmd in 0..cmds.len() {
        if cmd != 0 && match cmds[cmd].find("<") { Some(_) => true, None => false,} {return;}
        if cmd != cmds.len()-1 && match cmds[cmd].find(">") { Some(_) => true, None => false,} { return; }
        if cmd != cmds.len()-1 && match cmds[cmd].find("&") { Some(_) => true, None => false,}
        { println!("Parsing error: & can appear only after the last command"); return; }
        if cmd == cmds.len()-1 && match cmds[cmd].find("&") { Some(_) => true, None => false,}
        { is_background = true; }
    }
    /*
    if is_background {
        let l = cmds.len()-1;
        let last_cmd : Vec<_> = cmds[cmds.len()-1].split("&").collect();
        cmds[l] = last_cmd[0];
    }
    */

    for cmd in 0..cmds.len() {
        let command :Vec<_> = cmds[cmd].split_whitespace().collect();
        let mut ext_cmd;
        let op = command[0].trim();
        match op {
            "cd" => {
                unsafe{libc::chdir(CString::new(command[1]).unwrap().as_ptr())};
                continue;
            },
            "pwd" => {
                println!("{}",env::current_dir().unwrap().to_str().unwrap().to_string());
                // covert pathbuf to string  https://stackoverflow.com/questions/37388107/how-to-convert-the-pathbuf-to-string
                continue;
            },
            "history" => {
                let mut lines = String::new();
                for i in 0..history.len()-1 { //prints a counter that starts from 1, occupies 5 spaces, and is right-aligned;
                    lines += &format!("{:5}  {}\n",i+1,history[i]);
                }
                println!("{}",lines);
                continue;
            },
            "exit" => {
                exit(0); // assume running on a linux machine
            },
            "kill" => {
                unsafe{libc::kill(i32::from_str_radix(command[1],10).unwrap(), SIGTERM)};
            },
            _ => {
                ext_cmd = Command::new(op);
                if command.len() > 1 {
                    if is_background {
                        for i in 1..command.len()-1 {
                            ext_cmd.arg(command[i]);
                        }
                    }else {
                        for i in 1..command.len() {
                            ext_cmd.arg(command[i]);
                        }
                    }
                }
                let status = ext_cmd.output().unwrap_or_else(|e| {panic!("failed to execute process: {}", e)});
                if status.status.success() {
                    let s = String::from_utf8_lossy(&status.stdout);
                    print!("{}", s);
                }
            },
        } // end of match

    }

}

fn main() {
    let mut command_history : Vec<String> = Vec::new();
    loop {
        print!("$ ");
	    stdout().flush().unwrap(); // print out stdout before input
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(n) => { // n is number of bytes read
                if n == 0 { return; }
                let input = input.trim().to_owned();
                command_history.push(input.clone());
                run(&input , &command_history);
            }
            Err(_) => {  println!("wrong input"); return; },
        }
    }

}
