//! Algebra Operators
//!
//! Note that remaining operators are defined at the language level.
//! e.g. Trait `Not` for `!` operator (Bitwise or logical).
//!
//! All operator are defined in the (Ops)[https://doc.rust-lang.org/std/ops/index.html] documentation.

// Unary Operators

pub trait UnaryOperator<T, O = T>: Clone {
    fn op(value: T) -> O;
}

#[derive(Copy, Clone)]
pub struct IdendityOp;
#[derive(Copy, Clone)]
pub struct AbsoluteOp;
#[derive(Copy, Clone)]
pub struct AdditiveInverseOp;
#[derive(Copy, Clone)]
pub struct MultiplicativeInverseOp;
#[derive(Copy, Clone)]
pub struct LogicalNotOp;
#[derive(Copy, Clone)]
pub struct BitwiseNotOp;

// ------------------------------------------------------------------------------
// Unary Operators Implementations

impl<T> UnaryOperator<T> for IdendityOp {
    fn op(value: T) -> T {
        value
    }
}

macro_rules! impl_abs {
    ($($t:ty),*) => {
        $(
            impl UnaryOperator<$t> for AbsoluteOp {
                fn op(value: $t) -> $t {
                    value.abs()
                }
            }
        )*
    };
}

impl_abs!(i8, i16, i32, i64, f32, f64);

macro_rules! impl_abs_unsigned {
    ($($t:ty),*) => {
        $(
            impl UnaryOperator<$t> for AbsoluteOp {
                fn op(value: $t) -> $t {
                    value
                }
            }
        )*
    };
}

impl_abs_unsigned!(u8, u16, u32, u64);

impl<T> UnaryOperator<T> for AdditiveInverseOp
where
    Self: Clone,
    T: std::ops::Neg<Output = T>,
{
    fn op(value: T) -> T {
        -value
    }
}

macro_rules! impl_mul_inv {
    ($($t:ty),*) => {
        $(
            impl UnaryOperator<$t, f32> for MultiplicativeInverseOp {
                fn op(value: $t) -> f32 {
                    1.0f32 / value as f32
                }
            }
        )*
    };
}

macro_rules! impl_mul_inv_f64 {
    ($($t:ty),*) => {
        $(
            impl UnaryOperator<$t, f64> for MultiplicativeInverseOp {
                fn op(value: $t) -> f64 {
                    1.0f64 / value as f64
                }
            }
        )*
    };
}

impl_mul_inv!(i8, i16, u8, u16, f32);
impl_mul_inv_f64!(i32, i64, u32, u64, f64);

impl UnaryOperator<bool> for LogicalNotOp {
    fn op(value: bool) -> bool {
        !value
    }
}

impl<T> UnaryOperator<T> for BitwiseNotOp
where
    Self: Clone,
    T: std::ops::Not<Output = T>,
{
    fn op(value: T) -> T {
        !value
    }
}

// ------------------------------------------------------------------------------
// Binary Operators

pub trait BinaryOperator<D1, D2 = D1>: Clone {
    type Output;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output;
}

#[derive(Copy, Clone)]
pub struct LogicalOr {
    _marker: std::marker::PhantomData<()>,
}
impl LogicalOr {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl BinaryOperator<bool> for LogicalOr {
    type Output = bool;

    fn op(&self, lhs: bool, rhs: bool) -> Self::Output {
        lhs || rhs
    }
}

#[derive(Copy, Clone)]
pub struct LogicalAnd {
    _marker: std::marker::PhantomData<()>,
}
impl LogicalAnd {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl BinaryOperator<bool> for LogicalAnd {
    type Output = bool;

    fn op(&self, lhs: bool, rhs: bool) -> Self::Output {
        lhs && rhs
    }
}

#[derive(Copy, Clone)]
pub struct LogicalXor {
    _marker: std::marker::PhantomData<()>,
}
impl LogicalXor {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl BinaryOperator<bool> for LogicalXor {
    type Output = bool;

    fn op(&self, lhs: bool, rhs: bool) -> Self::Output {
        lhs ^ rhs
    }
}

#[derive(Copy, Clone)]
pub struct LogicalXnor {
    _marker: std::marker::PhantomData<()>,
}
impl LogicalXnor {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl BinaryOperator<bool> for LogicalXnor {
    type Output = bool;

