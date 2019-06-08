use std::fmt;

#[derive(Debug)]
struct MyStruct {
    name: &'static str,
}

trait MyTrait {
    fn do_something(&self) {
        println!("Defalt implementation.");
    }
}

// default implementation
fn default_do_something<T: fmt::Debug>(this: &T) -> () {
    println!("Default debug display: {:?}", this);
}

// but then we have to let the compiler know what to choose
impl MyTrait for MyStruct {
    fn do_something(&self) {
        default_do_something(self);
    }
}

impl MyTrait for u32 {
    fn do_something(&self) {
        println!("A Number");
    }
}

fn main() {
    (5).do_something();
    MyStruct{name: "Bob"}.do_something();
}