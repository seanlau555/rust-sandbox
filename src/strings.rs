// Primitive str = Immutable fixed-length string somewhere in memory
// -----
// String = Growable, heap-allocated data structure - Use when you need
// to modify or own string data
//

pub fn run() {
    let hello = "Hello"; // primitive str

    let world = String::from("World"); // Growable, heap-allocated
    println!("{} {}", hello, world);

    // Get length
    println!("Length: {} {}", hello.len(), world.len());
}