    fn op(&self, lhs: bool, rhs: bool) -> Self::Output {
        !(lhs ^ rhs)
    }
}

#[derive(Copy, Clone)]
pub struct BitwiseOr<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> BitwiseOr<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for BitwiseOr<D1, D2>
where
    Self: Clone,
    D1: std::ops::BitOr<D2, Output = D1>,
{
    type Output = D1;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs | rhs
    }
}

#[derive(Copy, Clone)]
pub struct BitwiseAnd<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> BitwiseAnd<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for BitwiseAnd<D1, D2>
where
    Self: Clone,
    D1: std::ops::BitAnd<D2, Output = D1>,
{
    type Output = D1;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs & rhs
    }
}

#[derive(Copy, Clone)]
pub struct BitwiseXor<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> BitwiseXor<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for BitwiseXor<D1, D2>
where
    Self: Clone,
    D1: std::ops::BitXor<D2, Output = D1>,
{
    type Output = D1;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs ^ rhs
    }
}

#[derive(Copy, Clone)]
pub struct BitwiseXnor<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> BitwiseXnor<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for BitwiseXnor<D1, D2>
where
    Self: Clone,
    D1: std::ops::Not<Output = D1>,
    D1: std::ops::BitXor<D2, Output = D1>,
{
    type Output = D1;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        !(lhs ^ rhs)
    }
}

#[derive(Copy, Clone)]
pub struct Equal<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> Equal<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for Equal<D1, D2>
where
    Self: Clone,
    D1: PartialEq<D2>,
{
    type Output = bool;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs == rhs
    }
}

#[derive(Copy, Clone)]
pub struct NotEqual<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> NotEqual<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for NotEqual<D1, D2>
where
    Self: Clone,
    D1: PartialEq<D2>,
{
    type Output = bool;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs != rhs
    }
}

#[derive(Copy, Clone)]
pub struct GreaterThan<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> GreaterThan<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for GreaterThan<D1, D2>
where
    Self: Clone,
    D1: PartialOrd<D2>,
{
    type Output = bool;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs > rhs
    }
}

#[derive(Copy, Clone)]
pub struct LessThan<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> LessThan<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for LessThan<D1, D2>
where
    Self: Clone,
    D1: PartialOrd<D2>,
{
    type Output = bool;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs < rhs
    }
}

#[derive(Copy, Clone)]
pub struct GreaterThanOrEqual<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> GreaterThanOrEqual<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for GreaterThanOrEqual<D1, D2>
where
    Self: Clone,
    D1: PartialOrd<D2>,
{
    type Output = bool;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs >= rhs
    }
}

#[derive(Copy, Clone)]
pub struct LessThanOrEqual<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> LessThanOrEqual<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for LessThanOrEqual<D1, D2>
where
    Self: Clone,
    D1: PartialOrd<D2>,
{
    type Output = bool;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs <= rhs
    }
}

#[derive(Copy, Clone)]
pub struct First<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> First<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for First<D1, D2>
where
    Self: Clone,
    D1: Clone,
{
    type Output = D1;

    fn op(&self, lhs: D1, _: D2) -> Self::Output {
        lhs.clone()
    }
}

#[derive(Copy, Clone)]
pub struct Second<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> Second<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for Second<D1, D2>
where
    Self: Clone,
    D2: Clone,
{
    type Output = D2;

    fn op(&self, _: D1, rhs: D2) -> Self::Output {
        rhs.clone()
    }
}

#[derive(Copy, Clone)]
pub struct Minimum<D1> {
    _marker: std::marker::PhantomData<(D1, D1)>,
}
impl<D1> Minimum<D1> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1> BinaryOperator<D1> for Minimum<D1>
where
    D1: Clone,
    D1: PartialOrd<D1>,
{
    type Output = D1;

    fn op(&self, lhs: D1, rhs: D1) -> Self::Output {
        if lhs < rhs {
            lhs.clone()
        } else {
            rhs.clone()
        }
    }
}

#[derive(Copy, Clone)]
pub struct Maximum<D1> {
    _marker: std::marker::PhantomData<(D1, D1)>,
}
impl<D1> Maximum<D1> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1> BinaryOperator<D1> for Maximum<D1>
where
    D1: Clone,
    D1: PartialOrd<D1>,
{
    type Output = D1;

    fn op(&self, lhs: D1, rhs: D1) -> Self::Output {
        if lhs > rhs {
            lhs.clone()
        } else {
            rhs.clone()
        }
    }
}

