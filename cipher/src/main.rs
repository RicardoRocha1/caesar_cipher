use std::collections::HashMap;
use std::io;
use std::fs;

fn main() {
    println!("Caesar te salutat.");
    println!("Do you wish to cipher a string or .txt file? [Input: STRING or TEXT]");
    let mut user_input_choice: String = String::new();
    io::stdin().read_line(&mut user_input_choice).expect("Could not detect type choice.");

    let letters: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let alphabet: HashMap<u8, char> = alphabet_creation(letters);

    let user_input: String = define_input(user_input_choice);

    let mut ciphered_string: String = cipher(alphabet, user_input);

    println!("\nCipher completed: \n{}", ciphered_string);
}

fn alphabet_creation(letters: String) -> HashMap<u8, char> {
    let mut alphabet: HashMap<u8, char> = HashMap::new();
    for (index, letter) in letters.chars().enumerate() {
        let index_u8: u8 = index as u8;
        alphabet.insert(index_u8, letter);
    }
    alphabet
}

fn define_input(user_input_choice:String) -> String {
    if user_input_choice.trim().to_uppercase() == "STRING" {
        println!("Please insert the string to be ciphered");
        let mut string_input: String = String::new();
        io::stdin().read_line(&mut string_input).expect("Could not read string.");
        return string_input;
    }

    else if user_input_choice.trim().to_uppercase() == "TEXT" {
        println!("Please enter the name of the text file (e.g.: my_file.txt)");
        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name).expect("Could not find or read file.");
        
        let file_name = file_name.trim();

        match fs::read_to_string(&file_name) {
            Ok(contents) => {
                println!("File contents: \n{}", contents);
                contents
            }
            Err(err) => {
                eprintln!("Error reading file: \n{}", err);
                String::new()
            }
        }
    }

    else {
        println!("Invalid type entered, please enter 'STRING' for string or 'TEXT' for .txt");  
        String::new()
    }

}

fn cipher(alphabet: HashMap<u8,char>, user_input: String) -> String {
    let mut ciphered_string: String = String::new();
    let shift: u8 = 3;

    for c in user_input.chars() {
        if c.is_ascii_alphabetic() {
            let c_lower = c.to_lowercase().next().unwrap();
            let index = match alphabet.iter().find(|&(_, &v)| v == c_lower) {
                Some((&k, _)) => k,
                None => continue,
            };

            let shifted_index = (index + shift) % 26; // Apply the shift and wrap around if necessary
            let shifted_char = *alphabet.get(&shifted_index).unwrap(); // Get the shifted character from the alphabet
            ciphered_string.push(shifted_char);
        } else {
            ciphered_string.push(c); // Non-alphabetic characters remain unchanged
        }
    }
    
    ciphered_string
}

// Let user choose shift value -> now is 3
// Let user choose output type -> now is only string
// Let user choose alphabet -> now is only latin