    trait MyTrait {}
    impl<T> MyTrait for T {}

    struct MyStruct<T> {
        data: T,
    }
    impl<T: MyTrait> MyStruct<T> {
        fn new(data: T) -> Self {
            MyStruct {
                data: data
            }
        }
    }

    fn bar() -> impl MyTrait {
        123
    }

    fn foo() -> impl MyTrait {
        bar()
    }

    fn main () {
        MyStruct::new(|| "Hello");
        MyStruct::new(foo());
        MyStruct::new(bar());
    }