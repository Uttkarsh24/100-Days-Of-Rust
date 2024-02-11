# Assignment On Modules - Day 18

This is the eighteenth day you have to Take this challenge and start your #100daysofrust journey.

## Task

### Assignment 1

```rs
// Something's missing. Fix the code so that it compiles.

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
```

- Solution - [Assignemnt 1](./assignment/assignment_1.rs)

### Assignment 2

```rs
// Complete the following code by addressing the TODO.

// TODO: Complete this use statement
use ???

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
```

- Solution - [Assignemnt 2](./assignment/assignment_2.rs)

### Assignment 3

```rs
// Complete the code by bringing the required items into scope.

mod days {
    pub enum WeekDay {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }
    
    pub fn is_holiday(day: &WeekDay) -> bool {
        match day {
            WeekDay::Sunday | WeekDay::Saturday => true,
            _ => false,
        }
    }
}

fn main() {
    let today = WeekDay::Friday;
    if is_holiday(&today) {
        println!("I can go out!");
    } else {
        println!("I have to work today!");
    }
}
```

- Solution - [Assignemnt 3](./assignment/assignment_3.rs)

### Assignment 4

```rs
// Complete the code by use declarations above main.

mod student {
    pub mod operations {
        use super::Student; // using super to refer to parent module

        pub fn assign_grade(student: &mut Student) {
            if student.marks >= 80 {
                student.grade = 'A';
            } else if student.marks >= 60 {
                student.grade = 'B';
            } else {
                student.grade = 'C';
            }
        }

    }

pub struct Student {
    pub name: String, // struct fields can also be made public
    pub marks: u8,
    pub grade: char,
}

impl Student {
    // make methods/associated functions public in order to access from outside the module
    pub fn new(name: &str, marks: u8) -> Self {
        Self {
            name: name.to_string(),
            marks,
            grade: 'X',
        }
    }
}

}

fn main() {
    let mut student = Student::new("Alice", 75);
    assign_grade(&mut student);
    println!("{} got {} grade", student.name, student.grade);
}
```

- Solution - [Assignemnt 4](./assignment/assignment_4.rs)

### Assignment 5

```rs
// Make the code compile by addressing the TODO.

mod delicious_snacks {
    // TODO: Fix these use statements
    use self::fruits::PEAR as ???
    use self::veggies::CUCUMBER as ???

    mod fruits { // 'static just implies that reference will be valid throughout program execution
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
```

- Solution - [Assignemnt 5](./assignment/assignment_5.rs)