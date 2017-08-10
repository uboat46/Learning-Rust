use std::io;

fn main() {
    println!("FAHRENHEIT TO CELCIUS");
    println!("=====================");

    let c = fahrenheit_to_celcius(-40);

    let f = celcius_to_fahrenheit(-40);

    println!("The only thing i won´t forget from highschool is...");
    println!("that {} Cº are the same as {} ºF",c,f);

    println!("    ");
    println!("    ");
    println!("Nth Fibonacci Number");
    println!("=====================");


    loop {
        println!("Give a number of the fibonacci seq");
        
        let mut number = String::new();
        
        io::stdin().read_line(&mut number)
            .expect("Failed to read line");
        
        let number: i64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("The {} fibonacci number is:", number);
        
        let number = fib(number);

        println!("{}", number);
        break;
    }

    println!("    ");
    println!("    ");
    println!("12 DAYS OF CHRISTMAS");
    println!("=====================");

    let days = ["first","second","third","fourht","fifth","sixth","seventh","eight","ninth","tenth",
                "eleventh","twelfth"];

    let gifts = ["partridge in a pear tree","turtle doves","french hens","calling birds",
                "golden rings","geese a-lying","swans a-swimming","maids a-milking",
                "ladies dancing","lords a-leaping","pipers piping","drummers drumming"];
    let mut days_number = 0;
    while days_number < 12 {
        println!("On the {} day of Christmas,", days[days_number]);
        println!("My true love sent to me");
        let mut gift = days_number + 1;
        let mut gift_index = days_number;
        while gift > 0 {
            println!("{} {}",gift, gifts[gift_index]);
            gift = gift - 1;
            if gift_index != 0 {
                gift_index = gift_index - 1; 
            }
        }
        println!("    ");
        days_number = days_number + 1;
    }
}

fn fahrenheit_to_celcius(f: i64) -> i64 {
    let f = f - 32;
    let f = f * 5;
    f / 9
}

fn celcius_to_fahrenheit(c: i64) -> i64 {
    let c = c * 9;
    let c = c / 5;
    c + 32
}

fn fib(n: i64) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let res = fib(n - 1) + fib(n - 2);
        res
    }
}