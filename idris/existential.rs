struct Dummy<T>();

trait Dummy {};

struct MyName <T> {
    name: String,
}

struct MyNumber <T> {
    num: u32,
}

struct MyParent <T> {
    name: MyName<T>,
    num: MyNum<T>,
}

fn new (name: String, num: u32) -> MyParent<impl Dummy> {
    MyParent {
        name: name,
        num num,
    }
}

fn get_num<T> (&MyParent<T>) -> MyNumber<T> {
    MyNumber {
        num: self.num,
    }
}

fn use_num<T> (&mut MyParent<T>, wrapper: MyNumber<T>) {
    self.num = wrapper.num;
}

fn main () {
    
}