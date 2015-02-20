
use std::clone::Clone;

pub enum List<T: Clone> {
    Empty,
    Value(T, Box<List<T>>)
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        match self {
            &List::Empty => List::Empty,
            &List::Value(ref v, ref next) => List::Value(v.clone(), next.clone())
        }
    }
}

impl<T: Clone> List<T> {
    pub fn new() -> List<T> {
        return List::Empty;
    }
    pub fn append(&self, value: T) -> List<T> {
        match self {
            &List::Empty => List::Value(value, Box::new(List::Empty)),
            &List::Value(ref v, ref next) => List::Value(v.clone(), Box::new(next.append(value)))
        }
    }
    pub fn push(&self, v: T) -> List<T> {
        List::Value(v, Box::new(self.clone()));
    }
    pub fn head(&self) -> &T {
        match self {
            &List::Empty => panic!("Empty list"),
            &List::Value(ref v, _) => v
        }
    }
    pub fn tail(&self) -> List<T> {
        match self {
            &List::Empty => panic!("Empty list"),
            &List::Value(_, ref t) => *t.clone()
        }
    }
    pub fn empty(&self) -> bool {
        match self {
            &List::Empty => true,
            _ => false
        }
    }
    pub fn len(&self) -> usize {
        match self {
            &List::Empty => 0,
            &List::Value(_, ref next) => 1 + next.len()
        }
    }
    pub fn take(&self, count: usize) -> List<T> {
        match self {
            &List::Empty => List::Empty,
            &List::Value(ref v, ref next) => if (count == 0) {
                    List::Empty
                } else {
                    List::Value(v.clone(), Box::new(next.take(count - 1)))
                }
        }
    }
    pub fn concat(&self, other: List<T>) -> List<T> {
        match self {
            &List::Empty => other,
            &List::Value(ref v, ref next) => List::Value(v.clone(), Box::new(next.concat(other)))
        }
    }
}

#[test]
fn it_works() {
    let l : List<isize> = List::new();
    let l0 = l.append(123);
    let l1 = l0.append(456);
    assert_eq!(*l1.head(), 123);
    assert_eq!(*l1.tail().head(), 456);
    assert!(l0.tail().empty());
}
