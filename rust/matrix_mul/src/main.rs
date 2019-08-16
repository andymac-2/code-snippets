use std::ops::Neg;
use std::ops::Add;
use std::ops::Mul;
use std::mem::size_of_val;
use std::io;

#[derive(Copy, Clone, Debug, Hash)]
struct Just(i32);
#[derive(Copy, Clone, Debug, Hash)]
struct Nil();

impl Add for Just {
    type Output = Just;
    fn add(self, rhs: Just) -> Self::Output {
        Just(self.0 + rhs.0)
    }
}
impl Add<Nil> for Just {
    type Output = Just;
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
impl Add<Just> for Nil {
    type Output = Just;
    fn add(self, rhs: Just) -> Self::Output {
        rhs
    }
}

impl Mul for Just {
    type Output = Just;
    fn mul(self, rhs: Just) -> Self::Output {
        Just(self.0 * rhs.0)
    }
}
impl Mul<Nil> for Just {
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
impl Mul<Just> for Nil {
    type Output = Nil;
    fn mul(self, _rhs: Just) -> Self::Output {
        Nil()
    }
}

impl Neg for Nil {
    type Output = Nil;
    fn neg (self) -> Self::Output {
        Nil()
    }
}
impl Neg for Just {
    type Output = Just;
    fn neg (self) -> Self::Output {
        Just(-self.0)
    }
}

trait VecScale {
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

trait VecSum {
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

struct Multivector3<E, E1, E2, E3, E12, E31, E23, E123> {
    e: E,
    e1: E1,
    e2: E2,
    e3: E3,
    e12: E12,
    e31: E31,
    e23: E23,
    e123: E123,
}
macro_rules! mvec_e {
    () => {
        Multivector3<E, E1, E2, E3, E12, E31, E23, E123>
    }
}

struct Se<T>(T);
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

struct Se1<T>(T);
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

struct Se2<T>(T);
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
struct Se3<T>(T);
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

struct Se12<T>(T);
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

struct Se31<T>(T);
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

struct Se23<T>(T);
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

struct Se123<T>(T);
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
    Add<&mvec_e!()>
    for &Multivector3<F, F1, F2, F3, F12, F31, F23, F123>
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
    fn add(self, rhs: &mvec_e!()) -> Self::Output {
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



trait DotProduct {
    type Output;
    fn value(&self) -> Self::Output;
}
impl<R1: Copy, R2: Copy, C1: Copy, C2: Copy> DotProduct for ((R1, R2), (C1, C2))
where
    R1: Mul<C1>,
    R2: Mul<C2>,
    <R1 as Mul<C1>>::Output: Add<<R2 as Mul<C2>>::Output>,
{
    type Output = <<R1 as Mul<C1>>::Output as Add<<R2 as Mul<C2>>::Output>>::Output;
    fn value(&self) -> Self::Output{
        let ((r1, r2), (c1, c2)) = *self;
        r1 * c1 + r2 * c2
    }
}
impl<R1: Copy, R2: Copy, R3: Copy, C1: Copy, C2: Copy, C3: Copy> DotProduct 
for ((R1, R2, R3), (C1, C2, C3))
where
    ((R1, R2), (C1, C2)): DotProduct,
    R3: Mul<C3>,
    <((R1, R2), (C1, C2)) as DotProduct>::Output: Add<<R3 as Mul<C3>>::Output>
{
    type Output = <<((R1, R2), (C1, C2)) as DotProduct>::Output as Add<<R3 as Mul<C3>>::Output>>::Output;
    fn value(&self) -> Self::Output{
        let ((r1, r2, r3), (c1, c2, c3)) = *self;
        ((r1, r2), (c1, c2)).value() + r3 * c3
    }
}

#[derive(Clone, Debug)]
struct Matrix22<UL, UR, LL, LR> {
    ul: UL, ur: UR,
    ll: LL, lr: LR,
}
impl<UL1: Copy, UR1: Copy, LL1: Copy, LR1: Copy, UL2: Copy, UR2: Copy, LL2: Copy, LR2: Copy>
    Mul<&Matrix22<UL2, UR2, LL2, LR2>> for &Matrix22<UL1, UR1, LL1, LR1> 
where
    ((UL1, UR1), (UL2, LL2)): DotProduct,
    ((UL1, UR1), (UR2, LR2)): DotProduct,
    ((LL1, LR1), (UL2, LL2)): DotProduct,
    ((LL1, LR1), (UR2, LR2)): DotProduct,
{
    type Output = Matrix22<
        <((UL1, UR1), (UL2, LL2)) as DotProduct>::Output, 
        <((UL1, UR1), (UR2, LR2)) as DotProduct>::Output,
        <((LL1, LR1), (UL2, LL2)) as DotProduct>::Output, 
        <((LL1, LR1), (UR2, LR2)) as DotProduct>::Output,
    >;

    fn mul(self, rhs: &Matrix22<UL2, UR2, LL2, LR2>) -> Self::Output {
        Matrix22 {
            ul: ((self.ul, self.ur), (rhs.ul, rhs.ll)).value(),
            ur: ((self.ul, self.ur), (rhs.ur, rhs.lr)).value(),
            ll: ((self.ll, self.lr), (rhs.ul, rhs.ll)).value(),
            lr: ((self.ll, self.lr), (rhs.ur, rhs.lr)).value(),
        }
    }
}
impl<UL1: Copy, UR1: Copy, LL1: Copy, LR1: Copy, UL2: Copy, UR2: Copy, LL2: Copy, LR2: Copy>
    Add<&Matrix22<UL2, UR2, LL2, LR2>> for &Matrix22<UL1, UR1, LL1, LR1> 
where
    UL1: Add<UL2>,
    UR1: Add<UR2>,
    LL1: Add<LL2>,
    LR1: Add<LR2>,
{
    type Output = Matrix22<
        <UL1 as Add<UL2>>::Output, 
        <UR1 as Add<UR2>>::Output,
        <LL1 as Add<LL2>>::Output,
        <LR1 as Add<LR2>>::Output
    >;

    fn add(self, rhs: &Matrix22<UL2, UR2, LL2, LR2>) -> Self::Output {
        Matrix22 {
            ul: self.ul + rhs.ul,
            ur: self.ur + rhs.ur,
            ll: self.ll + rhs.ll,
            lr: self.lr + rhs.lr,
        }
    }
}



fn main() {
    let mut line_buf = String::new();
    io::stdin().read_line(&mut line_buf).unwrap();
    let mut nums = line_buf.split_whitespace()
        .map(|str| str.parse::<i32>().unwrap());

    let matrix_identity = Matrix22 {
        ul: Just(1),   ur: Nil(),
        ll: Nil(),      lr: Just(1),
    };
    let matrix_full = Matrix22 {
        ul: Just(nums.next().unwrap()),   ur: Just(nums.next().unwrap()),
        ll: Just(nums.next().unwrap()),   lr: Just(nums.next().unwrap()),
    };
    let result = &matrix_identity * &matrix_full;
    let result = &matrix_identity * &result;
    let result = &result * &matrix_identity;
    let result = &matrix_identity * &result;
    let result = &result * &matrix_identity;
    let result = &matrix_identity * &result;
    let result = &result * &matrix_identity;
    let result = &matrix_identity * &result;
    let result = &result * &matrix_identity;
    let result = &matrix_identity * &result;
    let result = &result * &matrix_identity;

    println!("[{:?},\t {:?}]\n[{:?},\t {:?}]", &result.ul, &result.ur, &result.ll, &result.lr)
}


// #[derive(Clone, Debug)]
// struct Matrix33<UL, UM, UR, ML, MM, MR, LL, LM, LR> {
//     ul: UL, um: UM, ur: UR,
//     ml: ML, mm: MM, mr: MR,
//     ll: LL, lm: LM, lr: LR,
// }
// impl<UL1: Copy, UM1: Copy, UR1: Copy, 
//     ML1: Copy, MM1: Copy, MR1: Copy, 
//     LL1: Copy, LM1: Copy, LR1: Copy,
//     UL2: Copy, UM2: Copy, UR2: Copy, 
//     ML2: Copy, MM2: Copy, MR2: Copy, 
//     LL2: Copy, LM2: Copy, LR2: Copy>
//     Mul<&Matrix33<UL2, UM2, UR2, ML2, MM2, MR2, LL2, LM2, LR2>> 
//     for &Matrix33<UL1, UM1, UR1, ML1, MM1, MR1, LL1, LM1, LR1> 
// where
//     ((UL1, UM1, UR1), (UL2, ML2, LL2)): DotProduct,
//     ((UL1, UM1, UR1), (UM2, MM2, LM2)): DotProduct,
//     ((UL1, UM1, UR1), (UR2, MR2, LR2)): DotProduct,
//     ((ML1, MM1, MR1), (UL2, ML2, LL2)): DotProduct,
//     ((ML1, MM1, MR1), (UM2, MM2, LM2)): DotProduct,
//     ((ML1, MM1, MR1), (UR2, MR2, LR2)): DotProduct,
//     ((LL1, LM1, LR1), (UL2, ML2, LL2)): DotProduct,
//     ((LL1, LM1, LR1), (UM2, MM2, LM2)): DotProduct,
//     ((LL1, LM1, LR1), (UR2, MR2, LR2)): DotProduct,
// {
//     type Output = Matrix33<
//         <((UL1, UR1), (UL2, LL2)) as DotProduct>::Output, 
//         <((UL1, UR1), (UR2, LR2)) as DotProduct>::Output,
//         <((LL1, LR1), (UL2, LL2)) as DotProduct>::Output, 
//         <((LL1, LR1), (UR2, LR2)) as DotProduct>::Output,
//     >;

//     fn mul(self, rhs: &Matrix22<UL2, UR2, LL2, LR2>) -> Self::Output {
//         Matrix22 {
//             ul: ((self.ul, self.ur), (rhs.ul, rhs.ll)).value(),
//             ur: ((self.ul, self.ur), (rhs.ur, rhs.lr)).value(),
//             ll: ((self.ll, self.lr), (rhs.ul, rhs.ll)).value(),
//             lr: ((self.ll, self.lr), (rhs.ur, rhs.lr)).value(),
//         }
//     }
// }