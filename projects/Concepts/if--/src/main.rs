fn main() {
    if_num();
    
    if_let();

    loops();

    while_loops();

    while_arr();

    for_loops();

    for_rev();
}

fn if_num(){
    let number = 0;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number !=0{
        println!("numbar was something other than zero");
    }

    if number == 0{
        println!("number is 0");
    } else
    if number % 4 == 0{
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    // } else if number == 0{
    //      println!("number is 0");
    } else {
        println!("numver is not devisible by 4, 3, and 2");
    }
}

fn if_let(){
    let con = true;
    let num = if con{
        5
    } else {
        6
    };

    println!("{}", num);
}

fn loops(){
    let mut loop_num = 0;
    loop {
        println!("again!!!");
        loop_num += 1;

        if loop_num == 10{
            println!("{}complete!!!",loop_num);
            break;
        } 
        
        if loop_num < 10{
            println!("{}", loop_num);
            continue;
        }
    }
}

fn while_loops(){
    let mut num = 3;

    while num != 0 {
        println!("{}", num);

        num = num - 1
    }

    println!("liftoff!!");
}

fn while_arr(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("{}", a[index]);

        index = index + 1;
    }
}

fn for_loops(){
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("for loops {}", element);
    }
}

fn for_rev(){
    for num in (1..4).rev() {
        println!("for rev :{}",num);
    }
    println!("for rev :done");
}