use std::marker::PhantomData;
use std::any::Any;

struct Reflexivity<T>(T, T);

struct MyChild <T> {
    name: &'static str,
    phantom: PhantomData<T>,
}
impl<T> MyChild<T> {
    fn new (name: &'static str) -> Self {
        MyChild {
            name: name,
            phantom: PhantomData,
        }
    }

    fn print (&self) {
        println!("I am a child of {}.", self.name);
    }
}

struct MyParent <T> {
    name: &'static str,
    phantom: PhantomData<T>,
}

fn new_parent (name: &'static str) -> MyParent<Box<dyn Any>> {
    MyParent {
        name: name,
        phantom: PhantomData as PhantomData<Box<dyn Any>>,
    }
}

impl<T> MyParent<T> {
    fn get_child (&self) -> MyChild<T> {
        MyChild::new(self.name)
    }
    fn use_child (&self, name: &MyChild<T>) {
        println!("My name is {}.", self.name);
        name.print();
    }
}

fn main () {
    let a = new_parent("Alice");
    let b = new_parent("Bob");

    let _proof_a_equals_b = Reflexivity(&a, &b);

    let name_a = a.get_child();
    let name_b = b.get_child();

    // We want to be able to use an object with the parent which created it.
    a.use_child(&name_a);
    b.use_child(&name_b);

    // We want to avoid this: using an object with a different parent.
    // Currently it typechecks.
    a.use_child(&name_b);
    b.use_child(&name_a);
}
