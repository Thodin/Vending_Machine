use std::io;

// Could use &str as Err type here for better performance.
// Calling a variable "input_option" is a bit misleading if it is actually a String (and not an Option).
// Why do we have input_option as a parameter here in the first place? We overwrite it immediately. => Could use a local variable.
pub fn validateinput(input_option: &mut String) -> Result<u32, String> {
    // io::stdin will be hard to mock for unit testing. Can we move the user input behind some abstraction?
    io::stdin()
        .read_line(input_option)
        .map_err(|_| "Unable to read the input".to_string())?;

    *input_option = input_option.trim().to_string();
    // Validate the input
    let choice = input_option
        .parse::<u32>()
        .map_err(|_| "Input is not a numeric number".to_string())?;
    Ok(choice)
}
