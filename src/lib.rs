use std::boxed;
use std::any::Any;
use std::option;

#[derive(Copy,Debug)]
struct Digit<T> {
    elem: T,
    next: Option<Box<Digit<T>>>
}

impl <T> Digit<T> {
    fn full(&self) -> bool {
        fn full2<T_>(digit: Digit<T_>, count: isize) -> bool {
            if count == 0 {
                return true;
            }
            match digit {
                Digit{elem: e, next: Some(boxedDigit)} =>
                    return full2(*boxedDigit, count - 1),
                _ => return false
            }
        }
        return full2(*self, 4);
    }
}

#[derive(Copy,Debug)]
enum Node<T> {
    Node2(T, T),
    Node3(T, T, T)
}

trait FingerTree<T> {
    fn dequeue(&self, elem: T) -> &'static FingerTree<T>;
}

#[derive(Copy,Debug)]
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
                Digit{elem: el, next: None},
                Box::new(FingerTreeImpl::Empty),
                Digit{elem: elem, next: None}),
            FingerTreeImpl::Deep(lDigit, ftree, rDigit) => FingerTreeImpl::Empty
        };
    }
}

#[test]
fn it_works() {
    let ft = FingerTreeImpl::Empty;
    let ft2 = ft.dequeue(123);
}
