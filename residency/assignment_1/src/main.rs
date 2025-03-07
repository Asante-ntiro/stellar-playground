fn main() {

    match divide(4, 2) {
        Ok(result) => println!("Divide result: {}", result),
        Err(e) => println!("Divide error: {}", e),
    }

    match custom_divide(4, 2) {
        CustomResponse::GoodInput(result) => println!("Custom divide successful, the value is: {}", result),
        CustomResponse::NotDivisibleByZero => println!("Custom divide error: Not divisible by zero"),
    }

    match custom_divide(4, 0) {
        CustomResponse::GoodInput(result) => println!("Custom divide successful, the value is: {}", result),
        CustomResponse::NotDivisibleByZero => println!("Custom divide error: Not divisible by zero"),
    }

}

enum CustomResponse {
    NotDivisibleByZero,
    GoodInput(i32),
}

fn custom_divide(a: i32, b: i32) -> CustomResponse {
    if b == 0 {
        return CustomResponse::NotDivisibleByZero;
    }
    CustomResponse::GoodInput(a/b)
}

fn divide(a: i32, b: i32) -> Result<i32,&'static str> {
    match b {
        0 => Err("Cannot divide by zero"),
        _ => Ok(a/b),
        
    }
}




