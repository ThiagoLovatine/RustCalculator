use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();


    let result: f32 = operate(operator, first_number, second_number);

    let result_string: String = output(operator, first_number, second_number, result);

    println!("{}", result_string);
}

fn output(operator: char, first_number: f32, second_number: f32, result: f32) -> String {
    return format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    );
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used"),
    }
}
