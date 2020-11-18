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

fn main() {
    //if_statement();
    // while_and_loop();
    for_loop();
}
