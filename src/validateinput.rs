use std::io;
// This module is responsible for accepting user input
// and validating that it is a valid integer.

pub fn validateinput() -> Result<u32, String> {
    let mut input_choice = String::new();
    io::stdin()
        .read_line(&mut input_choice)
        .map_err(|_| "Unable to read the input".to_string())?;

    input_choice = input_choice.trim().to_string();
    // make sure it is an integer
    let choice = input_choice
        .parse::<u32>()
        .map_err(|_| "Input is not a numeric number".to_string())?;
    Ok(choice)
}
