use rand::seq::SliceRandom;

static RANDOM_WORD_LIST: [&str; 32] = [
    "business",
    "becoming",
    "sick",
    "poem",
    "whenever",
    "theory",
    "up",
    "born",
    "anyone",
    "religious",
    "gold",
    "cabin",
    "triangle",
    "cry",
    "felt",
    "interior",
    "image",
    "muscle",
    "simple",
    "noise",
    "baby",
    "port",
    "society",
    "around",
    "carefully",
    "lot",
    "element",
    "car",
    "key",
    "mix",
    "wait",
    "goodbye",
];

fn pick_random_word() -> String {
    RANDOM_WORD_LIST
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

fn read_guess() -> Option<char> {
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().chars().nth(0)
}

fn validate_user_guess(user_guess: Option<char>) -> bool {
    match user_guess {
        Some(guess) => {
            if guess.is_alphanumeric() {
                true
            } else {
                false
            }
        }
        None => false,
    }
}

fn main() {
    println!("Welcome to hangman!");
    let random_word = pick_random_word();

    let mut guess_display = (0..random_word.chars().count())
        .map(|_| "_ ")
        .collect::<String>()
        .trim()
        .to_string();

    loop {
        println!("Current word: \n{}", guess_display);
        if !guess_display.contains("_") {
            break;
        }
        println!("Please enter your guess");
        let user_guess = read_guess();

        if validate_user_guess(user_guess) {
            let formatted_guess = user_guess.unwrap().to_lowercase().next().unwrap();
            let guess_contained_in_answer = random_word.contains(formatted_guess);

            if guess_contained_in_answer {
                for (i, ch) in random_word.chars().enumerate() {
                    if ch.eq(&formatted_guess) {
                        guess_display = format!(
                            "{}{}{}",
                            &guess_display[0..i * 2],
                            formatted_guess,
                            &guess_display[i * 2 + 1..guess_display.len()]
                        );
                    }
                }
            }
        } else {
            println!("Invalid guess, please only use letters");
        }
    }
    println!("Congratulations!");
}
