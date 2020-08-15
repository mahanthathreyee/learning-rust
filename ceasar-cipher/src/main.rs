const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";

fn rotate_char(base: &str, c: char, sft: u32) -> char {
    match base.chars().position(|b| c == b) {
        Some(x) => {
            let x: u32 = x as u32;
            let idx: u32 = if (sft + x) >= 26u32 {
                sft + x - 26u32
            } else {
                sft + x
            };

            match base.chars().nth(idx as usize) {
                Some(x) => x,
                None => panic! {"An error occured at 2"}
            }
        },
        None => panic! {"An error occured at 1"}
    }
}

fn rotate_string(sft: u32) -> String {
    let x = "JmCRvPX TQuZsVy"; //INPUT STRING

    let mut cipher: String = String::new();
    for c in x.chars() {
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

fn main() {
    for i in 0..27 {
        println!("{}", rotate_string(i));
    }
}
