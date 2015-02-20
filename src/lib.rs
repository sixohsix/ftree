use std::boxed;
use std::any::Any;
use std::option;
use std::vec;

struct Digit<T> {
    vec: Vec<T>
}

impl<T> Digit<T> {
    fn new(val0: T) -> Self {
        let d : Digit<T>;
        d.vec.push(val0);
        return d;
    }
}

enum Node<T> {
    Node2(T, T),
    Node3(T, T, T)
}

trait FingerTree<T> {
    fn dequeue(&self, elem: T) -> &'static FingerTree<T>;
}

enum FingerTreeImpl<T> {
    Empty,
    Single(T),
    Deep(Digit<T>, Box<FingerTree<Node<T>> + 'static>, Digit<T>)
}

impl<T: 'static> FingerTree<T> for FingerTreeImpl<T> {
    fn dequeue(&self, elem: T) -> &'static FingerTree<T> {
        return match *self {
            FingerTreeImpl::Empty => FingerTreeImpl::Single(elem),
            FingerTreeImpl::Single(el) => FingerTreeImpl::Deep(
                Digit::new(el),
                Box::new(FingerTreeImpl::Empty),
                Digit::new(elem)),
            FingerTreeImpl::Deep(lDigit, ftree, rDigit) => FingerTreeImpl::Empty
        };
    }
}

#[test]
fn it_works() {
    let ft = FingerTreeImpl::Empty;
    let ft2 = ft.dequeue(123);
}
