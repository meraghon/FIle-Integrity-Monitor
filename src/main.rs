use std::io;
use std::process;
use std::ascii;
use sha2::{Sha512, Digest};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn sha512_hash(path: &Path) -> Result<String, io::Error> {

    let input = File::open(path)?;
    let mut reader = BufReader::new(input);
    let mut hasher = Sha512::new();

    let mut buffer = [0; 1024]; //read the file in chunks

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn main() {

    /*test file hashing (IT WORKS)
    let path = Path::new("C:/Users/Morrighan/OneDrive/Desktop/FIM/files/file1.txt");
    match sha512_hash(path) {
        Ok(hash) => println!("SHA512 Hash: {hash}"),
        Err(e) => eprintln!("Error: {e}"),
    } */
    


    println!("A) Collect New Baseline?");

    println!("B)  Monitor Files w/ Saved Baseline.");

    let mut input = String::new();

    // User input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim(); //Remove leading/trailing whitespace

    let input = input.to_ascii_uppercase(); 

    if input == "A" {
        println!("Collect baseline. Input folder path");
        //Have user input the path of folder with the files to test
        let mut FileLocation = String::new();

        io:stdin()
            .read_line(&mut FileLocation)
            .expect("Failed to readline");

        println!({FileLocatoin});



    }
    else if input == "B" {
        println!("Monitor");
    }
   else {
        println!("Process terminated. Must input A or B.");
        process::exit(0);
    }

    
}
