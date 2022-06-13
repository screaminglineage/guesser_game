use colored::Colorize;
use std::io;
use std::io::Write;

const MAX_TRIES: u32 = 6;
const WORD_LENGTH: usize = 5;

fn main() {
    clear();
    let answer = "roast".to_string();
    let mut hints: Vec<String> = Vec::new();
    let mut won = false;

    for i in 0..MAX_TRIES {
        // Displays the disclaimer and user prompt
        let disclaimer = get_disclaim(MAX_TRIES - i, &hints);
        println!("{}", disclaimer);
        let mut user_input = take_input(format!("{}", "Guess the word: ".purple()));
        clear();
        // print!("\x1B[2J\x1B[1;1H"); // Clears the screen

        // Keeps looping until a valid word is entered
        while user_input.len() != WORD_LENGTH {
            println!(
                "{} {} {}",
                "Only words with".blue(),
                WORD_LENGTH.to_string().blue(),
                "letters are accepted!".blue()
            );
            display_hints(&hints);
            println!("{}", disclaimer);
            user_input = take_input(format!("{}", "Guess the word: ".purple()));
            clear();
            // print!("\x1B[2J\x1B[1;1H"); // Clears the screen
        }

        // Compares word and actual answer, and then displays hints
        let checks = check_word(&answer, &user_input);
        hints.push(get_hint(&checks, &user_input.to_ascii_uppercase()));
        display_hints(&hints);

        if checks == vec!['g'; WORD_LENGTH] {
            won = true;
            break;
        }
    }

    // End Screen
    if won {
        println!("\n{}", "You Won!".purple());
    } else {
        println!("\n{}", "You Lost!".red());
        println!(
            "{} {}",
            "The word was".green(),
            answer.to_ascii_uppercase().green().bold()
        );
    }
    pause(format!(
        "{} {} {}",
        "Press".blue(),
        "ENTER".blue().bold(),
        "to Continue...".blue()
    ));
}

/*  Compares answer and a word and returns a char vector containing
    - g -> letter in answer and correct position
    - y -> letter in answer but wrong position
    - w -> letter not in answer
*/
fn check_word(answer: &String, word: &String) -> Vec<char> {
    let mut ans_letters = get_letters(answer);
    let mut word_letters = get_letters(word);
    let mut check_vec: Vec<char> = Vec::new();
    let length = WORD_LENGTH;
    let mut check: char;

    /* Checks for characters which are in the correct
       position and removes them after marking the position
       as correct in the check_vec vector
    */
    for i in 0..length {
        if ans_letters[i] == word_letters[i] {
            ans_letters[i] = ' ';
            word_letters[i] = ' ';
            check = 'g';
        } else {
            check = ' ';
        }
        check_vec.push(check);
    }

    /* Checks for every character in the input word which
       also present in the answer and removes them both after
       marking the position as wrong position but correct letter
       in the check_vec vector
    */
    for i in 0..length {
        let mut flag = false;

        if check_vec[i] == ' ' {
            for _ in 0..length {
                let letter = word_letters[i];
                if ans_letters.contains(&letter) {
                    let pos = ans_letters
                        .iter()
                        .position(|&x| x == letter)
                        .expect("Failed to find letter in answer");

                    flag = true;
                    check_vec[i] = 'y';
                    word_letters[i] = ' ';
                    ans_letters[pos] = ' ';
                    break;
                }
            }
            if flag == false {
                check_vec[i] = 'w';
            }
        }
    }
    check_vec
}

// Displays a prompt to the user and returns their input
fn take_input(prompt: String) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush buffer");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_ascii_lowercase()
}

// Waits for ENTER to be pressed
fn pause(prompt: String) {
    let mut x = String::with_capacity(1);
    println!("\n{}", prompt);
    io::stdin().read_line(&mut x).expect("Failed to Read");
    println!("\n\n\n\n\n\n\n")
}

// Converts word into an char vector
fn get_letters(word: &String) -> Vec<char> {
    let letters: Vec<char> = word.chars().collect();
    letters
}

/*  Returns a string containing the letters of the word
    in the following colour scheme:
    - green -> letter in answer and correct position
    - yellow -> letter in answer but wrong position
    - white -> letter not in answer
    - red -> anything else (shouldnt normally be possible)
*/
fn get_hint(checks: &Vec<char>, word: &String) -> String {
    let mut hint = String::new();
    if word.len() != 0 {
        let letters: Vec<char> = word.chars().collect();
        for i in 0..letters.len() {
            let letter = letters[i].to_string();

            hint = format!(
                "{}{}",
                hint,
                match checks[i] {
                    'g' => letter.green().bold(),
                    'y' => letter.bright_yellow().bold(),
                    'w' => letter.white(),
                    _ => letter.red().bold(),
                }
            );
        }
    }
    return hint;
}

fn display_hints(hints: &Vec<String>) {
    println!("");
    for hint in hints {
        println!("{}", hint);
    }
}

fn get_disclaim(i: u32, hints: &Vec<String>) -> String {
    let disclaimer = if i != 1 {
        format!("{} {}", (i).to_string().blue(), "Tries Remaining...".blue())
    } else {
        format!("{} {}", (i).to_string().red(), "Try Remaining...".blue())
    };
    if hints.len() > 0 {
        return format!("{}\n{}", "--------------------".purple(), disclaimer);
    } else {
        return disclaimer;
    }
}


fn clear() {
    for _ in 0..1000 {
        println!("\n");
    }
} 