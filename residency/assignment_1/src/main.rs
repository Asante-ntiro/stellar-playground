fn main() {

    match divide(4, 2) {
        Ok(result) => println!("Divide 1: Successful: the result is: {}", result),
        Err(e) => println!("Divide 1: Error: {}", e),
    }

    match divide(4, 0) {
        Ok(result) => println!("Divide 1: Successful the result is: {}", result),
        Err(e) => println!("Divide 1: Error: {}", e),
    }

    match custom_divide(4, 2) {
        CustomResponse::GoodInput(result) => println!("Custom divide: Successful, the value is: {}", result),
        CustomResponse::NotDivisibleByZero => println!("Custom divide: Error: Not divisible by zero"),
    }

    match custom_divide(4, 0) {
        CustomResponse::GoodInput(result) => println!("Custom divide: Successful, the value is: {}", result),
        CustomResponse::NotDivisibleByZero => println!("Custom divide: Error: Not divisible by zero"),
    }

    match custom_divide_2(4,2) {
        None => println!("Custom divide 2: Error: Not divisible by zero"),
        Some(quotient) => {println!("Custom divide 2: Successful, the value is: {}", quotient)},

    }

    match custom_divide_2(4,0) {
        None => println!("Custom divide 2: Error: Not divisible by zero"),
        Some(quotient) => {println!("Custom divide 2: Successful, the value is: {}", quotient)},

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

fn custom_divide_2(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    }
    else {
        Some(a/b)
    }
}



