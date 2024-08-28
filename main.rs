#[derive(Debug, PartialEq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::fmt::Display;
use std::ops::Deref;

struct MyBox<T>(T); // tuple struct

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // return a reference to the value we are pointing to
                // 拒绝引用递归
    }
}
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference the MyBox
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

impl List {
    fn from_array(arr: &[i32]) -> List {
        let mut list = List::Nil;
        for &i in arr.iter().rev() {
            list = Cons(i, Box::new(list));
        }
        list
        // second way to write
        // arr.iter().fold(List::Nil, |list, &i| Cons(i, Box::new(list)))

        // another way to write
        // if let [head, tail @ ..] = arr {
        //     Cons(*head, Box::new(List::from_array(tail)))
        // } else {
        //     List::Nil
        // }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = self;
        loop {
            match list {
                Cons(i, next) => {
                    write!(f, "{}, ", i)?;
                    list = next;
                }
                Nil => {
                    write!(f, "Nil")?;
                    break;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_array() {
        let list = List::from_array(&[1, 2, 3]);
        assert_eq!(
            list,
            Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
        );
    }

    #[test]
    fn test_display() {
        let list = List::from_array(&[1, 2, 3]);
        assert_eq!(format!("{}", list), "1, 2, 3, Nil");
    }

    #[test]
    fn test_mybox() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
