use std::cell::RefCell;
use std::fmt;
use std::mem;
use std::rc::Rc;

enum PrimitiveThunk<T> {
    Value(T),
    Func(Box<Fn() -> T>),
    Lock,
}

impl<T> PrimitiveThunk<T> {
    pub fn pure(value: T) -> PrimitiveThunk<T> {
        PrimitiveThunk::Value(value)
    }
    pub fn from_func(func: Box<Fn() -> T>) -> PrimitiveThunk<T> {
        PrimitiveThunk::Func(func)
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
impl<T: Clone> PrimitiveThunk<T> {
    pub fn eval(&mut self) -> T {
        self.force();
        if let PrimitiveThunk::Value(value) = self {
            return value.clone();
        }
        panic!("PrimitiveThunk: poisoned.");
    }
}

pub struct Thunk<T>(Rc<RefCell<PrimitiveThunk<T>>>);
impl<T> Clone for Thunk<T> {
    fn clone(&self) -> Thunk<T> {
        Thunk(self.0.clone())
    }
}
impl<T: fmt::Debug> fmt::Debug for Thunk<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0.borrow() {
            PrimitiveThunk::Value(ref value) => write!(f, "{:?}", value),
            PrimitiveThunk::Func(_) => write!(f, "Unevaluated"),
            PrimitiveThunk::Lock => write!(f, "Poisoned"),
        }
    }
}
impl<T> Thunk<T> {
    pub fn pure(value: T) -> Thunk<T> {
        Thunk(Rc::new(RefCell::new(PrimitiveThunk::pure(value))))
    }
    pub fn from_func<F: 'static + Fn() -> T>(func: F) -> Thunk<T> {
        Thunk(Rc::new(RefCell::new(PrimitiveThunk::from_func(Box::new(
            func,
        )))))
    }
}
impl<T: Clone> Thunk<T> {
    pub fn force(&self) {
        self.0.borrow_mut().force()
    }
    pub fn eval(&self) -> T {
        self.0.borrow_mut().eval()
    }

    pub fn bind_strict<U, F: Fn(T) -> Thunk<U>>(&self, f: F) -> Thunk<U> {
        f(self.eval())
    }
    pub fn map_strict<U, F: Fn(T) -> U>(&self, f: F) -> Thunk<U> {
        Thunk::pure(f(self.eval()))
    }
    pub fn join_strict(input: &Thunk<Thunk<T>>) -> Thunk<T> {
        Thunk::pure(input.eval().eval())
    }
}
impl<T: 'static + Clone> Thunk<T> {
    pub fn bind<U: Clone, F: 'static + Fn(T) -> Thunk<U>>(&self, f: F) -> Thunk<U> {
        let thunk_ref = self.clone();
        Thunk::from_func(move || f(thunk_ref.eval()).eval())
    }
    pub fn map<U, F: 'static + Fn(T) -> U>(&self, f: F) -> Thunk<U> {
        let thunk_ref = self.clone();
        Thunk::from_func(move || f(thunk_ref.eval()))
    }
}
impl<T: 'static + Clone> Thunk<Thunk<T>> {
    pub fn join(&self) -> Thunk<T> {
        let thunk_ref = self.clone();
        Thunk::from_func(move || thunk_ref.eval().eval())
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
pub struct List<T>(Thunk<ListInner<T>>);
impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl<T> Clone for List<T> {
    fn clone(&self) -> List<T> {
        List(self.0.clone())
    }
}
impl<T> List<T> {
    pub fn nil() -> List<T> {
        List(Thunk::pure(ListInner::Nil))
    }
    pub fn cons(value: T, this: &List<T>) -> List<T> {
        List(Thunk::pure(ListInner::Cons(value, this.clone())))
    }
    pub fn pure(value: T) -> List<T> {
        List::cons(value, &List::nil())
    }
}
impl<T: Clone> List<T> {
    fn bind_thunk_strict<U, F: Fn(ListInner<T>) -> List<U>>(&self, f: F) -> List<U> {
        f(self.0.eval())
    }
    pub fn map_strict<U, F: Clone + Fn(T) -> U>(&self, map_func: F) -> List<U> {
        self.bind_thunk_strict(|list| match list {
            ListInner::Nil => List::nil(),
            ListInner::Cons(x, xs) => List::cons(map_func(x), &xs.map_strict(map_func.clone())),
        })
    }
}
impl<T: 'static + Clone> List<T> {
    fn bind_thunk<U: Clone, F: 'static + Fn(ListInner<T>) -> List<U>>(&self, f: F) -> List<U> {
        let list_ref = self.0.clone();
        List(Thunk::from_func(move || f(list_ref.eval()).0.eval()))
    }
    pub fn map<U: 'static + Clone, F: 'static + Clone + Fn(T) -> U>(&self, map_func: F) -> List<U> {
        self.bind_thunk(move |list| match list {
            ListInner::Nil => List::nil(),
            ListInner::Cons(x, xs) => List::cons(map_func(x.clone()), &xs.map(map_func.clone())),
        })
    }
    pub fn bind<U: 'static + Clone, F: 'static + Clone + Fn(T) -> List<U>>(&self, f: F) -> List<U> {
        self.map(f).join()
    }
    pub fn append(&self, tail: &List<T>) -> List<T> {
        let tail = tail.clone();
        self.bind_thunk(move |list| match list {
            ListInner::Nil => tail.clone(),
            ListInner::Cons(x, xs) => List::cons(x.clone(), &xs.append(&tail)),
        })
    }
}
impl<T: 'static + Clone> List<List<T>> {
    pub fn join(&self) -> List<T> {
        self.bind_thunk(|list| match list {
            ListInner::Nil => List::nil(),
            ListInner::Cons(xs, xss) => xs.append(&xss.join()),
        })
    }
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
