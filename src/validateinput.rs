use std::io;
pub fn validateinput(input_option: &mut String) -> Result<u32, String> {
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
