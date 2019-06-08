Forgive my essay.

If you create anonymous sum types, I imagine there will be plenty of people to use it to return from a function which may give multiple types. 

> Type information is not available for the user (even though any implementation will require an internal type marker)

So I'm no expert either, but the basic idea is that `enum` is like a dictionary (which has some kind of ordering), and anonymous sum types are like a set (which have no ordering, and presumably no duplicates). That means that there is strictly less information in an anonymous sum type than there is in an equivalent `enum`. That means we're going to be ok converting an `enum` to an anonymous sum type, but converting it back is going to be undefined unless we specify additional information.

    #[derive(Debug)]
    enum Triplet<T, U, V> {
        Left(T),
        Centre(V),
        Right(U),
    }

    fn from_triplet<T, U, V> (input: Triplet<T, U, V>) -> (T | U | V) {
        match input {
            Left(x) => x,
            Centre(y) => y,
            Right(z) => z,
        } // OK
    }

    fn from_anonymous<T, U, V> (input: (T | U | V)) -> Triplet<T, U, V> {
        match input {
            x: T => Left(x),
            y: U => Centre(y),
            z: V => Right(z),
        } // The behaviour of this is undefined.
    }

`from_anonymous` is undefined. If we have no ordering, then `(T | U | V)` is the same as `(V | U | T)`. How does the compiler know which type to assign to `Left`? Is it `T` from `(T | U | V)`, or `V` from `(V | U | T)`. In the general case, this is undecidable. We simply don't have enough information to cast an anonymous sum type to an `enum`, so you would have to provide ordering information. If you think you can use the order of the `match` arms to resolve this ambiguity, this may not be sufficient:

    fn from_anonymous2<T, U, V> (input: (T | U | V)) -> Triplet<T, U, V> {
        if let x: T = input {
            return Left(T);
        }
        if let y: U = input {
            return Centre(y);
        }
        if let z: v = input {
            return Left(z);
        }
    }

Here is a particularly perplexing function:

    fn unholy<F, S>(a: (F | S), b: (F | S)) {
        match a {
            first_a: F => {
                match b {
                    first_b: F => feed_the_world(),
                    second_b: S => fire_missiles(),
                }
            },
            second_a: S => {
                match b {
                    first_b: F => feed_the_world(),
                    second_b: S => fire_missiles(),
                }
            }
        }
    }

    fn main() {
        a: (u32 | &str) = 10;
        b: (&str | u32) = "Hello.";
        unholy(a, b);
    }

Do we feed the world or do we fire missiles? if `F` is `&str` and `S` is `u32`, we feed the world. if `F` is `u32` and `S` is `&str`, we fire missiles. The compiler has no way of knowing which is which. Creating an ordering based on earlier branches of a `match` won't solve the problem, because we could access the data through a series of `if let` expressions.

> An invariant of anonymous types (or it should be) is that any given type appears only once, but this cannot be guaranteed with the current type system.

Here's an example:

    trait MyTrait {
        fn do_something(&self) {
            println!("Defalt implementation.");
        }
    }

    impl<A, B> MyTrait for (A | B) {
        fn do_something(&self) {
            fire_missiles();
        }
    }

    impl<A, B, C> MyTrait for (A | B | C) {
        fn do_something(&self) {
            feed_the_world();
        }
    }

    fn blissfully_ignorant() {
        let x: (u32, u32, &str) = 15;
        x.do_something(); // confusing to say the least.
    }

if we call `blissfully_ignorant`, do we fire missiles or feed the world? `x` will collapse into an `(A | B)`. Do we select to fire missiles at monomorphisation? Imagine being the guy that has to track down the bug where only some of their usages of a generic function break because it turns out another of their generic functions would spit out a result with two types the same.

On the other hand, we could choose not to collapse it. If we do that then have another look at the `from anonymous` function I wrote above, and consider the implications of adding a duplicate type to already undefined behaviour.

Essentially I believe that you would have to restrict anonymous sum types to concrete types only, and enforce no duplicate types at compile time. Note that forbidding matching on anything except a concrete type essentially means you can only put data inside a union and not remove it when you need it.