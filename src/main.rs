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

fn main() {
    if_statement();
}
