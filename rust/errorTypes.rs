    use std::fmt;
    use std::error;

    #[derive(Debug)]
    enum Either<T, U> { // we can reuse this for other error types.
        Left(T),
        Right(U),
    }

    #[derive(Debug)]
    enum Triplet<T, U, V> {
        Left(T),
        Centre(V),
        Right(U),
    }

    #[derive(Debug)]
    enum Many<A=(), B=(), C=(), D=(), E=(), F=()> {
        A(A),
        B(B),
        C(C),
        D(D),
        E(E),
        F(F),
    }

    use Many::{A, B, C, D, E, F};

    #[derive(Debug)]
    struct PrintError();
    impl fmt::Display for PrintError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "The document failed to print.")
        }
    }
    impl error::Error for PrintError {}

    #[derive(Debug)]
    struct MemoryError();
    impl fmt::Display for MemoryError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Insufficient Memory")
        }
    }
    impl error::Error for MemoryError {}

    #[derive(Debug)]
    struct DisplayError();
    impl fmt::Display for DisplayError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "The output did not display correctly")
        }
    }
    impl error::Error for DisplayError {}

    fn my_func() -> Result<(), Either<PrintError, MemoryError>> {
        // your code
        Ok(())
    }

    fn my_func2() -> Result<(), Triplet<PrintError, MemoryError, DisplayError>> {
        // your code
        Err(Triplet::Centre(DisplayError()))
    }

    fn my_func3() -> Result<(), Many<PrintError, MemoryError, DisplayError>> {
        // your code
        Err(Many::C(DisplayError()))
    }

    fn main() {
        println!("my_func: {:?}", my_func());
        println!("my_func2: {:?}", my_func2());

        match my_func3() {
            Ok(unit) => println!("OK!"),
            Err(A(PrintError())) => println!("Print Error."),
            Err(B(MemoryError())) => println!("Memory Error."),
            Err(C(DisplayError())) => println!("Display Error."),
            Err(D(())) => panic!("Makes this not compile if we add new cases"),
            _ => panic!("Unknown Error."),
        }
        println!("my_func3: {:?}", my_func3());
    }

    


