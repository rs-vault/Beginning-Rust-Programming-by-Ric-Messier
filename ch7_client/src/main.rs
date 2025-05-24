use std::io::prelude::*;
use std::net::TcpStream;
use std::env;
use std::io;
use regex::Regex;


fn validate_input(input: &String) -> bool {
    let mut valid: bool = false;
    let mut params = input.split_whitespace();
    let command = params.next().unwrap();
    match command {
        "flist" =>  valid = true,
        "md" => valid = true,
        _ => valid = false
    }
    valid
}

fn handle_input(mut serverstream: TcpStream) {
    let mut recvstring = [0; 4096];

    let mut keepgoing: bool = true;
    let re = Regex::new(r"^[eE][xX][iI][tT]$").unwrap();

    let mut size = serverstream.read(&mut recvstring);
    println!("{}", String::from_utf8_lossy(&recvstring));

    while keepgoing {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                input = input.trim().to_string();
                if re.is_match(input.as_str()) {
                    keepgoing = false;
                }
                else {
                    if validate_input(&input) {
                        match serverstream.write(&input.as_bytes()) {
                            Ok(_n) => {
                                size = serverstream.read(&mut recvstring);
                                println!("{}", String::from_utf8_lossy(&recvstring));
                            },
                            Err(_e) => {
                                panic!("Unable to write to server");
                            }
                        }
                    }
                    else {
                        println!("Not a valid command");
                    }
                }
            },
            Err(error) => println!("error: {}", error),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let serverstring = &args[1];

    match TcpStream::connect(serverstring) {
        Ok(mut serverstream) => {
            println!("Successfully connected to {}", serverstring);
            handle_input(serverstream);
        }, 
        Err(e) => {
            panic!("Unable to connect to {}", serverstring);
        }
    }



}
