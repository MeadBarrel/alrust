use serde::Serialize;
use std::ops::{Add, Sub, Mul};


#[derive(Serialize, Clone, Debug, Copy)]
pub enum Theoretical {
    Known(f64),
    Unknown(f64),
}


impl Theoretical {
    #[inline(always)]
    pub fn inner(&self) -> f64 {
        match self {
            Self::Known(x) => *x,
            Self::Unknown(x) => *x,
        }
    }

    pub fn is_known(&self) -> bool {
        match self {
            Self::Known(_) => true,
            Self::Unknown(_) => false
        }
    }

    pub fn known_or(&self, or_: impl Fn(f64) -> f64) -> f64 {
        match self {
            Self::Known(x) => *x,
            Self::Unknown(x) => or_(*x)
        }
    }
}


impl Default for Theoretical {
    fn default() -> Self {
        Self::Unknown(0.)
    }
}


impl Add for Theoretical {
    type Output = Theoretical;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Known(x) => {
                match rhs {
                    Self::Known(y) => Self::Known(x + y),
                    Self::Unknown(y) => Self::Unknown(x + y),
                }
            }
            Self::Unknown(x) => Self::Unknown(x + rhs.inner())
        }
    }
}


impl Sub for Theoretical {
    type Output = Theoretical;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Known(x) => {
                match rhs {
                    Self::Known(y) => Self::Known(x-y),
                    Self::Unknown(y) => Self::Unknown(x-y)
                }
            }
            Self::Unknown(x) => Self::Unknown(x - rhs.inner())
        }
    }
}


impl Mul for Theoretical {
    type Output = Theoretical;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Known(x) => {
                match rhs {
                    Self::Known(y) => Self::Known(x*y),
                    Self::Unknown(y) => Self::Unknown(x*y)
                }
            }
            Self::Unknown(x) => Self::Unknown(x * rhs.inner())
        }
    }    
}


impl From<Option<f64>> for Theoretical {
    #[inline(always)]
    fn from(src: Option<f64>) -> Self {
        match src {
            Some(x) => Self::Known(x),
            None => Self::Unknown(0.)
        }
    }
}


impl From<f64> for Theoretical {
    #[inline(always)]
    fn from(x: f64) -> Self {
        Self::Known(x)
    }
}
