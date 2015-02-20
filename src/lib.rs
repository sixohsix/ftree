
use std::clone::Clone;

enum List<T: Clone> {
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
    fn new() -> List<T> {
        return List::Empty;
    }
    fn append(&self, value: T) -> List<T> {
        match self {
            &List::Empty => List::Value(value, Box::new(List::Empty)),
            &List::Value(ref v, ref next) => List::Value(v.clone(), Box::new(next.append(value)))
        }
    }
    fn head(&self) -> &T {
        match self {
            &List::Empty => panic!("Empty list"),
            &List::Value(ref v, _) => v
        }
    }
    fn tail(&self) -> List<T> {
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
    let l : List<isize> = List::new();
    let l0 = l.append(123);
    let l1 = l0.append(456);
    assert_eq!(*l1.head(), 123);
    assert_eq!(*l1.tail().head(), 456);
    assert!(l0.tail().empty());
}
