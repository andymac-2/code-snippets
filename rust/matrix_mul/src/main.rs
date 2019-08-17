use std::ops::Neg;
use std::ops::Add;
use std::ops::Mul;
use std::mem::size_of;
use std::io;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Multivector3<E, E1, E2, E3, E12, E31, E23, E123> {
    e: E,
    e1: E1,
    e2: E2,
    e3: E3,
    e12: E12,
    e31: E31,
    e23: E23,
    e123: E123,
}
pub type W<T> = Multivector3<Just<T>, Nil, Nil, Nil, Nil, Nil, Nil, Nil>;
pub type X<T> = Multivector3<Nil, Just<T>, Nil, Nil, Nil, Nil, Nil, Nil>;
pub type Y<T> = Multivector3<Nil, Nil, Just<T>, Nil, Nil, Nil, Nil, Nil>;
pub type Z<T> = Multivector3<Nil, Nil, Nil, Just<T>, Nil, Nil, Nil, Nil>;
pub type XY<T> = Multivector3<Nil, Nil, Nil, Nil, Just<T>, Nil, Nil, Nil>;
pub type ZX<T> = Multivector3<Nil, Nil, Nil, Nil, Nil, Just<T>, Nil, Nil>;
pub type YZ<T> = Multivector3<Nil, Nil, Nil, Nil, Nil, Nil, Just<T>, Nil>;
pub type XYZ<T> = Multivector3<Nil, Nil, Nil, Nil, Nil, Nil, Nil, Just<T>>;
pub type Complex<T> = Multivector3<Just<T>, Nil, Nil, Nil, Nil, Nil, Nil, Just<T>>;
pub type Vec3<T> = Multivector3<Nil, Just<T>, Just<T>, Just<T>, Nil, Nil, Nil, Nil>;
pub type Bivec<T> = Multivector3<Nil, Nil, Nil, Nil, Just<T>, Just<T>, Just<T>, Nil>;
pub type Rotor<T> = Multivector3<Just<T>, Nil, Nil, Nil, Just<T>, Just<T>, Just<T>, Nil>;
pub type Line<T> = Multivector3<Nil, Just<T>, Just<T>, Just<T>, Just<T>, Just<T>, Just<T>, Nil>;
pub type Sphere<T> = Multivector3<Just<T>, Just<T>, Just<T>, Just<T>, Nil, Nil, Nil, Nil>;
pub type Plane<T> = Multivector3<Nil, Nil, Nil, Nil, Just<T>, Just<T>, Just<T>, Just<T>>;
pub type Frame<T> = Multivector3<Just<T>, Just<T>, Just<T>, Just<T>, Just<T>, Just<T>, Just<T>, Just<T>>;

