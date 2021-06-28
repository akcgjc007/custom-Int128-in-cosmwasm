use std::ops::{AddAssign, SubAssign};

pub struct Int128 {
    magnitude: u128,
    sign: bool,
}

impl Int128 {
    pub const fn as_i128(&self) -> i128 {
        self.magnitude as i128 * if self.sign { -1 } else { 1 }
    }
    pub const fn zero() -> Self {
        Int128 {
            magnitude: 0,
            sign: false,
        }
    }
}
impl AddAssign for Int128 {
    fn add_assign(&mut self, other: Self) {
        if self.sign == other.sign {
            *self = Self {
                magnitude: self.magnitude + other.magnitude,
                sign: self.sign,
            }
        } else {
            *self = Self {
                magnitude: self.magnitude - other.magnitude,
                sign: if self.sign {
                    if other.magnitude >= self.magnitude {
                        false
                    } else {
                        true
                    }
                } else {
                    if self.magnitude >= other.magnitude {
                        false
                    } else {
                        true
                    }
                },
            }
        }
    }
}

impl SubAssign for Int128 {
    fn sub_assign(&mut self, mut other: Self) {
        other.sign = !other.sign;
        self.add_assign(other);
    }
}

impl From<i128> for Int128 {
    fn from(val: i128) -> Self {
        Int128 {
            magnitude: val as u128,
            sign: if val < 0 { true } else { false },
        }
    }
}
