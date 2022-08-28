// Primitive str = Immutable fixed-length string somewhere in memory
// -----
// String = Growable, heap-allocated data structure - Use when you need
// to modify or own string data
//

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
pub fn run() {
    let hello = "Hello"; // primitive str // &str type

    let mut world = String::from("World "); // Growable, heap-allocated

    // Get length
    println!("Length: {} {}", hello.len(), world.len());

    // Push a char
    world.push('Y');
    // Push string
    world.push_str("ou");

    // type: alloc:string:String
    print_type_of(&world);
    // type: &str
    print_type_of(&hello);

    // Capacity in bytes
    println!("Capacity: {}", world.capacity());

    // Check if empty
    println!("Is Empty: {}", world.is_empty());

    // Contains
    println!("Contains 'You': {}", world.contains("You"));

    // Replace
    println!("Replace: {}", world.replace("You", "Me"));

    // Loop through string by whitespace
    for word in world.split_whitespace() {
        println!("_{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{} {}", hello, world);
}
