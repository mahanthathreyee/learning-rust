use std::collections::HashMap;

//INPUTS
//INPUT CIPHER STRING BELOW
const X: &str = "Pm ol ohk hufaopun jvumpkluaphs av zhf ol dyval pa pu jpwoly aoha pz if zv johunpun aol vykly vm aol slaalyz vm aol hswohila aoha uva h dvyk jvbsk il thkl vba";
const FREQ_DEPTH: usize = 5;    //NUMBER OF FREQUENT CHARACTERS TO COMPARE WITH

const UPPER: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
const LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const FREQ_CHARS: [u32; 26] = [4, 19, 0, 14, 8, 13, 18, 17, 7, 11, 3, 2, 20, 12, 5, 15, 6, 22, 24, 1, 21, 10, 23, 9, 16, 25];
const FREQ_TWO_CHAR: [&str; 24] = ["of", " to", " in", " it", " is", " be", " as", " at", " so", " we", " he", " by", " or", " on", " do", " if", " me", " my", " up", " an", " go", " no", " us", " am"];
const FREQ_THREE_CHAR: [&str; 39] = ["the", " and", " for", " are", " but", " not", " you", " all", " any", " can", " had", " her", " was", " one", " our", " out", " day", " get", " has", " him", " his", " how", " man", " new", " now", " old", " see", " two", " way", " who", " boy", " did", " its", " let", " put", " say", " she", " too", " use"];
const FREQ_FOUR_CHAR: [&str; 15] = ["that", " with", " have", " this", " will", " your", " from", " they", " know", " want", " been", " good", " much", " some", " time"];
const ENABLE_DEBUG: bool = false;

fn rotate_char(base: [char; 26], c: char, sft: u32) -> char {
    match base.iter().position(|b| c == *b) {
        Some(x) => {
            let x: u32 = x as u32;
            let idx: u32 = if (sft + x) >= 26u32 {
                sft + x - 26u32
            } else {
                sft + x
            };

            match base.iter().nth(idx as usize) {
                Some(x) => *x,
                _ => panic! {"An error occured at 2"},
            }
        }
        _ => panic! {"An error occured at 1"},
    }
}

fn rotate_string(sft: u32) -> String {
    let mut cipher: String = String::new();
    for c in X.chars() {
        if c.is_whitespace() {
            cipher.push(c);
        } else if c.is_uppercase() {
            cipher.push(rotate_char(UPPER, c, sft));
        } else {
            cipher.push(rotate_char(LOWER, c, sft));
        }
    }

    cipher
}

fn get_frequency_char() -> char {
    let mut char_map: HashMap<char, u32> = HashMap::new();
    for c in X.to_lowercase().chars() {
        if !c.is_whitespace() {
            *char_map.entry(c).or_insert(0) += 1;
        }
    }

    let mut freq_char: char = 'a';
    let mut freq_count: u32 = 0;

    for (c, v) in char_map.iter() {
        if *v > freq_count {
            freq_count = *v;
            freq_char = *c;
        }
    }
    freq_char
}

fn check_word_exists(dictionary: Vec<&str>, word: &str) ->bool {
    match dictionary.iter().position(|b| word.eq(*b)) {
        Some(_x) => true,
        _ => false
    }
}

fn analyze_freq_words(rotated_string: String) {
    let words: Vec<&str> = rotated_string.split(" ").collect();
    let mut possible_plain_text = false;
    for word in words {
        let word_length = word.chars().count();
        if match word_length {
            2 => check_word_exists(FREQ_TWO_CHAR.to_vec(), word),
            3 => check_word_exists(FREQ_THREE_CHAR.to_vec(), word),
            4 => check_word_exists(FREQ_FOUR_CHAR.to_vec(), word),
            _ => false
        } {
            possible_plain_text = true;
        }
    }

    if possible_plain_text {
        print!("POSSIBLE PLAIN TEXT::  ");
    }
    if ENABLE_DEBUG || possible_plain_text {
        println!("{}", rotated_string);
    }
}

fn analyze_freq_chars() {
    let c: char = get_frequency_char();
    let x: i32 = match LOWER.iter().position(|b| c == *b) {
        Some(x) => x as i32,
        _ => panic! {"An error occured at 3"},
    };
    println!("Most frequent character: {}", UPPER[x as usize]);
    for y in &FREQ_CHARS[0..FREQ_DEPTH] {
        let z: i32 = *y as i32 - x;
        let z: u32 = if z < 0 {
            ((z * -1) as u32) % 26
        } else {
            (z % 26) as u32
        };
        if ENABLE_DEBUG {
            println!("SELECTED OFFSET CHAR :: {}, ROT:: {}", UPPER[*y as usize], z);
        }
        analyze_freq_words(rotate_string(z));
    }
}

fn main() {
    analyze_freq_chars();
    println!("If the plain text wasn't found, increase the FREQ_DEPTH and ENABLE_DEBUG to see more rotated strings");
}
