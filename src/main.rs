use std::io;
use std::process;
use std::ascii;
use sha2::{Sha512, Digest};
use std::fs;
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

    //test file hashing
    let path = Path::new("C:/Users/Morrighan/OneDrive/Desktop/FIM/files/file1.txt");
    match sha512_hash(path) {
        Ok(hash) => println!("SHA512 Hash: {hash}"),
        Err(e) => eprintln!("Error: {e}"),
    } 
    


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

        println!("Collect baseline. Input folder path containing the files to test:");
        //Have user input the path of folder with the files to test
        //Testing path : C:\Users\Morrighan\Desktop\FIM Files\Files

        let mut file_location = String::new();

        io::stdin()
            .read_line(&mut file_location)
            .expect("Failed to readline");
        
        let file_location = file_location.trim();

        
        let path = Path::new(file_location);
        
        
        //Check if path provided is a directory
        if path.is_dir() {

            match fs::read_dir(path){

                Ok(entries) => {

                    for entry in entries {

                        match entry {

                            Ok(entry) => {

                                //Prints full paths of files within the folder
                                //println!("{}",entry.path().display());

                                let filepath = entry.path();

                                println!("File path: {} ", filepath.display());
                                
                                match sha512_hash(&filepath) {
                                    Ok(hash) => println!("SHA512 Hash: {hash}"),
                                    Err(e) => eprintln!("Error: {e}"),
                                } 
                                
                            }
                            Err(e) => {
                                eprintln!("Error reading entry: {}", e);
                            }

                        }

                    }

                }
                Err(e) => {
                    eprintln!("Error reading entry: {}", e);
                }
                
            }

        } else  {
            eprint!("error");
        }

    }

    
    else if input == "B" {
        println!("Monitor");
    }
   else {
        println!("Process terminated. Must input A or B.");
        process::exit(0);
    }

    
}
