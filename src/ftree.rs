use list::List;
use std::boxed::Box;

trait FingerTree<T> {
    fn dequeue(&self, value: T) -> Self;
}

#[derive(Copy)]
enum Node<T> {
    Node2(T, T),
    Node3(T, T, T)
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        *self
    }
}

fn makeNode<T: Clone>(list: List<T>) -> Node<T> {
    match list.len() {
        2 => Node::Node2(
            *list.head(),
            *list.tail().head()),
        3 => Node::Node3(
            *list.head(),
            *list.tail().head(),
            *list.tail().tail().head())
    }
}

enum FingerTreeImpl<T, S: FingerTree<Node<T>>> {
    Empty,
    Single(T),
    Deep(List<T>, S, List<T>)
}

impl<T, S: FingerTree<Node<T>>> FingerTreeImpl<T, S> {
    fn new() -> Self {
        return FingerTreeImpl::Empty;
    }
}

// impl<T: Clone, S: Clone> Clone for FingerTreeImpl<T> {
//     fn clone(&self) -> Self {
//         match self {
//             &FingerTreeImpl::Empty => FingerTreeImpl::Empty,
//             &FingerTreeImpl::Single(v) => FingerTreeImpl::Single(v),
//             &FingerTreeImpl::Deep(l, f, r) => FingerTreeImpl::Deep(
//                 l, f, r)
//         }
//     }
// }

impl<T: Clone, S: FingerTree<Node<T>>> FingerTree<T> for FingerTreeImpl<T, S> {
    fn dequeue(&self, v: T) -> Self {
        return match self {
            &FingerTreeImpl::Empty => FingerTreeImpl::Single(v.clone()),
            &FingerTreeImpl::Single(ref v0) =>
                FingerTreeImpl::Deep(
                    List::new().append(v.clone()),
                    FingerTreeImpl::new(),
                    List::new().append(v0.clone())),
            &FingerTreeImpl::Deep(lDigit, fTree, rDigit) =>
                if (lDigit.len() == 4) {
                    FingerTreeImpl::Deep(
                        List::new().append(v).concat(lDigit.take(1)),
                        fTree.dequeue(makeNode(lDigit.tail())),
                        rDigit)
                } else {
                    FingerTreeImpl::Deep(
                        lDigit.push(v),
                        fTree,
                        rDigit)
                }
        }
    }
}

#[test]
fn test_ftree() {
    let ft : FingerTreeImpl<isize> = FingerTreeImpl::new();
}
