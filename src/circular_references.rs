// use std::{rc::Rc, cell::RefCell};

struct  Student {
    name: String
}

struct  Course {
    name: String
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course
}

impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(
            Enrollment::new(student, course)
        )
    }
}

pub fn main_cr() {
    let john = Student {
        name: "John".into()
    };

    let course = Course {
        name: "Intro to rust".into()
    };

    let mut p = Platform::new();
    p.enroll(&john, &course);

    for c in john.courses(p) {
        println!("john is taking {}", c);
    }
}