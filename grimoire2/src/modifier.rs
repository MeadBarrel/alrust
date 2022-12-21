use serde::{Serialize, Deserialize};

use crate::theoretical::Theoretical;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Modifier {
    pub term: Theoretical<f64>,
    pub multiplier: Theoretical<f64>,
}

impl Modifier {
    pub fn new(term: Theoretical<f64>, multiplier: Theoretical<f64>) -> Self {
        Self { term, multiplier }
    }

    pub fn new_known(term: f64, multiplier: f64) -> Self {
        Self::new(Theoretical::Known(term), Theoretical::Known(multiplier))
    }
}

impl From<(Option<f64>, Option<f64>)> for Modifier {
    fn from((term, multiplier): (Option<f64>, Option<f64>)) -> Self {
        Self {
            term: term.into(),
            multiplier: multiplier.into(),
        }
    }
}


pub mod versioned {
    use serde::{Serialize, Deserialize};

    use super::Modifier;
    use crate::theoretical::versioned::TheoreticalVersioned;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ModifierV0 {
        term: TheoreticalVersioned<f64>,
        multiplier: TheoreticalVersioned<f64>
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ModifierVersioned {
        #[serde(rename="0")]
        V0(ModifierV0)
    }

    impl From<Modifier> for ModifierV0 {
        fn from(value: Modifier) -> Self {
            Self { term: value.term.into(), multiplier: value.multiplier.into() }
        }
    }

    impl From<ModifierV0> for Modifier {
        fn from(value: ModifierV0) -> Self {
            Self { term: value.term.into(), multiplier: value.multiplier.into() }
        }
    }

    impl From<Modifier> for ModifierVersioned {
        fn from(value: Modifier) -> Self {
            Self::V0(value.into())
        }
    }

    impl From<ModifierVersioned> for Modifier {
        fn from(value: ModifierVersioned) -> Self {
            match value {
                ModifierVersioned::V0(x) => x.into()
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use proptest::strategy::Strategy;
    use crate::theoretical::tests::theoretical_f64_strategy;
    use super::*;
    
    pub fn modifier_strategy() -> impl Strategy<Value=Modifier> {
        (
            theoretical_f64_strategy(),
            theoretical_f64_strategy(),
        ).prop_map(|(term, multiplier)| Modifier { term, multiplier })
    }
}