use crate::theoretical::Theoretical;

#[derive(Default, Debug, Clone)]
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
