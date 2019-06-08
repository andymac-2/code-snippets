// INCOMPLETE!!!

use std::cell::RefCell;
use std::fmt;
use std::mem;
use std::rc::Rc;

/// A naked primitive thunk. For most use cases, use `Thunk` instead of
/// `PrimitiveThunk`.
///
/// This implementation assumes that the value will be read many times,
/// therefore the overhead of calling a boxed function tio evaluate the thunk's
/// value is negligible. If this assumption is incorrect for your
/// implementation, you may wish to use a monomorphised version of
/// `PrimitiveThunk`.
#[derive(Clone, Debug)]
enum PrimitiveThunk<T> {
    Value(T),
    Func(Box<dyn FnOnce() -> T),
    Lock,
}
impl<T> PrimitiveThunk<T> {
    pub fn pure(value: T) -> Self {
        PrimitiveThunk::Value(value)
    }
    pub fn from_func(func: FnOnce() -> T) -> Self {
        PrimitiveThunk::Func(Box::new(func))
    }
    pub fn force(&mut self) {
        if let PrimitiveThunk::Value(_) = self {
            return;
        }

        let old = mem::replace(self, PrimitiveThunk::Lock);
        if let PrimitiveThunk::Func(func) = old {
            let value = func();
            mem::replace(self, PrimitiveThunk::Value(value));
            return;
        }
        panic!("PrimitiveThunk: poisoned.");
    }
}
impl<T> Deref for PrimitiveThunk<T> {
    type Target = T;

    /// Returns a reference to the value inside a thunk.
    ///
    /// This function will panic if you attempt to dereference an unresolved or
    /// locked thunk. The `Thunk` struct will not panic on trying to dereference
    /// an unresolved thunk.
    fn deref(&self) -> &Self::Target {
        match self {
            Value(val) => val,
            Func(_) => panic!("Attempted to dereference unresolved primitive thunk."),
            Lock(_) => panic!("Attempted to dereference locked primitive thunk."),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Thunk<T>(RefCell<PrimitiveThunk<T>>);
impl<T> Thunk<T> {
    pub fn pure(value: T) -> Self {
        Thunk(RefCell::new(PrimitiveThunk::pure(value)))
    }
    pub fn from_func(func: FnOnce() -> T) -> Self {
        Thunk(RefCell::new(PrimitiveThunk::from_func(func)))
    }
    fn force(&self) {
        self.0.borrow_mut().force()
    }
}
impl<T> Deref for Thunk<T> {
    type Target = PrimitiveThunk<T>;

    fn deref(&self) -> &Self::Target {
        self.force();
        self.0.borrow()
    }
}

macro_rules! list {
    () => (List::nil());
    ( $head:expr ; $tail:expr ) => (List::cons($head, &($tail)));
    ( $head:expr $(,$tail:expr)* ) => (list![$head ; list![$($tail),*]]);
}

#[derive(Debug, Clone)]
enum ListInner<T> {
    Nil,
    Cons(T, List<T>),
}
#[derive(Clone)]
pub struct List<T>(Rc<Thunk<ListInner<T>>>);
impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl<T> List<T> {
    pub fn nil() -> List<T> {
        List(Thunk::pure(ListInner::Nil))
    }
    pub fn cons(head: T, tail: &List<T>) -> List<T> {
        List(Thunk::pure(ListInner::Cons(value, tail.clone())))
    }
    pub fn pure(value: T) -> List<T> {
        List::cons(value, &List::nil())
    }
}
impl<T: Clone> List<T> {
    pub fn append(&self, tail: &List<T>) -> List<T> {
        match ***self {
            ListInner::Nil => tail.clone(),
            ListInner::Cons(x, xs) => List::cons(x.clone(), &xs.append(&tail)),
        }
    }
    pub fn map<U, F: Fn(&T) -> U>(&self, map_func: F) -> List<U> {
        match ***self {
            ListInner::Nil => List::nil(),
            ListInner::Cons(x, xs) => List::cons(map_func(x), &xs.map(map_func.clone())),
        }
    }
    pub fn bind<U, F: 'static + Clone + Fn(T) -> List<U>>(&self, f: F) -> List<U> {
        self.map(f).join()
    }
}
impl<T> List<List<T>> {
    pub fn join(&self) -> List<T> {
        match ***self {
            ListInner::Nil => List::nil(),
            ListInner::Cons(xs, xss) => xs.append(&xss.join()),
        }
    }
}
impl<T: 'static + Clone> List<List<T>> {

    pub fn prod(&self) -> List<List<T>> {
        self.bind_thunk(move |list| match list {
            ListInner::Nil => list![list![]],
            ListInner::Cons(xs, xss) => {
                xs.bind(move |k| xss.prod().map(move |xs| list![k.clone(); &xs]))
            }
        })
    }
}
impl<T: Clone + fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut cell = self.clone();
        loop {
            match cell.0.eval() {
                ListInner::Nil => break,
                ListInner::Cons(x, xs) => {
                    write!(f, "{}", x)?;
                    cell = xs;
                    if let ListInner::Nil = cell.0.eval() {
                        break;
                    }
                }
            }
            write!(f, ", ")?;
        }
        write!(f, "]")
    }
}

fn main() {
    let list = list![1, 2, 3];
    println!("{}", list);
    let list2 = list![4, 5];
    println!("{}", list2);

    let list3 = List::cons(list, &List::pure(list2));
    println!("{}", list3);

    let list4 = List::prod(&list3);

    println!("{:?}", list4); // Unevaluated
    println!("{}", list4); // Evaluate to print
    println!("{:?}", list4); // The thunk is now evaluated.
}
