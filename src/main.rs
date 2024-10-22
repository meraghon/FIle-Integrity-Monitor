use std::io;
use std::process;
use std::ascii;
use sha2::{Sha512, Digest};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, Read, BufWriter};
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

        // Prompt user to input the absolute path of the folder containing the files
        println!("Collect baseline. Input absolute folder path containing the files to test:");

        // Create variable to store the folder path
        let mut file_location = String::new();

        io::stdin()
            .read_line(&mut file_location)
            .expect("Failed to readline");
        
        let file_location = file_location.trim();

        
        let path = Path::new(file_location);

        // Input folder where the baseline.txt file is to be stored 
        let folder_path = "C:/Users/mdayt/Documents/FIM Project/Baseline Output";
        let baseline_path = format!("{}/baseline.txt", folder_path);

        // Create the baseline.txt file
        let mut baseline = File::create(&baseline_path).expect("Failed to create file");
        // Writes to baseline
        let mut writer = BufWriter::new(baseline);
        
        
        //Check if path provided is a directory
        if path.is_dir() {

            match fs::read_dir(path){

                Ok(entries) => {

                    // Iterates over each file in the directory
                    for entry in entries {

                        match entry {

                            Ok(entry) => {

                                let filepath = entry.path();
                                
                                match sha512_hash(&filepath) {
        
                                    Ok(hash) => {
                                        // Writes file path and hash to the baseline.txt file
                                        let output = format!("File path: {}\nSHA512 Hash: {hash}\n", path.display());
                                        writer.write_all(output.as_bytes()).expect("Failed to write to file.");
                                    }
                            
                                    Err(e) => {
                                        let error_message = format!("Error: {e}");
                                        writer.write_all(error_message.as_bytes()).expect("Failed to write to file.");
                                    }
                                } 

                                // Ensure data is flushed to disk
                                writer.flush().expect("Failed to flush buffer");
                                
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
        // Go through and re-hash all files within the folder
        // Compare them to what is in the baseline.txt
    }
   else {
        println!("Process terminated. Must input A or B.");
        process::exit(0);
    }

    
}
