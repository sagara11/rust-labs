#[derive(Debug)]
struct Student<T, K> {
    grade: T,
    point: K,
    name: String,
}

impl Student<i32, f64> {
    fn new(name: &'static str, grade: i32, point: f64) -> Student<i32, f64> {
        Student::<i32, f64> {
            grade,
            point,
            name: name.to_string(),
        }
    }

    // We have to specify that 'self' is an argument.
    fn get_grade(&self) -> i32 {
        self.grade
    }
    // We can specify different kinds of ownership and mutability of self.
    fn inc_grade(&mut self) {
        self.grade += 1;
    }
    // There are three different types of 'self'
    fn destructor(self) {
        println!("Destructing {}", self.name);
    }
}

struct Context<'a>(&'a mut Student<i32, f64>);

fn main() {
    let mut object_1 = Student::new("Alice", 5, 10.0);
    let object_2 = Student::new("Bob", 6, 20.0);

    // let get_total_grade = |x: &Student<i32, f64>| x.point + object_1.point;
    let mut get_total_grade = |x: &Student<i32, f64>| {
        object_1.inc_grade();
        x.grade + object_1.get_grade()
    };

    // let mut ctx = Context(&mut object_1);
    assert_eq!(fn_1(&object_2, &mut get_total_grade), 12);
    assert_eq!(fn_1(&object_2, &mut get_total_grade), 13);

    // assert_eq!(get_total_grade(&object_2), 12);
    // assert_eq!(get_total_grade(&object_2), 13);
    // assert_eq!(get_total_grade(&object_2), 14);
    // assert_eq!(fn_1(&object_2, &mut ctx), 12);
}

fn fn_1<F>(obj_2: &Student<i32, f64>, mut func: F) -> i32
where
    F: FnMut(&Student<i32, f64>) -> i32,
{
    let object_2 = Student::new("Bob", 6, 20.0);
    func(&object_2)
}

fn test_even<F>(mut is_even: F)
where
    F: FnMut(&Box<Student<i32, f64>>),
{
    let mut Alice: Box<Student<i32, f64>> = Box::new(Student::<i32, f64> {
        grade: 2,
        point: 9.0,
        name: String::from("Alice"),
    });

    dbg!(is_even(&mut Alice));
}
