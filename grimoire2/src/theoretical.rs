use std::ops::{Add, Sub, Mul};


#[derive(Clone, Debug, Copy)]
pub enum Theoretical<T> {
    Known(T),
    Unknown(T),
}


impl<T> Theoretical<T> 
    where T: Copy
{
    #[inline(always)]
    pub fn inner(&self) -> T {
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

    pub fn known_or(&self, or_: impl Fn(T) -> T) -> T {
        match self {
            Self::Known(x) => *x,
            Self::Unknown(x) => or_(*x)
        }
    }
}


impl<T> From<Theoretical<T>> for Option<T> {
    fn from(src: Theoretical<T>) -> Self {
        match src {
            Theoretical::Known(x) => Some(x),
            Theoretical::Unknown(_) => None,
        }
    }
}



impl<T> Default for Theoretical<T> 
    where T: Default
{
    fn default() -> Self {
        Self::Unknown(T::default())
    }
}


impl<T> Add for Theoretical<T> 
    where T: Add<Output=T> + Copy
{
    type Output = Theoretical<T>;

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


impl<T> Sub for Theoretical<T> 
    where T: Sub<Output=T> + Copy
{
    type Output = Theoretical<T>;

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


impl<T> Mul for Theoretical<T> 
    where T: Mul<Output=T> + Copy
{
    type Output = Theoretical<T>;

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


impl<T> From<Option<T>> for Theoretical<T> 
    where T: Default
{
    #[inline(always)]
    fn from(src: Option<T>) -> Self {
        match src {
            Some(x) => Self::Known(x),
            None => Self::Unknown(T::default())
        }
    }
}


impl<T> From<T> for Theoretical<T> {
    #[inline(always)]
    fn from(x: T) -> Self {
        Self::Known(x)
    }
}
