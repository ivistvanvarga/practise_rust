use std::io;

pub fn getuser_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read user input.");
    return user_input;
    
}