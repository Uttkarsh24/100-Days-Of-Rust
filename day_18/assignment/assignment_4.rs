mod student {
    pub mod operations {
        use super::Student;
  
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
    pub name: String,
    pub marks: u8,
    pub grade: char,
  }
  
  impl Student {
    pub fn new(name: &str, marks: u8) -> Self {
        Self {
            name: name.to_string(),
            marks,
            grade: 'X',
        }
    }
  }
  
  }
  
  use student::{Student, operations::assign_grade};
  
  fn main() {
    let mut student = Student::new("Alice", 75);
    assign_grade(&mut student);
    println!("{} got {} grade", student.name, student.grade);
  }