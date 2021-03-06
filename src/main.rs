use colored::Colorize;

mod config;
use config::*;

mod handlers {
    pub mod ui_handler;
    pub mod game_handler;
    pub mod file_handler;
    pub mod menu_handler;
}
use handlers::{
    ui_handler::*,
    game_handler::*,
    file_handler::*,
    menu_handler
};

fn main() {
    menu_handler::display_menu();
   

    let words = get_word_file(WORDS_FILE);
    loop {
        clear();
        let answer = get_word(&words).to_lowercase();
        // println!("{answer}"); // FOR DEBUGGING, REMOVE LATER
        let mut hints: Vec<String> = Vec::new();
        let mut won = false;

        for i in 0..MAX_TRIES {
            // Displays the disclaimer and user prompt
            let disclaimer = get_disclaimer(MAX_TRIES - i, &hints);
            println!("{}", disclaimer);
            let mut user_input = take_input(
                format!("{}", "Guess the word: ".purple())
            );
            clear();
            check_input_loop(&mut user_input, &disclaimer, &hints);

            // Compares word and actual answer, and then displays hints
            let checks = check_word(&answer, &user_input);
            hints.push(get_hint(&checks, &user_input.to_ascii_uppercase()));
            display_hints(&hints);

            if checks == vec!['g'; WORD_LENGTH] {
                won = true;
                break;
            }
        }
        show_end_screen(won, &answer);
        if go_back() {
            menu_handler::display_menu();
        } 
    }
}


// Keeps looping until a valid word is entered
fn check_input_loop(user_input: &mut String, disclaimer: &String, hints: &Vec<String>) {
    while user_input.len() != WORD_LENGTH {
        println!(
            "{} {} {}",
            "Only words with".blue(),
            WORD_LENGTH.to_string().blue(),
            "letters are accepted!".blue()
        );
        display_hints(&hints);
        println!("{}", disclaimer);
        *user_input = take_input(
            format!("{}", "Guess the word: ".purple())
        );
        clear();
    }
}


fn go_back() -> bool {
    let back = take_input(
        format!("{}", 
                      "Enter 'm' or 'menu' to Go Back or Press ENTER to Play Again: "
                        .blue()
                        .bold()
                    )
                );
    if back.to_lowercase() == "m"
       || back.to_lowercase() == "menu" {
        return true;
       } else {
        false
       }
}