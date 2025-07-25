//use std::{thread::sleep, time::Duration};

enum TypesProfile {
    SuperAdmin,
    User,
    Admin,
}

fn main() {
    // Part-1: Borrow and Reference
    let name = String::from("MK47");
    println!("Before-Name: {}", name);
    greet(&name);
    println!("After-Name: {}", name);

    // Part-2: Enum and match
    check_user(TypesProfile::SuperAdmin);
    check_user(TypesProfile::Admin);
    check_user(TypesProfile::User);

    // Part-3: Error Handling
    let result = divide(6, 0);
    match result {
        Ok(val) => println!("Divide-Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}

fn greet(name: &String) {
    println!("Inner-Name: {}", name);
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("you are fucked"))
    } else {
        Ok(a / b)
    }
}

fn check_user(user_type: TypesProfile) {
    match user_type {
        TypesProfile::SuperAdmin => println!("TEST_123_Super_admin"),
        TypesProfile::Admin => println!("TEST_123_Admin"),
        TypesProfile::User => println!("TEST_123_User"),
    }
}
