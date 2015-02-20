
use std::clone::Clone;

enum List {
    Empty,
    Value(isize, Box<List>)
}

impl Clone for List {
    fn clone(&self) -> Self {
        match self {
            &List::Empty => List::Empty,
            &List::Value(v, ref next) => List::Value(v, next.clone())
        }
    }
}

impl List {
    fn new() -> List {
        return List::Empty;
    }
    fn append(&self, value: isize) -> List {
        match self {
            &List::Empty => List::Value(value, Box::new(List::Empty)),
            &List::Value(v, ref next) => List::Value(v, Box::new(next.append(value)))
        }
    }
    fn head(&self) -> isize {
        match self {
            &List::Empty => panic!("Empty list"),
            &List::Value(v, _) => v
        }
    }
    fn tail(&self) -> List {
        match self {
            &List::Empty => panic!("Empty list"),
            &List::Value(_, ref t) => *t.clone()
        }
    }
    fn empty(&self) -> bool {
        match self {
            &List::Empty => true,
            _ => false
        }
    }
}

#[test]
fn it_works() {
    let l = List::new();
    let l0 = l.append(123);
    let l1 = l0.append(456);
    assert_eq!(l1.head(), 123);
    assert_eq!(l1.tail().head(), 456);
    assert!(l0.tail().empty());
}
