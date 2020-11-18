#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_see)]

use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

fn if_statement() {
    let temp = 28;

    if temp > 30 {
        println!("It's really hot outside!");
    } else if temp < 10 {
        println!("It's really cold outside!");
    } else {
        println!("It's great outside!")
    }

    let day = if temp > 20 {"Sunny"} else {"cloudy"};
    println!("Today is {}", day);

    println!("It is {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"});

    println!("It is {}, Nested",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"ok"});
}

fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 { continue; }

        println!("x = {}", x)
    }

    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 { break; }
    }
}

fn for_loop() {
    for x in 1..11 {
        if x == 3 { continue; }

        if x == 8 { break; }

        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statement() {
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1 => "United States of America",
        2..=1000 => "Unknown",
        _ => "invalid"
    };

    println!("The country with code {} is {}",
        country_code, country);

    let x = false;
    
    let s = match x {
        true => "yes",
        false => "no"
    };

    println!("Is it true? {}", s);
}

fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear(); // ""
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}

fn main() {
    //if_statement();
    // while_and_loop();
    //for_loop();
    //match_statement();
    combination_lock();
}
