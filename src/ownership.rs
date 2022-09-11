//  The stack stores values in the order it gets them and removes the values
//  in the opposite order. This is referred to as last in, first out. Think
//  of a stack of plates: when you add more plates, you put them on top of
//  the pile, and when you need a plate, you take one off the top. Adding
//  or removing plates from the middle or bottom wouldn’t work as well!
//  Adding data is called pushing onto the stack, and removing data is
//  called popping off the stack. All data stored on the stack must have a known,
//  fixed size. Data with an unknown size at compile time or a size that might
//  change must be stored on the heap instead.
//
//  Rules:
//  1. Each value in Rust has an owner.
//  2. There can only be one owner at a time.
//  3. When the owner goes out of scope, the value will be dropped.
