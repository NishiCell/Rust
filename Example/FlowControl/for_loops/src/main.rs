fn main() {
    // `n` will take the values: 1, 2, ...,100 in each iterarion
    // for n in 1..101 {
    //     if n % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if n % 3 == 0 {
    //         println!("fizz");
    //     } else if n % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", n);
    //     }
    // }
    iter_();

    into_iter_();

    iter_mut_();
}

fn iter_ () {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn into_iter_() {
    let names = vec! {"Bob","Frank", "Ferris"};

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("THere is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn iter_mut_() {
    let mut names = vec!{"Bob", "Frank", "Ferris"};

    for name in names.iter_mut(){
        match name {
            &mut "Ferris" => println!("There is a rustacean among us !"),
            _ =>println!("Hello {}", name),
        }
    }
}

