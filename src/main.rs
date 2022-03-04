use rand::prelude::*;
use std::fs;
use std::io::Read;

/*
fn inputp(text: String) -> String {
    let mut input: String = String::new();
    println!("{}", text);
    std::io::stdin().read_line(&mut input).expect("Failed to read");
    let mut tmp: String = String::new();
    let mut tmp_chars = input.chars();
    for x in 0..input.len() {
        if x == input.len() - 1 {
            break;
        }
        tmp.push(tmp_chars.nth(0).unwrap());
    }
    return tmp;
}
*/

fn input() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let mut tmp: String = String::new();
    let mut tmp_chars = input.chars();
    for x in 0..input.len() {
        if x == input.len() - 1 {
            break;
        }
        tmp.push(tmp_chars.nth(0).unwrap());
    }
    return tmp;
}

fn read_file() -> String {
    let mut f = fs::File::open("words.txt").expect("Unable to open");
    let mut s: String = String::new();
    f.read_to_string(&mut s).expect("Could not read file");
    return s;
}

fn vector_each_line(words_to_be_split: String) -> Vec<String> {
    let mut chars = words_to_be_split.chars();
    let mut words: Vec<String> = Vec::new();
    let mut tmp: String = String::new();
    for _x in 0..chars.as_str().len() {
        let each_char: char = chars.nth(0).unwrap();
        if each_char == '\n' {
            words.push(tmp);
            tmp = String::new();
        } else if each_char == '\r' {
            continue;
        } else {
            tmp.push(each_char);
        }
    }
    //println!("{:?}", vec);
    return words;
}

fn get_a_random_item(vector: Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let mut tmp_vec: Vec<String> = vector;
    tmp_vec.shuffle(&mut rng);
    return tmp_vec[0].to_string();
}

fn game_loop() {
    println!("Would you like to play? (y or n)? ");
    let option: String = input().to_lowercase();

    if option == "y" {
        game();
    } else if option == "n" {
        println!("Goodbye!");
    } else {
        println!("Try again with a competent answer this time :)");
        game_loop();
    }
}

fn get_colors(word: &String, guess: &String) -> String {
    if *guess == "EMPTYGUESS" {
        return String::from("⬛⬛⬛⬛⬛");
    } else if *guess == *word {
        return String::from("🟩🟩🟩🟩🟩");
    } else {
        let mut index_word = word.chars();
        let mut index_guess = guess.chars();
        let mut tmp = String::new();
        for _i in 0..5 {
            let word_with_new_index = index_word.nth(0);
            let guess_with_new_index = index_guess.nth(0);
            if guess_with_new_index == word_with_new_index {
                tmp.push('🟩');
            } else {
                tmp.push('_');
            }
        }
    }

    return String::from("smile");
}

fn game() {
    let words: Vec<String> = vector_each_line(read_file());
    let word: String = get_a_random_item(words);
    println!("{}", word);

    let responce = get_colors(&word, &"EMPTYGUESS".to_string());
    let mut guesses = 6;
    while guesses > 0 {
        println!("{:?} guess left", guesses);
        println!("{}", responce);
        let guess: String = input();
        println!("{}", guess);
        if guess == word {
            guesses = -1;
            println!("{}", get_colors(&word, &guess))
        } else {
            if guess.len() == 5 {
                guesses -= 1;
                println!("{}", get_colors(&word, &guess));
            } else {
                println!("Enter in a valid 5 letter word");
            }
        }
    }
    game_loop();
}

fn main() {
    println!("Welcome to wordle 😀 built in rust");
    game();
}
