#![allow(unused_mut, unused_variables)]

fn inspect(text: &String) {
    if text.ends_with("s") {
        println!("{} is plural", text);
    } else {
        println!("{} is singular", text);
    }
}

fn change(text: &mut String) {
    if !text.ends_with("s") { text.push_str("s") };
}

fn eat(consumes: String) -> bool {
    if consumes.starts_with("b") && consumes.contains("a") {
        return true;
    } else {
        return false;
    }
}

fn add(x: &i32, y: &i32) -> i32 {
    x + y
}

fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    println!("1 + 2 = {}, even via references", add(&1, &2));
}
