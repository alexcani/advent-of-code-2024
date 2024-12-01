use std::fmt::{Display, Formatter, Result};
use std::time::Instant;

pub struct Context {
    input: Vec<String>,
    sol1: Option<Solution>,
    sol2: Option<Solution>,
    base_time: Instant,
    sol1_time_ms: f64,
    sol2_time_ms: f64,
}

impl Context {
    pub fn new(input: Vec<String>) -> Self {
        Self {
            input,
            sol1: None,
            sol2: None,
            base_time: Instant::now(),
            sol1_time_ms: 0.0,
            sol2_time_ms: 0.0,
        }
    }

    pub fn start_timer(&mut self) {
        self.base_time = Instant::now();
    }

    pub fn input(&self) -> &[String] {
        &self.input
    }

    pub fn set_sol1<T: Into<Solution>>(&mut self, value: T) {
        self.sol1 = Some(value.into());
        self.sol1_time_ms = self.base_time.elapsed().as_nanos() as f64 / 1_000_000.0;
    }

    pub fn set_sol2<T: Into<Solution>>(&mut self, value: T) {
        self.sol2 = Some(value.into());
        self.sol2_time_ms = self.base_time.elapsed().as_nanos() as f64 / 1_000_000.0;
    }

    pub fn sol1(&self) -> Option<(&Solution, f64)> {
        self.sol1.as_ref().map(|s| (s, self.sol1_time_ms))
    }

    pub fn sol2(&self) -> Option<(&Solution, f64)> {
        self.sol2
            .as_ref()
            .map(|s| (s, self.sol2_time_ms - self.sol1_time_ms))
    }
}

#[derive(Clone)]
pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Solution::*;
        match self {
            I8(x) => x.fmt(f),
            I16(x) => x.fmt(f),
            I32(x) => x.fmt(f),
            I64(x) => x.fmt(f),
            I128(x) => x.fmt(f),
            Isize(x) => x.fmt(f),
            U8(x) => x.fmt(f),
            U16(x) => x.fmt(f),
            U32(x) => x.fmt(f),
            U64(x) => x.fmt(f),
            U128(x) => x.fmt(f),
            Usize(x) => x.fmt(f),
            Str(x) => x.fmt(f),
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    };
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, Isize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, Usize);
impl_from!(String, Str);

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}
