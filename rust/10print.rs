fn dummy () {}

fn functional (a: u32) -> impl Fn (u32) -> u32 {
    |b| a + b;
}

let copy = dummy;
let add_three = functional(3);

fn main () {
    ()
}