use std::ops::Add;
use std::ops::Mul;
use std::mem::size_of_val;
use std::io;

#[derive(Copy, Clone, Debug, Hash)]
struct Just(i32);

#[derive(Copy, Clone, Debug, Hash)]
struct Nil();

trait Maybe {
    fn is_nil(&self);
    fn is_just(&self);
}

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