macro_rules! mvec_e {
    () => {
        Multivector3<E, E1, E2, E3, E12, E31, E23, E123>
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Just<T>(T);
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Nil();

impl<T: Add> Add for Just<T> {
    type Output = Just<<T as Add>::Output>;
    fn add(self, rhs: Self) -> Self::Output {
        Just(self.0 + rhs.0)
    }
}
impl<T> Add<Nil> for Just<T> {
    type Output = Self;
    fn add(self, _rhs: Nil) -> Self::Output {
        self
    }
}
impl Add for Nil {
    type Output = Nil;
    fn add(self, _rhs: Nil) -> Self::Output {
        Nil()
    }
}
impl<T> Add<Just<T>> for Nil {
    type Output = Just<T>;
    fn add(self, rhs: Just<T>) -> Self::Output {
        rhs
    }
}

impl<T: Mul> Mul for Just<T> {
    type Output = Just<<T as Mul>::Output>;
    fn mul(self, rhs: Self) -> Self::Output {
        Just(self.0 * rhs.0)
    }
}
impl<T> Mul<Nil> for Just<T> {
    type Output = Nil;
    fn mul(self, _rhs: Nil) -> Self::Output {
        Nil()
    }
}
impl Mul for Nil {
    type Output = Nil;
    fn mul(self, _rhs: Nil) -> Self::Output {
        Nil()
    }
}
impl<T> Mul<Just<T>> for Nil {
    type Output = Nil;
    fn mul(self, _rhs: Just<T>) -> Self::Output {
        Nil()
    }
}

impl Neg for Nil {
    type Output = Nil;
    fn neg (self) -> Self::Output {
        Nil()
    }
}
impl<T: Neg> Neg for Just<T> {
    type Output = Just<<T as Neg>::Output>;
    fn neg (self) -> Self::Output {
        Just(-self.0)
    }
}

pub trait VecScale {
    type Output;
    fn value(self) -> Self::Output;
}
impl<A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8> 
    VecScale for ((A1, A2, A3, A4, A5, A6, A7, A8), (B1, B2, B3, B4, B5, B6, B7, B8))
where 
    A1: Mul<B1>,
    A2: Mul<B2>,
    A3: Mul<B3>,
    A4: Mul<B4>,
    A5: Mul<B5>,
    A6: Mul<B6>,
    A7: Mul<B7>,
    A8: Mul<B8>,
{
    type Output = (
        <A1 as Mul<B1>>::Output,
        <A2 as Mul<B2>>::Output,
        <A3 as Mul<B3>>::Output,
        <A4 as Mul<B4>>::Output,
        <A5 as Mul<B5>>::Output,
        <A6 as Mul<B6>>::Output,
        <A7 as Mul<B7>>::Output,
        <A8 as Mul<B8>>::Output,
    );
    fn value(self) -> Self::Output {
        let ((a1, a2, a3, a4, a5, a6, a7, a8), (b1, b2, b3, b4, b5, b6, b7, b8)) = self;
        (a1 * b1, a2 * b2, a3 * b3, a4 * b4,
            a5 * b5, a6 * b6, a7 * b7, a8 * b8)
    }
}

pub trait VecSum {
    type Output;
    fn value(self) -> Self::Output ;
}
impl <A1, A2> VecSum for (A1, A2) 
where
    A1: Add<A2>,
{
    type Output = <A1 as Add<A2>>::Output;
    fn value(self) -> Self::Output {
        let (a1, a2) = self;
        a1 + a2
    }
}
impl <A1, A2, A3> VecSum for (A1, A2, A3) 
where
    (A1, A2): VecSum,
    <(A1, A2) as VecSum>::Output: Add<A3>,
{
    type Output = <<(A1, A2) as VecSum>::Output as Add<A3>>::Output;
    fn value(self) -> Self::Output {
        let (a1, a2, a3) = self;
        (a1, a2).value() + a3
    }
}
impl <A1, A2, A3, A4> VecSum for (A1, A2, A3, A4) 
where
    (A1, A2, A3): VecSum,
    <(A1, A2, A3) as VecSum>::Output: Add<A4>,
{
    type Output = <<(A1, A2, A3) as VecSum>::Output as Add<A4>>::Output;
    fn value(self) -> Self::Output {
        let (a1, a2, a3, a4) = self;
        (a1, a2, a3).value() + a4
    }
}
impl <A1, A2, A3, A4, A5> VecSum for (A1, A2, A3, A4, A5) 
where
    (A1, A2, A3, A4): VecSum,
    <(A1, A2, A3, A4) as VecSum>::Output: Add<A5>,
{
    type Output = <<(A1, A2, A3, A4) as VecSum>::Output as Add<A5>>::Output;
    fn value(self) -> Self::Output {
        let (a1, a2, a3, a4, a5) = self;
        (a1, a2, a3, a4).value() + a5
    }
}
impl <A1, A2, A3, A4, A5, A6> VecSum for (A1, A2, A3, A4, A5, A6) 
where
    (A1, A2, A3, A4, A5): VecSum,
    <(A1, A2, A3, A4, A5) as VecSum>::Output: Add<A6>,
{
    type Output = <<(A1, A2, A3, A4, A5) as VecSum>::Output as Add<A6>>::Output;
    fn value(self) -> Self::Output {
        let (a1, a2, a3, a4, a5, a6) = self;
        (a1, a2, a3, a4, a5).value() + a6
    }
}
impl <A1, A2, A3, A4, A5, A6, A7> VecSum for (A1, A2, A3, A4, A5, A6, A7) 
where
    (A1, A2, A3, A4, A5, A6): VecSum,
    <(A1, A2, A3, A4, A5, A6) as VecSum>::Output: Add<A7>,
{
    type Output = <<(A1, A2, A3, A4, A5, A6) as VecSum>::Output as Add<A7>>::Output;
    fn value(self) -> Self::Output {
        let (a1, a2, a3, a4, a5, a6, a7) = self;
        (a1, a2, a3, a4, a5, a6).value() + a7
    }
}
impl <A1, A2, A3, A4, A5, A6, A7, A8> VecSum for (A1, A2, A3, A4, A5, A6, A7, A8) 
where
    (A1, A2, A3, A4, A5, A6, A7): VecSum,
    <(A1, A2, A3, A4, A5, A6, A7) as VecSum>::Output: Add<A8>,
{
    type Output = <<(A1, A2, A3, A4, A5, A6, A7) as VecSum>::Output as Add<A8>>::Output;
    fn value(self) -> Self::Output {
        let (a1, a2, a3, a4, a5, a6, a7, a8) = self;
        (a1, a2, a3, a4, a5, a6, a7).value() + a8
    }
}

pub struct Se<T>(T);
impl<T: Copy, E: Copy, E1: Copy, E2: Copy, E3: Copy, E12: Copy, E31: Copy, E23: Copy, E123: Copy> 
    Mul<&mvec_e!()> for Se<T> 
where 
    T: Mul<E> + Mul<E1> + Mul<E2> + Mul<E3> + Mul<E12> + Mul<E31> + Mul<E23> + Mul<E123>
{
    type Output = Multivector3<
        <T as Mul<E>>::Output, 
        <T as Mul<E1>>::Output, 
        <T as Mul<E2>>::Output, 
        <T as Mul<E3>>::Output, 
        <T as Mul<E12>>::Output, 
        <T as Mul<E31>>::Output, 
        <T as Mul<E23>>::Output, 
        <T as Mul<E123>>::Output
    >;
    fn mul(self, rhs: &mvec_e!()) -> Self::Output {
        Multivector3 {
            e: self.0 * rhs.e,
            e1: self.0 * rhs.e1,
            e2: self.0 * rhs.e2,
            e3: self.0 * rhs.e3,
            e12: self.0 * rhs.e12,
            e31: self.0 * rhs.e31,
            e23: self.0 * rhs.e23,
            e123: self.0 * rhs.e123,
        }
    }
}

pub struct Se1<T>(T);
impl<T: Copy, E: Copy, E1: Copy, E2: Copy, E3: Copy, E12: Copy, E31: Copy, E23: Copy, E123: Copy> 
    Mul<&mvec_e!()> for Se1<T> 
where 
    T: Neg + Mul<E> + Mul<E1> + Mul<E2> + Mul<E12> + Mul<E23> + Mul<E123>,
    <T as Neg>::Output: Mul<E31> + Mul<E3>,
{
    type Output = Multivector3<
        <T as Mul<E1>>::Output, 
        <T as Mul<E>>::Output, 
        <T as Mul<E12>>::Output, 
        <<T as Neg>::Output as Mul<E31>>::Output, 
        <T as Mul<E2>>::Output, 
        <<T as Neg>::Output as Mul<E3>>::Output, 
        <T as Mul<E123>>::Output, 
        <T as Mul<E23>>::Output
    >;
    fn mul(self, rhs: &mvec_e!()) -> Self::Output {
        Multivector3 {
            e: self.0 * rhs.e1,
            e1: self.0 * rhs.e,
            e2: self.0 * rhs.e12,
            e3: -self.0 * rhs.e31,
            e12: self.0 * rhs.e2,
            e31: -self.0 * rhs.e3,
            e23: self.0 * rhs.e123,
            e123: self.0 * rhs.e23,
        }
    }
}

pub struct Se2<T>(T);
impl<T: Copy, E: Copy, E1: Copy, E2: Copy, E3: Copy, E12: Copy, E31: Copy, E23: Copy, E123: Copy> 
    Mul<&mvec_e!()> for Se2<T> 
where 
    T: Neg + Mul<E> + Mul<E2> + Mul<E3> + Mul<E31> + Mul<E23> + Mul<E123>,
    <T as Neg>::Output: Mul<E1> + Mul<E12>,
{
    type Output = Multivector3<
        <T as Mul<E2>>::Output, 
        <<T as Neg>::Output as Mul<E12>>::Output, 
        <T as Mul<E>>::Output, 
        <T as Mul<E23>>::Output, 
        <<T as Neg>::Output as Mul<E1>>::Output, 
        <T as Mul<E123>>::Output, 
        <T as Mul<E3>>::Output, 
        <T as Mul<E31>>::Output
    >;
    fn mul(self, rhs: &mvec_e!()) -> Self::Output {
        Multivector3 {
            e: self.0 * rhs.e2,
            e1: -self.0 * rhs.e12,
            e2: self.0 * rhs.e,
            e3: self.0 * rhs.e23,
            e12: -self.0 * rhs.e1,
            e31: self.0 * rhs.e123,
            e23: self.0 * rhs.e3,
            e123: self.0 * rhs.e31,
        }
    }
}
pub struct Se3<T>(T);
impl<T: Copy, E: Copy, E1: Copy, E2: Copy, E3: Copy, E12: Copy, E31: Copy, E23: Copy, E123: Copy> 
    Mul<&mvec_e!()> for Se3<T> 
where 
    T: Neg + Mul<E> + Mul<E1> + Mul<E3> + Mul<E12> + Mul<E31> + Mul<E123>,
    <T as Neg>::Output: Mul<E2> + Mul<E23>,
{
    type Output = Multivector3<
        <T as Mul<E3>>::Output, 
        <T as Mul<E31>>::Output, 
        <<T as Neg>::Output as Mul<E23>>::Output, 
        <T as Mul<E>>::Output, 
        <T as Mul<E123>>::Output, 
        <T as Mul<E1>>::Output, 
        <<T as Neg>::Output as Mul<E2>>::Output, 
        <T as Mul<E12>>::Output
    >;
    fn mul(self, rhs: &mvec_e!()) -> Self::Output {
        Multivector3 {
            e: self.0 * rhs.e3,
            e1: self.0 * rhs.e31,
            e2: -self.0 * rhs.e23,
            e3: self.0 * rhs.e,
            e12: self.0 * rhs.e123,
            e31: self.0 * rhs.e1,
            e23: -self.0 * rhs.e2,
            e123: self.0 * rhs.e12,
        }
    }
}

pub struct Se12<T>(T);
impl<T: Copy, E: Copy, E1: Copy, E2: Copy, E3: Copy, E12: Copy, E31: Copy, E23: Copy, E123: Copy> 
    Mul<&Multivector3<E, E1, E2, E3, E12, E31, E23, E123>> for Se12<T> 
where 
    T: Neg + Mul<E2> + Mul<E> + Mul<E31> + Mul<E3>,
    <T as Neg>::Output: Mul<E12> + Mul<E1> + Mul<E123> + Mul<E23>,
{
    type Output = Multivector3<
        <<T as Neg>::Output as Mul<E12>>::Output, 
        <T as Mul<E2>>::Output, 
        <<T as Neg>::Output as Mul<E1>>::Output, 
        <<T as Neg>::Output as Mul<E123>>::Output, 
        <T as Mul<E>>::Output, 
        <<T as Neg>::Output as Mul<E23>>::Output, 
        <T as Mul<E31>>::Output, 
        <T as Mul<E3>>::Output
    >;
    fn mul(self, rhs: &mvec_e!()) -> Self::Output {
        Multivector3 {
            e: -self.0 * rhs.e12,
            e1: self.0 * rhs.e2,
            e2: -self.0 * rhs.e1,
            e3: -self.0 * rhs.e123,
            e12: self.0 * rhs.e,
            e31: -self.0 * rhs.e23,
            e23: self.0 * rhs.e31,
            e123: self.0 * rhs.e3,
        }
    }
}

pub struct Se31<T>(T);
impl<T: Copy, E: Copy, E1: Copy, E2: Copy, E3: Copy, E12: Copy, E31: Copy, E23: Copy, E123: Copy> 
    Mul<&Multivector3<E, E1, E2, E3, E12, E31, E23, E123>> for Se31<T> 
where 
    T: Neg + Mul<E1> + Mul<E23> + Mul<E> + Mul<E2>,
    <T as Neg>::Output: Mul<E31> + Mul<E3> + Mul<E123> + Mul<E12>,
{
    type Output = Multivector3<
        <<T as Neg>::Output as Mul<E31>>::Output, 
        <<T as Neg>::Output as Mul<E3>>::Output, 
        <<T as Neg>::Output as Mul<E123>>::Output, 
        <T as Mul<E1>>::Output, 
        <T as Mul<E23>>::Output, 
        <T as Mul<E>>::Output, 
        <<T as Neg>::Output as Mul<E12>>::Output, 
        <T as Mul<E2>>::Output
    >;
    fn mul(self, rhs: &mvec_e!()) -> Self::Output {
        Multivector3 {
            e: -self.0 * rhs.e31,
            e1: -self.0 * rhs.e3,
            e2: -self.0 * rhs.e123,
            e3: self.0 * rhs.e1,
            e12: self.0 * rhs.e23,
            e31: self.0 * rhs.e,
            e23: -self.0 * rhs.e12,
            e123: self.0 * rhs.e2,
        }
    }
}

pub struct Se23<T>(T);
impl<T: Copy, E: Copy, E1: Copy, E2: Copy, E3: Copy, E12: Copy, E31: Copy, E23: Copy, E123: Copy> 
    Mul<&mvec_e!()> for Se23<T> 
where 
    T: Neg + Mul<E3> + Mul<E12> + Mul<E> + Mul<E1>,
    <T as Neg>::Output: Mul<E23> + Mul<E123> + Mul<E2> + Mul<E31>,
{
    type Output = Multivector3<
        <<T as Neg>::Output as Mul<E23>>::Output, 
        <<T as Neg>::Output as Mul<E123>>::Output, 
        <T as Mul<E3>>::Output, 
        <<T as Neg>::Output as Mul<E2>>::Output, 
        <<T as Neg>::Output as Mul<E31>>::Output, 
        <T as Mul<E12>>::Output, 
        <T as Mul<E>>::Output, 
        <T as Mul<E1>>::Output
    >;
    fn mul(self, rhs: &mvec_e!()) -> Self::Output {
        Multivector3 {
            e: -self.0 * rhs.e23,
            e1: -self.0 * rhs.e123,
            e2: self.0 * rhs.e3,
            e3: -self.0 * rhs.e2,
            e12: -self.0 * rhs.e31,
            e31: self.0 * rhs.e12,
            e23: self.0 * rhs.e,
            e123: self.0 * rhs.e1,
        }
    }
}

pub struct Se123<T>(T);
impl<T: Copy, E: Copy, E1: Copy, E2: Copy, E3: Copy, E12: Copy, E31: Copy, E23: Copy, E123: Copy> 
    Mul<&mvec_e!()> for Se123<T> 
where 
    T: Neg + Mul<E3> + Mul<E2> + Mul<E1> + Mul<E>,
    <T as Neg>::Output: Mul<E123> + Mul<E23> + Mul<E31> + Mul<E12>,
{
    type Output = Multivector3<
        <<T as Neg>::Output as Mul<E123>>::Output, 
        <<T as Neg>::Output as Mul<E23>>::Output, 
        <<T as Neg>::Output as Mul<E31>>::Output, 
        <<T as Neg>::Output as Mul<E12>>::Output, 
        <T as Mul<E3>>::Output, 
        <T as Mul<E2>>::Output, 
        <T as Mul<E1>>::Output, 
        <T as Mul<E>>::Output
    >;
    fn mul(self, rhs: &mvec_e!()) -> Self::Output {
        Multivector3 {
            e: -self.0 * rhs.e123,
            e1: -self.0 * rhs.e23,
            e2: -self.0 * rhs.e31,
            e3: -self.0 * rhs.e12,
            e12: self.0 * rhs.e3,
            e31: self.0 * rhs.e2,
            e23: self.0 * rhs.e1,
            e123: self.0 * rhs.e,
        }
    }
}

impl<E: Copy, E1: Copy, E2: Copy, E3: Copy, E12: Copy, E31: Copy, E23: Copy, E123: Copy,
    F: Copy, F1: Copy, F2: Copy, F3: Copy, F12: Copy, F31: Copy, F23: Copy, F123: Copy> 
    Add<mvec_e!()>
    for Multivector3<F, F1, F2, F3, F12, F31, F23, F123>
where
    F: Add<E>,
    F1: Add<E1>,
    F2: Add<E2>,
    F3: Add<E3>,
    F12: Add<E12>,
    F31: Add<E31>,
    F23: Add<E23>,
    F123: Add<E123>,
{
    type Output = Multivector3 <
        <F as Add<E>>::Output,
        <F1 as Add<E1>>::Output,
        <F2 as Add<E2>>::Output,
        <F3 as Add<E3>>::Output,
        <F12 as Add<E12>>::Output,
        <F31 as Add<E31>>::Output,
        <F23 as Add<E23>>::Output,
        <F123 as Add<E123>>::Output,
    >;
    fn add(self, rhs: mvec_e!()) -> Self::Output {
        Multivector3 {
            e: self.e + rhs.e,
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2,
            e3: self.e3 + rhs.e3,
            e12: self.e12 + rhs.e12,
            e31: self.e31 + rhs.e31,
            e23: self.e23 + rhs.e23,
            e123: self.e123 + rhs.e123,
        }
    }
}


impl<'a, E: Copy, E1: Copy, E2: Copy, E3: Copy, E12: Copy, E31: Copy, E23: Copy, E123: Copy,
    F: Copy, F1: Copy, F2: Copy, F3: Copy, F12: Copy, F31: Copy, F23: Copy, F123: Copy> 
    Mul<&'a mvec_e!()>
    for &Multivector3<F, F1, F2, F3, F12, F31, F23, F123>
where
    Se<F>: Mul<&'a mvec_e!()>,
    Se1<F1>: Mul<&'a mvec_e!()>,
    Se2<F2>: Mul<&'a mvec_e!()>,
    Se3<F3>: Mul<&'a mvec_e!()>,
    Se12<F12>: Mul<&'a mvec_e!()>,
    Se31<F31>: Mul<&'a mvec_e!()>,
    Se23<F23>: Mul<&'a mvec_e!()>,
    Se123<F123>: Mul<&'a mvec_e!()>,
    (
        <Se<F> as Mul<&'a mvec_e!()>>::Output,
        <Se1<F1> as Mul<&'a mvec_e!()>>::Output,
        <Se2<F2> as Mul<&'a mvec_e!()>>::Output,
        <Se3<F3> as Mul<&'a mvec_e!()>>::Output,
        <Se12<F12> as Mul<&'a mvec_e!()>>::Output,
        <Se31<F31> as Mul<&'a mvec_e!()>>::Output,
        <Se23<F23> as Mul<&'a mvec_e!()>>::Output,
        <Se123<F123> as Mul<&'a mvec_e!()>>::Output,
    ): VecSum
{
    type Output = <(
        <Se<F> as Mul<&'a mvec_e!()>>::Output,
        <Se1<F1> as Mul<&'a mvec_e!()>>::Output,
        <Se2<F2> as Mul<&'a mvec_e!()>>::Output,
        <Se3<F3> as Mul<&'a mvec_e!()>>::Output,
        <Se12<F12> as Mul<&'a mvec_e!()>>::Output,
        <Se31<F31> as Mul<&'a mvec_e!()>>::Output,
        <Se23<F23> as Mul<&'a mvec_e!()>>::Output,
        <Se123<F123> as Mul<&'a mvec_e!()>>::Output,
    ) as VecSum>::Output;
    fn mul(self, rhs: &'a mvec_e!()) -> Self::Output {
        (
            Se(self.e) * rhs, 
            Se1(self.e1) * rhs, 
            Se2(self.e2) * rhs, 
            Se3(self.e3) * rhs, 
            Se12(self.e12) * rhs, 
            Se31(self.e31) * rhs, 
            Se23(self.e23) * rhs, 
            Se123(self.e123) * rhs, 
        ).value()
    }
}

pub fn main() {
    let mut line_buf = String::new();
    io::stdin().read_line(&mut line_buf).unwrap();
    let mut nums = line_buf.split_whitespace()
        .map(|str| str.parse::<i32>().unwrap());

    let a = nums.next().unwrap();
    let b = nums.next().unwrap();

    let ma = Multivector3 {
        e: Just(a), 
        e1: Nil(), e2: Nil(), e3: Nil(), 
        e12: Nil(), e31: Nil(), e23: Nil(), 
        e123: Nil()
    };
    let mb = Multivector3 {
        e: Just(b), 
        e1: Nil(), e2: Nil(), e3: Nil(), 
        e12: Nil(), e31: Nil(), e23: Nil(), 
        e123: Nil()
    };

    // HOLY SHIT!!! Rust is smart enough to figure out that both the regular
    // multiplication and the multivector multiplication are the same thing, and
    // not only optimises the multivector multiplication to a single
    // insturction, but also merges that with the original multiplication. This
    // is despite the fact that the multivector multiplication implementation is
    // hundreds of lines long.
    println!("Regular multiplication: {:?}", a * b);
    println!("Multivector multiplication: {:?}", &ma * &mb);
}

#[cfg(test)]
mod test {
    use super:: *;

    #[test]
    fn multiply_in_parts() {
        let a = Multivector3 {
            e: 2,
            e1: 1, e2: 7, e3: -5,
            e12: -3, e31: 5, e23: 2,
            e123: -7,
        };
        let scalar = Se(1);
        assert_eq!(scalar * &a, a);

        let e = Se(10);
        let re = e * &a;
        assert_eq!(re, Multivector3 { 
            e: 20, e1: 10, e2: 70, e3: -50, e12: -30, e31: 50, e23: 20, e123: -70 
        });
        let e1 = Se1(2);
        let re1 = e1 * &a;
        assert_eq!(re1, Multivector3 { 
            e: 2, e1: 4, e2: -6, e3: -10, e12: 14, e31: 10, e23: -14, e123: 4 
        });
        let e2 = Se2(3);
        let re2 = e2 * &a;
        assert_eq!(re2, Multivector3 { 
            e: 21, e1: 9, e2: 6, e3: 6, e12: -3, e31: -21, e23: -15, e123: 15 
        });
        let e3 = Se3(4);
        let re3 = e3 * &a;
        assert_eq!(re3, Multivector3 { 
            e: -20, e1: 20, e2: -8, e3: 8, e12: -28, e31: 4, e23: -28, e123: -12 
        });
        let e12 = Se12(1);
        let re12 = e12 * &a;
        assert_eq!(re12, Multivector3 { 
            e: 3, e1: 7, e2: -1, e3: 7, e12: 2, e31: -2, e23: 5, e123: -5 
        });
        let e31 = Se31(6);
        let re31 = e31 * &a;
        assert_eq!(re31, Multivector3 { 
            e: -30, e1: 30, e2: 42, e3: 6, e12: 12, e31: 12, e23: 18, e123: 42 
        });
        let e23 = Se23(2);
        let re23 = e23 * &a;
        assert_eq!(re23, Multivector3 { 
            e: -4, e1: 14, e2: -10, e3: -14, e12: -10, e31: -6, e23: 4, e123: 2 
        });
        let e123 = Se123(0);
        let re123 = e123 * &a;
        assert_eq!(re123, Multivector3 { 
             e: 0, e1: 0, e2: 0, e3: 0, e12: 0, e31: 0, e23: 0, e123: 0 
        });

        assert_eq!((re, re1, re2, re3, re12, re31, re23, re123).value(), Multivector3 { 
             e: -8, e1: 94, e2: 93, e3: -47, e12: -43, e31: 47, e23: -10, e123: -24
        });

    }

    #[test]
    fn add_full() {
        let b = Multivector3 {
            e: Just(10),
            e1: Just(2), e2: Just(3), e3: Just(4),
            e12: Just(1), e31: Just(6), e23: Just(2),
            e123: Just(0),
        };
        let a = Multivector3{
            e: Just(2),
            e1: Just(1), e2: Just(7), e3: Just(-5),
            e12: Just(-3), e31: Just(5), e23: Just(2),
            e123: Just(-7),
        };
        let result = Multivector3 {
            e: Just(12), 
            e1: Just(3), e2: Just(10), e3: Just(-1), 
            e12: Just(-2), e31: Just(11), e23: Just(4), 
            e123: Just(-7)
        };
        assert_eq!(b + a, result);
    }

    #[test]
    fn miltivector_compile_tests() {
        fn  mul_scalar (a: W<f64>, b: W<f64>) -> W<f64> {
            &a * &b
        }
        fn mul_vector (a: Vec3<f64>, b: Vec3<f64>) -> Rotor<f64> {
            &a * &b
        }
        fn mul_complex (a: Complex<f64>, b: Complex<f64>) -> Complex<f64> {
            &a * &b
        }
    }

    #[test] 
    fn sizes () {
        assert_eq!(size_of::<W<f64>>(), size_of::<f64>());
        assert_eq!(size_of::<Vec3<f64>>(), size_of::<f64>() * 3);
        assert_eq!(size_of::<Bivec<f64>>(), size_of::<f64>() * 3);
        assert_eq!(size_of::<Sphere<f64>>(), size_of::<f64>() * 4);
    }
}
