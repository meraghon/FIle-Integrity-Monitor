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

        // File location for testing: C:/Users/mdayt/Documents/FIM Project/testfiles
        let path = Path::new(file_location);

        println!("Store Baseline. Input absolute folder path to store the baseline file");
        // Input folder where the baseline.txt file is to be stored     
        let mut folder_path = String::new();
        io::stdin()
            .read_line(&mut folder_path)
            .expect("Failed to read line");
        let folder_path = folder_path.trim();
        // Folder path for testing : C:/Users/mdayt/Documents/FIM Project/Baseline Output

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

                                // Skip if the entry is a directory or executable
                                if filepath.is_dir() || filepath.extension() == Some("exe".as_ref()) {
                                continue;
                            }
                                
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
        // Confirmation of hashes written to file
        println!("Hashes successfully written to baseline.txt");

    }

    
    else if input == "B" {
        println!("Monitor");
        // Check if baseline.txt exists, if it exists continue if not then end program?
        // Go through and re-hash all files within the folder
        // Compare them to what is in the baseline.txt
    }
   else {
        println!("Process terminated. Must input A or B.");
        process::exit(0);
    }

    
}