#[derive(Copy, Clone)]
pub struct Addition<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> Addition<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for Addition<D1, D2>
where
    Self: Clone,
    D1: std::ops::Add<D2, Output = D1>,
{
    type Output = D1;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs + rhs
    }
}

#[derive(Copy, Clone)]
pub struct Subtraction<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> Subtraction<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for Subtraction<D1, D2>
where
    Self: Clone,
    D1: std::ops::Sub<D2, Output = D1>,
{
    type Output = D1;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs - rhs
    }
}

#[derive(Copy, Clone)]
pub struct Multiplication<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> Multiplication<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for Multiplication<D1, D2>
where
    Self: Clone,
    D1: std::ops::Mul<D2, Output = D1>,
{
    type Output = D1;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs * rhs
    }
}

#[derive(Copy, Clone)]
pub struct Division<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> Division<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<D1, D2> BinaryOperator<D1, D2> for Division<D1, D2>
where
    Self: Clone,
    D1: std::ops::Div<D2, Output = D1>,
{
    type Output = D1;

    fn op(&self, lhs: D1, rhs: D2) -> Self::Output {
        lhs / rhs
    }
}

// ------------------------------------------------------------------------------
// Binary Operators Implementations

// ---------------------------------------------------------------------------
// Monoids

pub trait Monoid<D>: Clone {
    fn identity(&self) -> D;

    fn operate(&self, lhs: D, rhs: D) -> D;
}

// ---------------------------------------------------------------------------
// Monoids implementations

