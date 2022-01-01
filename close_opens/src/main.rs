use std::io;

fn parser(count: usize, line: &String) -> &String {
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
            //} else if &line[i - 1..i] in closes {
                // we should check we i-2 ...
            } else if &line[i - 1..i] == "{" || &line[i - 1..i] ==  "(" {
                println!("error!, can't parse that string!");
                break;
            }
        }
    }
    line
}

fn main() {
    let mut line = String::new();
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
    line = parser(count, &line).to_string();
    println!("edited string: {}", line);
}
