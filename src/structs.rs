//kinda like classes... Used to create custom data types
//! USE impl
//! //! A good thing to note is that when you want to return something, dont use ;

// Traditional struct, not a tuple struct. see below.
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Let's also see a tuple struct
struct Attendance(String, String, String);

//let's build a struct with functions.
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // First construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    // create a method
    fn full_name(&self) -> String {
        // format!() is a macro simillar to print. more info - https://doc.rust-lang.org/std/macro.format.html
        format!("{}{}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    
    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    }

    // Name to tuple 
    fn become_tuple(self) -> (String, String) {
      (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut attend = Attendance("j".to_string(), "a".to_string(), "b".to_string());

    println!("Your initials please: {}{}{}", attend.0, attend.1, attend.2);

    attend.1 = "u".to_string();
    println!("Your initials please: {}{}{}", attend.0, attend.1, attend.2);

    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    // with mut you can mutate properties on struct
    c.blue = 48;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut p = Person::new("John", " Doe");
    println!("Nice to meet you, I'm {}", p.full_name());
    println!("Hey {},{}!", p.first_name, p.last_name);
    p.set_last_name(" Dillon");
    println!("I'm your step-bro! {}!", p.full_name());

    p.set_first_name("Francis");
    println!("I guess I'm the step uncle, {}", p.full_name());

    //how to use tuple -> ("Francis", " Dillon")
    println!("Person tuple?: {:?}", p.become_tuple());
}
