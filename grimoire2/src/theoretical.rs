use std::ops::{Add, Mul, Sub};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theoretical<T> {
    Known(T),
    Theory(T),
    Unknown,
}


pub use Theoretical::*;


impl<T> Theoretical<T>
where
    T: Copy,
{
    pub fn to_known(self) -> Theoretical<T> 
        where T: Default
    {
        Self::Known(self.inner())
    }

    pub fn to_unknown(self) -> Theoretical<T> 
        where T: Default
    {
        Self::Theory(self.inner())
    }

    pub fn default_theory(self, default: T) -> Self {
        match self {
            Self::Known(_) | Self::Theory(_) => self,
            Self::Unknown => Self::Theory(default)
        }
    }

    #[inline(always)]
    pub fn inner(&self) -> T 
        where T: Default
    {
        match self {
            Self::Known(x) => *x,
            Self::Theory(x) => *x,
            Self::Unknown => T::default(),
        }
    }

    pub fn is_known(&self) -> bool {
        match self {
            Self::Known(_) => true,
            Self::Theory(_) => false,
            Self::Unknown => false,
        }
    }

    pub fn is_theory(&self) -> bool {
        match self {
            Self::Known(_) => false,
            Self::Theory(_) => true,
            Self::Unknown => false
        }
    }

    pub fn is_unknown(&self) -> bool {
        match self {
            Self::Known(_) => false,
            Self::Theory(_) => false,
            Self::Unknown => true
        }
    }

    pub fn known_or(&self, or_: impl Fn(T) -> T) -> T 
        where T: Default
    {
        match self {
            Self::Known(x) => *x,
            Self::Theory(x) => or_(*x),
            Self::Unknown => or_(T::default())
        }
    }
}

impl<T> From<Theoretical<T>> for Option<T> {
    fn from(src: Theoretical<T>) -> Self {
        match src {
            Theoretical::Known(x) => Some(x),
            Theoretical::Theory(_) => None,
            Theoretical::Unknown => None,
        }
    }
}

impl<T> Default for Theoretical<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::Unknown
    }
}

impl<T> Add for Theoretical<T>
where
    T: Add<Output = T> + Copy + Default,
{
    type Output = Theoretical<T>;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Known(x) => match rhs {
                Self::Known(y) => Self::Known(x + y),
                Self::Theory(y) => Self::Theory(x + y),
                Self::Unknown => Self::Theory(x + T::default())
            },
            Self::Theory(x) => Self::Theory(x + rhs.inner()),
            Self::Unknown => match rhs {
                Self::Known(x) | Self::Theory(x) => Self::Theory(T::default() + x),
                Self::Unknown => Self::Unknown
            }
        }
    }
}

impl<T> Sub for Theoretical<T>
where
    T: Sub<Output = T> + Copy + Default,
{
    type Output = Theoretical<T>;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Known(x) => match rhs {
                Self::Known(y) => Self::Known(x - y),
                Self::Theory(y) => Self::Theory(x - y),
                Self::Unknown => Self::Theory(x - T::default())
            },
            Self::Theory(x) => Self::Theory(x - rhs.inner()),
            Self::Unknown => match rhs {
                Self::Known(x) | Self::Theory(x) => Self::Theory(T::default() - x),
                Self::Unknown => Self::Unknown
            }

        }
    }
}

impl<T> Mul for Theoretical<T>
where
    T: Mul<Output = T> + Copy + Default,
{
    type Output = Theoretical<T>;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Known(x) => match rhs {
                Self::Known(y) => Self::Known(x * y),
                Self::Theory(y) => Self::Theory(x * y),
                Self::Unknown => Self::Theory(x * T::default())
            },
            Self::Theory(x) => Self::Theory(x * rhs.inner()),
            Self::Unknown => match rhs {
                Self::Known(x) | Self::Theory(x) => Self::Theory(T::default() * x),
                Self::Unknown => Self::Unknown
            }

        }
    }
}

impl<T> From<Option<T>> for Theoretical<T>
where
    T: Default,
{
    #[inline(always)]
    fn from(src: Option<T>) -> Self {
        match src {
            Some(x) => Self::Known(x),
            None => Self::Theory(T::default()),
        }
    }
}

impl<T> From<T> for Theoretical<T> {
    #[inline(always)]
    fn from(x: T) -> Self {
        Self::Known(x)
    }
}



pub mod versioned {
    use serde::{Serialize, Deserialize};

    use super::Theoretical;
    

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum TheoreticalVersioned<T> {
        #[serde(rename="0")]
        V0(Theoretical<T>)
    }

    impl<T> From<Theoretical<T>> for TheoreticalVersioned<T> {
        fn from(value: Theoretical<T>) -> Self {
            TheoreticalVersioned::V0(value)
        }
    }

    impl<T> From<TheoreticalVersioned<T>> for Theoretical<T> {
        fn from(value: TheoreticalVersioned<T>) -> Self {
            match value {
                TheoreticalVersioned::V0(x) => x
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use proptest::strategy::Strategy;
    use proptest::sample::select;
    use super::*;
    
    pub fn theoretical_f64_strategy() -> impl Strategy<Value=Theoretical<f64>> {
        select(vec![
            Theoretical::Known(0.),
            Theoretical::Known(0.5),
            Theoretical::Known(1.0),
        ])
    }
}