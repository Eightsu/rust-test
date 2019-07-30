// Print Line Macro

pub fn run() {
    println!("Hello from print.rs file.");

    // Syntax to print numbers
    println!("Number : {}", 1);

    // Basic Formating
    println!("_sutra is from {}, and he is {}!", "Canada", 24);

    // Positional Arguments
    println!("_sutra can {1} and {0}", "walk", "talk");
    println!("_sutra can {0} the {1}", "walk", "talk");
    println!("_sutra can {0} the {1} and {1} the {0}", "walk", "talk");

    //Named Arguments
    println!(
        "{name} like to play the {activity}",
        name = "Jamar",
        activity = "guitar"
    );

    //Placeholder Traits
    println!("Binary: {:b} Hex {:x} Octal {:o}", 10, 10, 10);

    //Placeholder for Debug Trait
    // See Tuples https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
    println!("{:?}", (12, true, "hello"));

    // Math Expressions
    println!("Answer: 10 + 10 = {}", 10 + 10);
}
