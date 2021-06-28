use std::ops::{AddAssign, SubAssign};

pub struct Int128 {
    mag: u128,
    sin: bool,
}

impl Int128 {
    pub const fn as_i128(&self) -> i128 {
        self.mag as i128 * if self.sin { -1 } else { 1 }
    }
    pub const fn zero() -> Self {
        Int128 { mag: 0, sin: false }
    }
}
impl AddAssign for Int128 {
    fn add_assign(&mut self, other: Self) {
        if self.sin == other.sin {
            *self = Self {
                mag: self.mag + other.mag,
                sin: self.sin,
            }
        } else {
            *self = Self {
                mag: if self.mag >= other.mag {
                    self.mag - other.mag
                } else {
                    other.mag - self.mag
                },
                sin: if self.sin {
                    if other.mag >= self.mag {
                        false
                    } else {
                        true
                    }
                } else {
                    if self.mag >= other.mag {
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
        other.sin = !other.sin;
        self.add_assign(other);
    }
}

impl From<i128> for Int128 {
    fn from(val: i128) -> Self {
        Int128 {
            mag: val as u128,
            sin: if val < 0 { true } else { false },
        }
    }
}
