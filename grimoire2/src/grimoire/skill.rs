use crate::theoretical::Theoretical;

#[derive(Debug, Clone)]
pub struct Skill {
    pub effectiveness: Theoretical<f64>,
    pub parent: Option<String>,
    pub parent_2: Option<String>,
}


impl Skill {
    pub fn new(
        effectiveness: Theoretical<f64>, 
        parent: Option<String>, 
        parent_2: Option<String>
    ) -> Self {
        Self {
            effectiveness, parent, parent_2
        }    
    }
}


impl Default for Skill {
    fn default() -> Self {
        Self {
            effectiveness: Theoretical::Unknown(0.66666),
            parent: None,
            parent_2: None,
        }
    }
}