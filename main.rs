use std::env;
use std::fs;
use std::process;
use std::error::Error;
use std::path::Path;

// This is the function that will run on start-up.
fn main() {
    // Collect the arguments given to the app on the command line.
    let args: Vec<String> = env::args().collect();

    // Parse the arguments into a file to encrypt/decrypt, an output file, and whether to be verbose.
    // If there is an error, print it to stderr and exit the process with an error code.
    let (in_file_path, out_file_path, verbose) = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(1);
    });

    // Print some output for the user displaying which files are being used.
    if verbose {
        println!("File to encrypt/decrypt: {in_file_path}");
        println!("File to save to: {out_file_path}");
    } else {
        print!("Encrypting/decrypting {in_file_path} to {out_file_path}...")
    }

    // Encrypt/decrypt the file and write it to the output file.
    // If there is an error, print it to stderr and exit the process with an error code.
    if let Err(e) = run(in_file_path, out_file_path, verbose) {
        eprintln!("Error: {e}");
        process::exit(1);
    }

    // Print some output for the user to know the program has completed.
    if verbose {
        println!("Program completed.")
    } else {
        println!("success.")
    }
}

// This function parses the arguments given on the command line into a file to read,
// a file to write to, and the verbosity flag.
fn parse_args(args: &[String]) -> Result<(&str, &str, bool), &'static str> {
    // If no arguments are provided (the name of the script is always the first argument),
    // display a help message to tell the user how to run the script.
    if args.len() < 2 {
        return Err("usage: infile.txt outfile.txt -verbose");
    }
    // If one argument was provided, raise an error.
    if args.len() < 3 {
        return Err("Not enough arguments.");
    }
    // Get the file to read and file to write to as the second and third arguments.
    let in_file_path = &args[1];
    let out_file_path = &args[2];
    // Verbosity is an optional flag, so we need to check if it is there.
    let verbose;
    if args.len() > 3 {
        if &args[3][..2] == "-v" || &args[3][..2] == "--v" {
            // If the flag starts with "-v" or "--v", assume the rest of the argument spells out "verbose".
            // This also allows single character flags.
            verbose = true;
        } else {
            // Unknown fourth argument, verbosity defaults to false.
            verbose = false;
        }
    } else {
        // No verbosity, so it must be false.
        verbose = false;
    }
    // Return the arguments to the main function.
    Ok((in_file_path, out_file_path, verbose))
}

fn run(in_file_path: &str, out_file_path: &str, verbose: bool) -> Result<(), Box<dyn Error>> {
    // Get some pretty file names for verbose output.
    let in_file_name = get_file_name(&in_file_path);
    let out_file_name = get_file_name(&out_file_path);

    // Read the contents from the file needing to be encrypted/decrypted.
    // Return an error upon failure.
    if verbose {print!("Reading {in_file_name}... working")}
    let contents = fs::read_to_string(in_file_path)?;
    // The "\x08" character is the ASCII backspace character.
    // It moves the cursor back one space (but does not delete).
    // Here we use a format string to fill an arbitrary length space with backspace characters.
    // This replaces the "working" string with a "complete!" string in stdout.
    if verbose {println!("{:\x08<1$}complete!", "", 7)}

    // If verbosity is enabled, print the data of the file to encrypt/decrypt.
    if verbose {
        println!("Size of {in_file_name}: {} bytes", contents.len());
        println!("Contents of {in_file_name}:\n{contents}");
    }

    // Encrypt/decrypt the contents of the file via ROT13.
    if verbose {print!("Encrypting/decrypting text... working")}
    let new_contents = rot13(&contents);
    if verbose {println!("{:\x08<1$}complete!", "", 7)}

    // Write the encrypted/decrypted contents to the output file.
    // Return an error upon failure.
    if verbose {print!("Writing to {out_file_name}... working")}
    fs::write(&out_file_path, &new_contents)?;
    if verbose {println!("{:\x08<1$}complete!", "", 7)}

    // If verbosity is enabled, print the data of the output file.
    if verbose {
        println!("Size of {out_file_name}: {} bytes", new_contents.len());
        println!("Contents of {out_file_name}:\n{new_contents}");
    }
    
    // Return from the function with a signalling value that everything went okay.
    Ok(())
}

// This function gets the "stem" of a path, which is the filename minus the last extension.
fn get_file_name(filename: &str) -> &str {
    match Path::new(filename).file_stem() {
        // If this isn't a valid filename, return the name unchanged.
        None => filename,
        Some(os_str) => match os_str.to_str() {
            // If the filename couldn't be decoded via valid Unicode, return the name unchanged.
            None => filename,
            // Otherwise, return the stem of the filename.
            Some(name) => name
        }
    }
}

// This function encrypts/decrypts a string via ROT13, ignoring non-alphabetical characters.
fn rot13(text: &str) -> String {
    // This one expression function breaks the string into an iterator via `chars`, then applies the
    // ensuing match statement per character, then `collect`s them into a string at the end.
    // The match function rotates uppercase and lowercase letters and ignores non-alphabetical symbols.
    // The rotation translates the character into an unsigned 8-bit integer (ASCII representation) and then
    // either adds or subtracts depending on the letter and turns the integer back into a character.
    text.chars().map(|c| {
        match c {
            'A'..='M' | 'a'..='m' => ((c as u8) + 13) as char,
            'N'..='Z' | 'n'..='z' => ((c as u8) - 13) as char,
            _ => c
        }
    }).collect()
}
