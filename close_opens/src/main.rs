use std::io;
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn parser(count: usize, mut line: &String, mut lr_parsing: bool) -> &String {
    //let opens = ["[", "(", "{"];
    //let closes = ["]", ")", "}"];
    let mut rbracket: i32 = 0; // ()
    let mut sbracket: i32 = 0; // []
    let mut cbracket: i32 = 0; // {}
    for i in 0..count - 1 {
        println!("char in index {} is: {}", i, &line[i..i + 1]);
        if &line[i..i + 1] == "[" {
            sbracket += 1;
            println!("s:{} c:{} r:{}", sbracket, cbracket, rbracket);
            continue;
        } else if &line[i..i + 1] == "{" {
            cbracket += 1;
            println!("s:{} c:{} r:{}", sbracket, cbracket, rbracket);
            continue;
        } else if &line[i..i + 1] == "(" {
            rbracket += 1;
            println!("s:{} c:{} r:{}", sbracket, cbracket, rbracket);
            continue;
        } else if &line[i..i + 1] == "]" {
            if &line[i - 1..i] == "[" {
                sbracket -= 1;
                println!("s:{} c:{} r:{}", sbracket, cbracket, rbracket);
                continue;
            // } else if &line[i - 1..i] in closes {
            // we should check we i-2 ...
            } else if &line[i - 1..i] == "{" || &line[i - 1..i] == "(" {
                println!("error!, can't parse this string!");
                continue;
            } // else if &line[i - 1..i] == ""
        } else if &line[i..i + 1] == "}" {
            if &line[i - 1..i] == "{" {
                sbracket -= 1;
                println!("s:{} c:{} r:{}", sbracket, cbracket, rbracket);
                continue;
            } else if &line[i - 1..i] == "[" || &line[i - 1..i] == "(" {
                println!("error!, can't parse this string!");
                break;
            }
        } else if &line[i..i + 1] == ")" {
            if &line[i - 1..i] == "(" {
                sbracket -= 1;
                println!("s:{} c:{} r:{}", sbracket, cbracket, rbracket);
                continue;
            } else if &line[i - 1..i] == "[" || &line[i - 1..i] == "{" {
                println!("error!, can't parse this string!");
                break;
            }
        }
    }
    if lr_parsing == true {
        lr_parsing = false;
        let enil: String = line
        .graphemes(true)
        .rev()
        .collect();
        line = parser(count, &enil, lr_parsing);
        &line
    } else {
         let enil: String = line
        // Split the string into an Iterator of &strs, where each element is an
        // extended grapheme cluster.
        .graphemes(true)
        // Reverse the order of the grapheme iterator.
        .rev()
        // Collect all the chars into a new owned String.
        .collect();
        &enil
    }
}

fn main() {
    let mut line = String::new();
    let mut lr_parsing = true;
    println!("enter string: ");
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line!");
    let temp = &line[..1];
    //println!("first char you entered is: {}", temp);
    if temp == "}" {
        line = "{".to_owned() + &line;
    } else if temp == "]" {
        line = "[".to_owned() + &line;
    } else if temp == ")" {
        line = "(".to_owned() + &line;
    }
    let count = line.len();
    line = parser(count, &line, true).to_string();
    println!("edited string: {}", line);
}
