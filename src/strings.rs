// Primitive str = Immutable fixed length string somewhere in memory
// String = Growable, heap allocated data struct - use for modifying or "owning" string data

pub fn run() {
    // Create string
    let mut _hello = String::from("hello");
    println!("{}, nice to meet you", _hello);

    // Get Length
    println!("Length: {}", _hello.len());

    // push - for char-type Single char only
    _hello.push('W');
    println!("New Word: {}", _hello);

    // to push string
    _hello.push_str("hale");
    println!("{}", _hello);

    //capacity in bytes -> 10bytes
    println!("Capacity: {}", _hello.capacity());

    // Check if empty
    println!("Is this empty? {}", _hello.is_empty());

    //check for substring - has to be exact match.
    println!(
        "_hello...does this variable contain hello? {}",
        _hello.contains("hello")
    );

    //replace substr
    println!("Replace: {}", _hello.replace("Whale", "   Goodbye"));

    // Loop through string -> line 1: "Baby" line 2: "Boy"
    let who = "Baby Boy";
    for word in who.split_whitespace() {
        println!("{}", word);
    }

    // create string with certain capacity
    let mut s = String::with_capacity(10);
    s.push('b');
    s.push('a');
    s.push('c');
    s.push('k');

    println!("{}", s);

    // Assertion test - if success nothing happens, if fail, the compiler panics.
    assert_eq!(4, s.len());
    assert_eq!(10, s.capacity());
}
