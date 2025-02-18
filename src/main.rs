use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::str;

type Result<T> = std::result::Result<T, Error>; // Type-def for Result så vi slipper for at erklære Error er del af Result konstant

fn main() -> () {
    test_using_new();
    test_with_capacity();
}

#[derive(Debug)]
struct Stack<T> {
    vec: Vec<T>,
    max_capacity: i32,
}

impl<T> Stack<T>
where
    T: Debug,
{
    fn printf(mut self) -> Self {
        // Imiterer print af hvert element i en Stack og lede dem
        // tilbage til deres oprindelige placering
        let mut buffer = Vec::with_capacity(self.max_capacity as usize);
        for _i in 0..self.vec.len() as i32 {
            let placeholder: Option<T> = match self.pop() {
                None => None,
                Some(x) => Some(x),
            };
            println!("{:?}", placeholder);
            if placeholder.is_some() {
                buffer.push(placeholder.unwrap());
            }
        }

        for _i in 0..buffer.len() as i32 {
            let placeholder: Option<T> = match buffer.pop() {
                None => None,
                Some(x) => Some(x),
            };
            println!("{:?}", placeholder);
            if placeholder.is_some() {
                let _ = self.push(placeholder.unwrap());
            }
            // Discard her da vores push-fn returnerer Option<T>, men vi er ligeglade idet dette tilfælde kontrolleret
        }
        self
    }

    fn push(&mut self, element: T) -> Result<i32> {
        // Implementation af Stack-like datastruktur på i en Vector
        // med begrænsning på størrelse
        if self.max_capacity == self.vec.len() as i32 {
            Err(Error::new(ErrorKind::StorageFull, "stack overflow: yes"))
        } else {
            let _ = &self.vec.push(element);
            Ok(1)
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.vec.last()
    }

    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn new(size: u8) -> Self {
        // Instantiér struct med den bestemte kapacitet,
        // Vi er dog ikke sikre på at typen bliver inferred korrekt
        // Vi har dog ikke i de to "test" af vores program fået nogle fejl vedr. type-størrelse
        Stack {
            vec: Vec::with_capacity(size as usize),
            max_capacity: size as i32,
        }
    }
}

fn test_with_capacity() {
    let mut my_stack: Stack<i32> = Stack {
        vec: vec![1, 2, 3, 4, 5],
        max_capacity: 7,
    };
    let _ = my_stack.push(9);
    let res: Option<i32> = match my_stack.pop() {
        Some(x) => Some(x),
        None => None,
    };
    println!("{:?}", res);

    let res2: Option<i32> = match my_stack.pop() {
        //
        Some(x) => Some(x),
        None => None,
    };
    println!("Integer læst ud af pop: {:?}", res2); //

    let res3: Option<i32> = match my_stack.peek() {
        Some(x) => Some(*x), // Denne linje ser lidt sjov ud, men der sker bare de-ref af referencen
        None => None,
    };
    println!("Integer læst ud af peek: {:?}", res3); // Forvent integer her
    my_stack.pop();
    my_stack.pop();
    my_stack.pop();
    my_stack.pop();

    let res4: bool = my_stack.is_empty();
    println!("Er stack tom: {:?}", res4);
}

fn test_using_new() {
    let mut new_stack: Stack<&str> = Stack::new(3);
    let first_push: Result<i32> = match new_stack.push("Hello") {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::new(
            ErrorKind::StorageFull,
            "Error pushing element onto stack; it's full",
        )),
    };

    let second_push: Result<i32> = match new_stack.push("World") {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::new(
            ErrorKind::StorageFull,
            "Error pushing element onto stack; it's full",
        )),
    };

    let third_push: Result<i32> = match new_stack.push("!") {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::new(
            ErrorKind::StorageFull,
            "Error pushing element onto stack; it's full",
        )),
    };
    let fourth_push: Result<i32> = match new_stack.push("#") {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::new(
            ErrorKind::StorageFull,
            "Error pushing element onto stack; it's full",
        )),
    };

    println!("Attempt of push #1: {:?}", first_push);
    println!("Attempt of push #2: {:?}", second_push);
    println!("Attempt of push #3: {:?}", third_push);
    println!("Attempt of push #4: {:?}", fourth_push);

    new_stack = new_stack.printf();
    new_stack.printf();
}
