use std::io;

pub fn conversion_text_to_number(conversion_text: &str) -> u32 {
    conversion_text.trim().parse().expect("Please type a number!")
}

pub fn get_user_answer_in_number() -> u32 {
    let mut user_answer = String::new();

    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");

    conversion_text_to_number(&user_answer)
}

pub fn get_user_answer_in_string() -> String {
    let mut user_answer = String::new();
    
    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");
    user_answer
}