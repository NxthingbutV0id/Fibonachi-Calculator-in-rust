use std::io;

fn main() {
    loop {
        println!("What Fibonachi number do you want to get?");
    
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Error");

        let number: i32 = match number.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("The {} Fibonachi number is {}.", number, fibonachi(number));

        break;
    }
}

fn fibonachi(n: i32) -> f64 {
    let phi = (1.0 + 5.0f64.sqrt())/2.0;
    ((phi.powi(n) - (-1.0/phi).powi(n))/5.0f64.sqrt()).round()
}