macro_rules! impl_monoid_def {
    ($name:ident) => {
        #[derive(Copy, Clone)]
        pub struct $name<D> {
            _marker: std::marker::PhantomData<(D, D)>,
        }
        impl<D> $name<D> {
            pub fn new() -> Self {
                Self {
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

macro_rules! impl_monoid_detail {
    ($name:ident, $bin_op:ident, $domain:ty, $identity:expr) => {
        impl Monoid<$domain> for $name<$domain>
        where
            Self: Clone,
        {
            fn identity(&self) -> $domain {
                $identity
            }

            fn operate(&self, lhs: $domain, rhs: $domain) -> $domain {
                $bin_op::new().op(lhs, rhs)
            }
        }
    };
}

// Macro to associate a BinaryOp implementation with a Monoid
macro_rules! impl_monoid {
    ($name:ident, $bin_op:ident, $domain:ty, $identity:expr) => {
        impl_monoid_def!($name);
        impl_monoid_detail!($name, $bin_op, $domain, $identity);
    };
}

macro_rules! impl_monoid_many {
    ($name:ident, $bin_op:ident, $( ($domain:ty, $identity:expr) ), *) => {
        impl_monoid_def!($name);
        $(
            impl_monoid_detail!($name, $bin_op, $domain, $identity);
        )*
    };
}

impl_monoid_many!(
    PlusMonoid,
    Addition,
    (i8, 0),
    (i16, 0),
    (i32, 0),
    (i64, 0),
    (u8, 0),
    (u16, 0),
    (u32, 0),
    (u64, 0),
    (f32, 0.0),
    (f64, 0.0)
);

// TimesMonoid
impl_monoid_many!(
    TimesMonoid,
    Multiplication,
    (i8, 1),
    (i16, 1),
    (i32, 1),
    (i64, 1),
    (u8, 1),
    (u16, 1),
    (u32, 1),
    (u64, 1),
    (f32, 1.0),
    (f64, 1.0)
);

// MinMonoid
impl_monoid_many!(
    MinMonoid,
    Minimum,
    (i8, i8::MAX),
    (i16, i16::MAX),
    (i32, i32::MAX),
    (i64, i64::MAX),
    (u8, u8::MAX),
    (u16, u16::MAX),
    (u32, u32::MAX),
    (u64, u64::MAX),
    (f32, f32::INFINITY),
    (f64, f64::INFINITY)
);

// MaxMonoid
impl_monoid_many!(
    MaxMonoid,
    Maximum,
    (i8, i8::MIN),
    (i16, i16::MIN),
    (i32, i32::MIN),
    (i64, i64::MIN),
    (u8, u8::MIN),
    (u16, u16::MIN),
    (u32, u32::MIN),
    (u64, u64::MIN),
    (f32, f32::NEG_INFINITY),
    (f64, f64::NEG_INFINITY)
);

// Only works for bool
#[derive(Copy, Clone)]
pub struct LogicalOrMonoid {
    _marker: std::marker::PhantomData<()>,
}
impl LogicalOrMonoid {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl Monoid<bool> for LogicalOrMonoid {
    fn identity(&self) -> bool {
        false
    }

    fn operate(&self, lhs: bool, rhs: bool) -> bool {
        LogicalOr::new().op(lhs, rhs)
    }
}

#[derive(Copy, Clone)]
pub struct LogicalAndMonoid {
    _marker: std::marker::PhantomData<()>,
}
impl LogicalAndMonoid {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl Monoid<bool> for LogicalAndMonoid {
    fn identity(&self) -> bool {
        true
    }

    fn operate(&self, lhs: bool, rhs: bool) -> bool {
        LogicalAnd::new().op(lhs, rhs)
    }
}

#[derive(Copy, Clone)]
pub struct LogicalXorMonoid {
    _marker: std::marker::PhantomData<()>,
}
impl LogicalXorMonoid {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl Monoid<bool> for LogicalXorMonoid {
    fn identity(&self) -> bool {
        false
    }

    fn operate(&self, lhs: bool, rhs: bool) -> bool {
        LogicalXor::new().op(lhs, rhs)
    }
}

#[derive(Copy, Clone)]
pub struct LogicalXnorMonoid {
    _marker: std::marker::PhantomData<()>,
}
impl LogicalXnorMonoid {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl Monoid<bool> for LogicalXnorMonoid {
    fn identity(&self) -> bool {
        true
    }

    fn operate(&self, lhs: bool, rhs: bool) -> bool {
        LogicalXnor::new().op(lhs, rhs)
    }
}

// ---------------------------------------------------------------------------
// Semirings

pub trait Semiring<D1, D2 = D1>: Clone {
    // 2 types of output for operations that return values and operations that return references
    // e.g Addition: (&T, &T) -> T, Min: (&T, &T) -> &T
    type Output;

    fn add(&self, lhs: Self::Output, rhs: Self::Output) -> Self::Output;

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output;

    fn zero(&self) -> Self::Output;
}

#[derive(Copy, Clone)]
pub struct PlusTimesSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> PlusTimesSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MinPlusSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> MinPlusSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MaxPlusSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> MaxPlusSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MinTimesSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> MinTimesSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MinMaxSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> MinMaxSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MaxMinSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> MaxMinSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MaxTimesSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> MaxTimesSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct PlusMinSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> PlusMinSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct LogicalSemiring {
    _marker: std::marker::PhantomData<()>,
}
impl LogicalSemiring {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct AndOrSemiring {
    _marker: std::marker::PhantomData<()>,
}
impl AndOrSemiring {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct XorAndSemiring {
    _marker: std::marker::PhantomData<()>,
}
impl XorAndSemiring {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct XorOrSemiring {
    _marker: std::marker::PhantomData<()>,
}
impl XorOrSemiring {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MinFirstSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> MinFirstSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MinSecondSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> MinSecondSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MaxFirstSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> MaxFirstSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MaxSecondSemiring<D1, D2 = D1> {
    _marker: std::marker::PhantomData<(D1, D2)>,
}
impl<D1, D2> MaxSecondSemiring<D1, D2> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

// ---------------------------------------------------------------------------
// Semirings implementations

impl<D1, D2> Semiring<D1, D2> for PlusTimesSemiring<D1, D2>
where
    Self: Clone,
    PlusMonoid<D1>: Monoid<D1>,
    Multiplication<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: Self::Output, rhs: Self::Output) -> Self::Output {
        PlusMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        Multiplication::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        PlusMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for MinPlusSemiring<D1, D2>
where
    Self: Clone,
    MinMonoid<D1>: Monoid<D1>,
    Addition<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        MinMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        Addition::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        MinMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for MaxPlusSemiring<D1, D2>
where
    Self: Clone,
    MaxMonoid<D1>: Monoid<D1>,
    Addition<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        MaxMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        Addition::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        MaxMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for MinTimesSemiring<D1, D2>
where
    Self: Clone,
    MinMonoid<D1>: Monoid<D1>,
    Multiplication<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        MinMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        Multiplication::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        MinMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for MinMaxSemiring<D1, D2>
where
    Self: Clone,
    MinMonoid<D1>: Monoid<D1>,
    Maximum<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        MinMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        Maximum::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        MinMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for MaxMinSemiring<D1, D2>
where
    Self: Clone,
    MaxMonoid<D1>: Monoid<D1>,
    Minimum<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        MaxMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        Minimum::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        MaxMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for MaxTimesSemiring<D1, D2>
where
    Self: Clone,
    MaxMonoid<D1>: Monoid<D1>,
    Multiplication<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        MaxMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        Multiplication::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        MaxMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for PlusMinSemiring<D1, D2>
where
    Self: Clone,
    PlusMonoid<D1>: Monoid<D1>,
    Minimum<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        PlusMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        Minimum::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        PlusMonoid::new().identity()
    }
}

impl Semiring<bool> for LogicalSemiring
where
    Self: Clone,
    LogicalOrMonoid: Monoid<bool>,
    LogicalAnd: BinaryOperator<bool, Output = bool>,
{
    type Output = bool;

    fn add(&self, lhs: bool, rhs: bool) -> Self::Output {
        LogicalOrMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: bool, rhs: bool) -> Self::Output {
        LogicalAnd::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        LogicalOrMonoid::new().identity()
    }
}

impl Semiring<bool> for AndOrSemiring
where
    Self: Clone,
    LogicalAndMonoid: Monoid<bool>,
    LogicalOr: BinaryOperator<bool, Output = bool>,
{
    type Output = bool;

    fn add(&self, lhs: bool, rhs: bool) -> Self::Output {
        LogicalAndMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: bool, rhs: bool) -> Self::Output {
        LogicalOr::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        LogicalAndMonoid::new().identity()
    }
}

impl Semiring<bool> for XorAndSemiring
where
    Self: Clone,
    LogicalXorMonoid: Monoid<bool>,
    LogicalAnd: BinaryOperator<bool, Output = bool>,
{
    type Output = bool;

    fn add(&self, lhs: bool, rhs: bool) -> Self::Output {
        LogicalXorMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: bool, rhs: bool) -> Self::Output {
        LogicalAnd::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        LogicalXorMonoid::new().identity()
    }
}

impl Semiring<bool> for XorOrSemiring
where
    Self: Clone,
    LogicalXorMonoid: Monoid<bool>,
    LogicalOr: BinaryOperator<bool, Output = bool>,
{
    type Output = bool;

    fn add(&self, lhs: bool, rhs: bool) -> Self::Output {
        LogicalXorMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: bool, rhs: bool) -> Self::Output {
        LogicalOr::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        LogicalXorMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for MinFirstSemiring<D1, D2>
where
    Self: Clone,
    MinMonoid<D1>: Monoid<D1>,
    First<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        MinMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        First::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        MinMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for MinSecondSemiring<D1, D2>
where
    Self: Clone,
    MinMonoid<D1>: Monoid<D1>,
    Second<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        MinMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        Second::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        MinMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for MaxFirstSemiring<D1, D2>
where
    Self: Clone,
    MaxMonoid<D1>: Monoid<D1>,
    First<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        MaxMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        First::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        MaxMonoid::new().identity()
    }
}

impl<D1, D2> Semiring<D1, D2> for MaxSecondSemiring<D1, D2>
where
    Self: Clone,
    MaxMonoid<D1>: Monoid<D1>,
    Second<D1>: BinaryOperator<D1, D2, Output = D1>,
{
    type Output = D1;

    fn add(&self, lhs: D1, rhs: D1) -> Self::Output {
        MaxMonoid::new().operate(lhs, rhs)
    }

    fn mult(&self, lhs: D1, rhs: D2) -> Self::Output {
        Second::new().op(lhs, rhs)
    }

    fn zero(&self) -> Self::Output {
        MaxMonoid::new().identity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_ops() {
        let or = LogicalOr::new();
        assert!(or.op(true, true));

        let and = LogicalAnd::new();
        assert!(and.op(true, true));

        let xor = LogicalXor::new();
        assert!(!xor.op(true, true));

        let xnor = LogicalXnor::new();
        assert!(xnor.op(true, true));

        let bit_or = BitwiseOr::new();
        assert_eq!(bit_or.op(1, 1), 1);

        let bit_and = BitwiseAnd::new();
        assert_eq!(bit_and.op(1, 1), 1);

        let bit_xor = BitwiseXor::new();
        assert_eq!(bit_xor.op(3, 4), 7);

        let bit_xnor = BitwiseXnor::new();
        assert_eq!(bit_xnor.op(3, 4), -8);

        let equal = Equal::new();
        assert!(equal.op(1, 1));

        let not_equal = NotEqual::new();
        assert!(!not_equal.op(1, 1));

        let greater = GreaterThan::new();
        assert!(!greater.op(1, 1));

        let less = LessThan::new();
        assert!(!less.op(1, 1));

        let greater_equal = GreaterThanOrEqual::new();
        assert!(greater_equal.op(1, 1));

        let less_equal = LessThanOrEqual::new();
        assert!(less_equal.op(1, 1));

        let first = First::new();
        assert_eq!(first.op(1, 2), 1);

        let second = Second::new();
        assert_eq!(second.op(1, 2), 2);

        let min = Minimum::new();
        assert_eq!(min.op(1, 2), 1);

        let max = Maximum::new();
        assert_eq!(max.op(1, 2), 2);

        let add = Addition::new();
        assert_eq!(add.op(1, 1), 2);

        let sub = Subtraction::new();
        assert_eq!(sub.op(1, 1), 0);

        let mult = Multiplication::new();
        assert_eq!(mult.op(1, 1), 1);

        let div = Division::new();
        assert_eq!(div.op(1, 1), 1);
    }

    #[test]
    fn test_monoids() {
        let plus = PlusMonoid::new();
        assert_eq!(plus.operate(1, 1), 2);

        let times = TimesMonoid::new();
        assert_eq!(times.operate(1, 1), 1);

        let min = MinMonoid::new();
        assert_eq!(min.operate(1, 2), 1);

        let max = MaxMonoid::new();
        assert_eq!(max.operate(1, 2), 2);

        let or = LogicalOrMonoid::new();
        assert!(or.operate(true, true));

        let and = LogicalAndMonoid::new();
        assert!(and.operate(true, true));

        let xor = LogicalXorMonoid::new();
        assert!(!xor.operate(true, true));

        let xnor = LogicalXnorMonoid::new();
        assert!(xnor.operate(true, true));
    }

    #[test]
    fn test_semirings() {
        let plus_times = PlusTimesSemiring::new();
        assert_eq!(plus_times.add(1, 1), 2);
        assert_eq!(plus_times.mult(1, 1), 1);

        let min_plus = MinPlusSemiring::new();
        assert_eq!(min_plus.add(1, 1), 1);
        assert_eq!(min_plus.mult(1, 1), 2);

        let max_plus = MaxPlusSemiring::new();
        assert_eq!(max_plus.add(1, 1), 1);
        assert_eq!(max_plus.mult(1, 1), 2);

        let min_times = MinTimesSemiring::new();
        assert_eq!(min_times.add(1, 1), 1);
        assert_eq!(min_times.mult(1, 1), 1);

        let min_max = MinMaxSemiring::new();
        assert_eq!(min_max.add(1, 1), 1);
        assert_eq!(min_max.mult(1, 1), 1);

        let max_min = MaxMinSemiring::new();
        assert_eq!(max_min.add(1, 1), 1);
        assert_eq!(max_min.mult(1, 1), 1);

        let max_times = MaxTimesSemiring::new();
        assert_eq!(max_times.add(1, 1), 1);
        assert_eq!(max_times.mult(1, 1), 1);

        let plus_min = PlusMinSemiring::new();
        assert_eq!(plus_min.add(1, 1), 2);
        assert_eq!(plus_min.mult(1, 2), 1);

        let logical = LogicalSemiring::new();
        assert!(logical.add(true, true));
        assert!(logical.mult(true, true));

        let and_or = AndOrSemiring::new();
        assert!(and_or.add(true, true));
        assert!(and_or.mult(true, true));

        let xor_and = XorAndSemiring::new();
        assert!(!xor_and.add(true, true));
        assert!(xor_and.mult(true, true));

        let xor_or = XorOrSemiring::new();
        assert!(!xor_or.add(true, true));
        assert!(xor_or.mult(true, true));

        let min_first = MinFirstSemiring::new();
        assert_eq!(min_first.add(1, 2), 1);
        assert_eq!(min_first.mult(1, 2), 1);

        let min_second = MinSecondSemiring::new();
        assert_eq!(min_second.add(1, 2), 1);
        assert_eq!(min_second.mult(1, 2), 2);

        let max_first = MaxFirstSemiring::new();
        assert_eq!(max_first.add(1, 2), 2);
        assert_eq!(max_first.mult(1, 2), 1);

        let max_second = MaxSecondSemiring::new();
        assert_eq!(max_second.add(1, 2), 2);
        assert_eq!(max_second.mult(1, 2), 2);
    }
}
