    use std::ops::Mul;
    use std::ops::Div;

    struct Length<T>(T);
    struct Acceleration<T>(T);
    #[derive(Debug)]
    struct Time<T>(T);
    struct Time2<T>(T);

    impl<T: Mul<Output=T>> Mul<T> for Length<T> {
        type Output = Self;
        fn mul (self, rhs: T) -> Self::Output {
            Length(self.0 * rhs)
        }
    }

    impl<T: Div<Output=T>> Div<Acceleration<T>> for Length<T> {
        type Output = Time2<T>;
        fn div (self, rhs: Acceleration<T>) -> Self::Output {
            Time2(self.0 / rhs.0)
        }
    }

    trait Sqrt {
        type Output;
        fn sqrt (self) -> Self::Output;
    }

    impl Sqrt for f64 {
        type Output = Self;
        fn sqrt (self) -> Self::Output {
            f64::sqrt(self)
        }
    }

    impl Sqrt for f32 {
        type Output = Self;
        fn sqrt (self) -> Self::Output {
            f32::sqrt(self)
        }
    }

    impl<T: Sqrt<Output = T>> Sqrt for Time2<T> {
        type Output = Time<T>;
        fn sqrt (self) -> Self::Output {
            Time(self.0.sqrt())
        }
    }

    fn time_to_hit_ground<T> (height: Length<T>, gravity: Acceleration<T>) -> Time<T> 
        where T: Mul<Output=T> + Div<Output=T> + Sqrt<Output=T> + From<f32>
    {
        (height * T::from(2.0) / gravity).sqrt()
    }

    fn main () {
        let height: Length<f64> = Length(20.0);
        let gravity: Acceleration<f64> = Acceleration(9.8);

        println!("{:?}", time_to_hit_ground(height, gravity));

        let height: Length<f32> = Length(20.0);
        let gravity: Acceleration<f32> = Acceleration(9.8);

        println!("{:?}", time_to_hit_ground(height, gravity));
    }