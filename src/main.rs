fn main() {
    let mut choice = String::new();
    loop {
        calculate();
        println!("Do you want to continue? y/n ");
        std::io::stdin()
            .read_line(&mut choice)
            .expect("failed to read num1");

        if choice.trim() == "n" {
            break;
        }
    }
}

fn calculate() {
    // String module
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    println!("Please enter first number :");
    //   std -> crate
    //    io -> module
    // stdin -> method returning std variable
    std::io::stdin()
        .read_line(&mut num1)
        .expect("failed to read num1");

    println!("Please enter second number :");
    std::io::stdin()
        .read_line(&mut num2)
        .expect("failed to read num2");

    println!("Please enter operator (+,-,/,*):");
    std::io::stdin()
        .read_line(&mut operator)
        .expect("failed to read operator");

    let num1: f64 = num1.trim().parse().expect(" error during parsing num1 ");
    let num2: f64 = num2.trim().parse().expect(" error during parsing num2 ");

    if operator.trim() == "+" {
        println!("{}", num1 + num2);
    } else if operator.trim() == "-" {
        println!("{}", num1 - num2);
    } else if operator.trim() == "*" {
        println!("{}", num1 * num2);
    } else if operator.trim() == "/" {
        println!("{}", num1 / num2);
    } else {
        println!("invalid operator");
    }
}
