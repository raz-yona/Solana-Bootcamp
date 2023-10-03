use std::io;

fn main() {
    //Get the calculation from the user
    let mut calc: String = String::new();
    let stdin: io::Stdin = io::stdin();
    println!("write the calculation you would like to do.");
    match stdin.read_line(&mut calc){
        Ok(_) => {
            println!("The equation is: {}", calc);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }

    //split the string, parse the numbers and push them into the vector parsed_numbers
    let mut parsed_numbers: Vec<f64> = Vec::new();
    for op in calc.chars() {
        if op == '+' || op == '-' || op == '*' || op == '/' {
            println!("The math operation is: {}", op);
            let calc_numbers_as_chars = calc.split(op);
            for unparsed_num in calc_numbers_as_chars {
                match unparsed_num.trim().parse::<f64>() {
                    Ok(parsed_number) => {
                        parsed_numbers.push(parsed_number);
                        println!("{} has been pushed to the vector", parsed_number);
                    },
                    Err(error) => println!("Parsing error, {}", error)
                }
            }   

            match op {
                '+' => {
                    let mut first_unparsed_number: f64 = 0.0;
                let mut second_unparsed_number: f64 = 0.0;
                match parsed_numbers.get(0) {
                    Some(value) => {first_unparsed_number = *value},
                    None => { println!("no number in the vector")}
                }
                match parsed_numbers.get(1) {
                    Some(value) => {second_unparsed_number = *value},
                    None => { println!("no number in the vector")}
                }
                let addition_calc: Calculator = Calculator::Add{x: first_unparsed_number, y: second_unparsed_number};
                addition_calc.calculate();
                },
                '-' => {
                    let mut first_unparsed_number: f64 = 0.0;
                let mut second_unparsed_number: f64 = 0.0;
                match parsed_numbers.get(0) {
                    Some(value) => {first_unparsed_number = *value},
                    None => { println!("no number in the vector")}
                }
                match parsed_numbers.get(1) {
                    Some(value) => {second_unparsed_number = *value},
                    None => { println!("no number in the vector")}
                }
                let addition_calc: Calculator = Calculator::Subtract{x: first_unparsed_number, y: second_unparsed_number};
                addition_calc.calculate();
                },
                '*' => {
                    let mut first_unparsed_number: f64 = 0.0;
                let mut second_unparsed_number: f64 = 0.0;
                match parsed_numbers.get(0) {
                    Some(value) => {first_unparsed_number = *value},
                    None => { println!("no number in the vector")}
                }
                match parsed_numbers.get(1) {
                    Some(value) => {second_unparsed_number = *value},
                    None => { println!("no number in the vector")}
                }
                let addition_calc: Calculator = Calculator::Multiply {x: first_unparsed_number, y: second_unparsed_number};
                addition_calc.calculate();
                },
                '/' => {
                    let mut first_unparsed_number: f64 = 0.0;
                let mut second_unparsed_number: f64 = 0.0;
                match parsed_numbers.get(0) {
                    Some(value) => {first_unparsed_number = *value},
                    None => { println!("no number in the vector")}
                }
                match parsed_numbers.get(1) {
                    Some(value) => {second_unparsed_number = *value},
                    None => { println!("no number in the vector")}
                }
                let addition_calc: Calculator = Calculator::Divide{x: first_unparsed_number, y: second_unparsed_number};
                addition_calc.calculate();
                },
                _ => { println!("calculation isn't valid")}
            }
        }    
    }
    enum Calculator{
        Add{x: f64, y: f64},
        Subtract{x: f64, y: f64},
        Multiply{x: f64, y: f64},
        Divide{x: f64, y: f64}
    }

    impl Calculator {
        fn calculate(&self) {
            match self {
                Calculator::Add{x,y} => println!("result is {}", x+y),
                Calculator::Subtract{x,y} => println!("result is {}", x-y),
                Calculator::Multiply{x,y} => println!("result is {}", x*y),
                Calculator::Divide{x,y} => println!("result is {}", x/y),
            }
        }
    }
}