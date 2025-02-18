use std::fmt::Debug;
use std::str;
use std::io::{Error, ErrorKind};

fn main() {


    let mut my_stack:stack<i32> = stack{vec:vec![1,2,3,4,5], max_capacity: 7 };

    let mut new_stack:stack<&str> = stack::new(3);
    new_stack.push("Hello");
    new_stack.push("Hello");
    new_stack.push("Hello");

    new_stack = new_stack.print_F();
    new_stack.print_F();

    //let res_max_capa = new_stack.push("Hello");
    //println!("{:?}", res_max_capa);
    //println!(" {:?}", new_stack);


    my_stack.push(9);
    let res: Option<i32> = match my_stack.pop() {
        Some(x) => Some(x),
        None => None
    };
    println!("{:?}", res);


    let res2: Option<i32> = match my_stack.pop() {
        Some(x) => Some(x),
        None => None
    };
    println!("{:?}", res2);

    let res3: Option<i32> = match my_stack.peek() {
        Some(x) => Some(*x),
        None => None
    };
    println!("{:?}", res3);
    my_stack.pop();
    my_stack.pop();
    my_stack.pop();
    my_stack.pop();


    let res4: bool = my_stack.is_empty();
    println!("{:?}", res4);


}

#[derive (Debug)]
struct stack<T> {
    vec: Vec<T>,
    max_capacity: i32,

}

impl <T> stack<T> where T : Debug {

    fn print_F(mut self) -> Self {
        let mut buffer = Vec::with_capacity(self.max_capacity as usize);
        for i in 0..self.vec.len() as i32 {
            let buf = self.pop().unwrap();
            println!("{:?}", buf);
            buffer.push(buf);
        }

        for i in 0..buffer.len() as i32 {
            self.push(buffer.pop().unwrap());
        }
        self
    }

    fn push(&mut self, element: T) -> Result<i32,Error> {
        if self.max_capacity == self.vec.len() as i32 {Err(Error::new(ErrorKind::StorageFull, "stack overflow yes"))} else {
            &self.vec.push(element);
            Ok(1)}
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

    fn new (size: u8) -> Self {
        stack{vec:Vec::with_capacity(size as usize), max_capacity:size as i32}
    }

}