// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;

    // If/Else
    if age >= 21 && check_id || person_of_age {
        println!("Bartender: What");
    } else if age < 21 && checkid {
        println!("Bartender: Sor, you have to leave");
    } else {
        println!("Need your ID");
    }

    // shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
