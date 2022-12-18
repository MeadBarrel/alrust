use serde::{Serialize, Deserialize};

use crate::theoretical::TheoreticalWrapper;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct IngredientUpdateSerializable {
    skill: Option<String>,
    remove_skill: bool,
    weight: bool,

    dh: TheoreticalWrapper,
    dp: TheoreticalWrapper,
    mdh: TheoreticalWrapper,
    mdp: TheoreticalWrapper,
    hot: TheoreticalWrapper,
    pot: TheoreticalWrapper,
    hl: TheoreticalWrapper,
    pl: TheoreticalWrapper,
    a: TheoreticalWrapper,
}
