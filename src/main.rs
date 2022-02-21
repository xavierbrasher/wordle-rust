use std::fs;
use std::io::Read;
use rand::prelude::*;


fn read_file() -> String {
    let mut f = fs::File::open("words.txt").expect("Unable to open");
    let mut s: String = String::new();
    f.read_to_string(&mut s).expect("Could not read file");
    return s;
}

fn vector_each_line(words_to_be_split:String) -> Vec<String> {
    let mut chars = words_to_be_split.chars();
    let mut words: Vec<String> = Vec::new();
    let mut tmp: String = String::new();
    for _x in 0..chars.as_str().len() {
        let each_char: char = chars.nth(0).unwrap();
        if each_char == '\n' {
            words.push(tmp);
            tmp = String::new();
        }
        else if each_char == '\r' {
            continue;
        }
        else {
            tmp.push(each_char);
        }
        
    }
    //println!("{:?}", vec);
    return words;
}

fn get_a_random_item(vector:Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let mut tmp_vec: Vec<String> = vector;
    tmp_vec.shuffle(&mut rng);
    return tmp_vec[0].to_string();
}

fn input() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input;
}

fn game_loop() {
    println!("Would you like to play? (y or n): ");
    let option: String = input().to_lowercase();
    println!("{}", option);
    if option == String::from("y") {
        game();
    } else if option == String::from("n") {
        println!("Goodbye!");
    }
    
}


fn game() {
    let words: Vec<String> = vector_each_line(read_file());
    let word: String = get_a_random_item(words);
    println!("{}", word);
}


fn main() {
 
    println!("Welcome to wordle 😀 built in rust");
    game_loop();
    
    

}

