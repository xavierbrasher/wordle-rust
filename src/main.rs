use std::fs;
use std::io::Read;

fn read_file() -> String{
    let mut f = fs::File::open("words.txt").expect("Unable to open");
    let mut s = String::new();
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

fn main() {

    println!("Hello rust");
    println!("{:?}", vector_each_line(read_file()));
}
