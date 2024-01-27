fn main() {
    // tuple
    let tuple_e = ('E', 5i32, true);
    println!("{}, {} and {}.", tuple_e.0, tuple_e.1, tuple_e.2);

    // classic struct
    struct Student {
        name: String,
        level: u8,
        remote: bool,
    }

    let c_student = Student {
        name: String::from("Anmol Anand"),
        level: 4,
        remote: true,
    };
    println!(
        "{}, {}, {}.",
        c_student.name, c_student.level, c_student.remote
    );

    // tuple struct
    struct Grades(char, char, char, f32);

    let t_grades = Grades('A', 'A', 'A', 8.25);
    println!(
        "{}, {}, {} and {}.",
        t_grades.0, t_grades.1, t_grades.2, t_grades.3
    );

    // enum
    #[derive(Debug)]
    struct KeyPress(String, char);

    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }

    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEKeys(KeyPress),
        WEClick(MouseClick),
    }

    let we_load = WebEvent::WELoad(true);
    
    let click = MouseClick {
        x: 100,
        y: 100,
    };
    let we_click = WebEvent::WEClick(click);

    let keys = KeyPress (
        String::from("Ctrl+"),
        'N'
    );
    let we_key = WebEvent::WEKeys(keys);

    println!("WebEvent Enum Structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_key, we_click);
}