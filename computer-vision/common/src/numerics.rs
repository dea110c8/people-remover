pub trait NumericType: Copy {
    fn zero() -> Self;
    fn one() -> Self;
}

impl NumericType for i8 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl NumericType for u8 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl NumericType for i16 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl NumericType for u16 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl NumericType for i32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl NumericType for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl NumericType for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl NumericType for u64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl NumericType for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

impl NumericType for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

pub trait ToUsize {
    fn to_usize(&self) -> usize;
}

impl ToUsize for u32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}