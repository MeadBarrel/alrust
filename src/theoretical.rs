use serde::{Serialize, Serializer};

use grimoire2::theoretical::Theoretical;


#[derive(Debug, Clone, Copy)]
pub struct TheoreticalWrapper<T>(pub Theoretical<T>);


impl Serialize for TheoreticalWrapper<f64>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        
        match &self.0 {
            Theoretical::Known(x) => {
                serializer.serialize_newtype_variant("Theoretical", 0, "_", x)
            }
            Theoretical::Unknown(x) => {
                serializer.serialize_newtype_variant("Theoretical", 1, "Unknown", x)
            }            
        }

    }
}
