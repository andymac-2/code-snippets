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