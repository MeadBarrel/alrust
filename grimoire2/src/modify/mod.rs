pub mod character;
pub mod skill;
pub mod ingredient;


use serde::{Serialize, Deserialize};

use crate::grimoire::Grimoire;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrimoireUpdateCommand {
    Character(String, character::CharacterUpdate),
    Skill(String, skill::SkillUpdate),
    Ingredient(String, ingredient::IngredientUpdate),
    RemoveCharacter(String),
    RemoveSkill(String),
    RemoveIngredient(String),
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrimoireUpdate {
    commands: Vec<GrimoireUpdateCommand>,
}


impl GrimoireUpdate {
    pub fn create(&self) -> Grimoire {
        let mut result = Grimoire::default();
        self.update(&mut result);
        result
    }

    pub fn update(&self, grimoire: &mut Grimoire) {
        for command in &self.commands {
            match command {
                GrimoireUpdateCommand::Character(name, update) => {
                    let character = grimoire.characters.entry(name.clone()).or_default();
                    update.update(character);
                }
                GrimoireUpdateCommand::Skill(name, update) => {
                    let skill = grimoire.skills.entry(name.clone()).or_default();
                    update.update(skill);
                }
                GrimoireUpdateCommand::Ingredient(name, update) => {
                    let ingredient = grimoire.ingredients.entry(name.clone()).or_default();
                    update.update(ingredient);
                },
                GrimoireUpdateCommand::RemoveCharacter(name) => {
                    grimoire.characters.remove(name);
                },
                GrimoireUpdateCommand::RemoveSkill(name) => {
                    grimoire.skills.remove(name);
                },
                GrimoireUpdateCommand::RemoveIngredient(name) => {
                    grimoire.ingredients.remove(name);
                }
            }
        }
    }

    pub fn character(&mut self, name: &str, update: character::CharacterUpdate) -> &mut Self {
        self.commands.push(GrimoireUpdateCommand::Character(name.to_string(), update));
        self
    }

    pub fn skill(&mut self, name: &str, update: skill::SkillUpdate) -> &mut Self {
        self.commands.push(GrimoireUpdateCommand::Skill(name.to_string(), update));
        self
    }

    pub fn ingredient(&mut self, name: &str, update: ingredient::IngredientUpdate) -> &mut Self {
        self.commands.push(GrimoireUpdateCommand::Ingredient(name.to_string(), update));
        self
    }

    pub fn remove_character(&mut self, name: &str) -> &mut Self {
        self.commands.push(GrimoireUpdateCommand::RemoveCharacter(name.to_string()));
        self
    }

    pub fn remove_skill(&mut self, name: &str) -> &mut Self {
        self.commands.push(GrimoireUpdateCommand::RemoveSkill(name.to_string()));
        self
    }
    pub fn remove_ingredient(&mut self, name: &str) -> &mut Self {
        self.commands.push(GrimoireUpdateCommand::RemoveIngredient(name.to_string()));
        self    
    }
}



#[cfg(test)]
mod tests {
    use super::GrimoireUpdate;
    use super::character::CharacterUpdate;
    use super::skill::SkillUpdate;
    use super::ingredient::IngredientUpdate;
    use crate::theoretical::Theoretical;
    use crate::effect::Effect;

    #[test]
    fn test_create() {
        let grimoire = grimoire_update().create();

        assert!(grimoire.characters.contains_key("Tashka"));
        assert!(grimoire.characters.contains_key("Another"));

        assert!(grimoire.skills.contains_key("a"));
        assert!(grimoire.skills.contains_key("b"));

        assert!(grimoire.ingredients.contains_key("A"));
        assert!(grimoire.ingredients.contains_key("B"));

        assert!(grimoire.characters.get("Tashka").unwrap().has_clade("A"));
        assert!(grimoire.characters.get("Another").unwrap().has_clade("B"));
        assert_eq!(grimoire.skills.get("a").unwrap().effectiveness, Theoretical::Known(0.5) );
        assert_eq!(grimoire.skills.get("b").unwrap().effectiveness, Theoretical::Theory(0.3) );
        assert_eq!(grimoire.ingredients.get("A").unwrap().modifiers[Effect::Alcohol].term, Theoretical::Known(0.5));
        assert_eq!(grimoire.ingredients.get("A").unwrap().modifiers[Effect::Alcohol].multiplier, Theoretical::Unknown);
        assert_eq!(grimoire.ingredients.get("B").unwrap().skill, Some("a".to_string()));

    }

    #[test]
    fn test_update() {
        let mut grimoire = grimoire_update().create();
        GrimoireUpdate::default()
            .character(
                "Third", CharacterUpdate::default()
                    .add_clade("C")
                    .clone()
            )
            .character(
                "Tashka", CharacterUpdate::default()
                    .remove_clade("A")
                    .clone()
            )
            .skill(
                "c", SkillUpdate::default()
                    .set_parent2("a")
                    .clone()
            )
            .skill(
                "a", SkillUpdate::default()
                    .set_effectiveness(Theoretical::Unknown)
                    .clone()
            )
            .ingredient(
                "A", IngredientUpdate::default()
                    .set_multiplier(Effect::Alcohol, Theoretical::Known(1.0))
                    .clone()
            )
            .ingredient(
                "C", IngredientUpdate::default()
                    .set_skill("d")
                    .clone()
            )
            .remove_character("ShallRemove")
            .remove_skill("ShallRemove")
            .remove_ingredient("ShallRemove")
            .update(&mut grimoire);

            assert!(grimoire.characters.contains_key("Third"));
            assert!(grimoire.characters.contains_key("Tashka"));
            assert!(grimoire.characters.contains_key("Another"));
            assert!(!grimoire.characters.contains_key("ShallRemove"));

            assert!(grimoire.skills.contains_key("a"));
            assert!(grimoire.skills.contains_key("b"));
            assert!(grimoire.skills.contains_key("c"));
            assert!(!grimoire.skills.contains_key("ShallRemove"));

            assert!(grimoire.ingredients.contains_key("A"));
            assert!(grimoire.ingredients.contains_key("B"));
            assert!(grimoire.ingredients.contains_key("C"));
            assert!(!grimoire.ingredients.contains_key("ShallRemove"));

            assert!(grimoire.characters.get("Third").unwrap().has_clade("C"), "New character was not added");
            assert!(!grimoire.characters.get("Tashka").unwrap().has_clade("A"), "Old character's clade was not removed");
            assert!(grimoire.characters.get("Tashka").unwrap().has_clade("B"), "Old character's clade was removed when it shouldnt've been");
            assert!(grimoire.characters.get("Another").is_some(), "An old character was deleted");

            assert_eq!(grimoire.skills.get("c").unwrap().parent_2, Some("a".to_string()), "New skill wasn't added");
            assert_eq!(grimoire.skills.get("a").unwrap().effectiveness, Theoretical::Unknown, "Old skill wasn't modified");
            assert_eq!(grimoire.skills.get("a").unwrap().parent, Some("b".to_string()), "Old skill was recreated");
            assert!(grimoire.skills.get("b").is_some(), "Old skill was removed");

            assert_eq!(grimoire.ingredients.get("C").unwrap().skill, Some("d".to_string()), "New ingredient wasn't added properly");
            assert_eq!(grimoire.ingredients.get("A").unwrap().modifiers[Effect::Alcohol].multiplier, Theoretical::Known(1.0), "Old ingredient wasn't updated");
            assert_eq!(grimoire.ingredients.get("A").unwrap().modifiers[Effect::Alcohol].term, Theoretical::Known(0.5), "Old ingredient was recreated");
            assert!(grimoire.ingredients.get("B").is_some(), "Old ingredient was deleted");

            assert!(grimoire.characters.get("ShallRemove").is_none(), "Character not deleted");
            assert!(grimoire.skills.get("ShallRemove").is_none(), "Skill not deleted");
            assert!(grimoire.ingredients.get("ShallRemove").is_none(), "Ingredient not deleted");

    }

    fn grimoire_update() -> GrimoireUpdate {
        GrimoireUpdate::default()
            .character(
                "Tashka", CharacterUpdate::default()
                    .add_clade("A")
                    .add_clade("B")
                    .set_skill("a", 50)
                    .set_skill("b", 100)
                    .clone()
            )
            .character(
                "Another", CharacterUpdate::default()
                    .add_clade("B")
                    .clone()
            )
            .character("ShallRemove", CharacterUpdate::default())
            .skill(
                "a", SkillUpdate::default()
                    .set_effectiveness(Theoretical::Known(0.5))
                    .set_parent("b")
                    .clone()
            )
            .skill(
                "b", SkillUpdate::default()
                    .set_effectiveness(Theoretical::Theory(0.3))
                    .clone()
            )
            .skill("ShallRemove", SkillUpdate::default())
            .ingredient(
                "A", IngredientUpdate::default()
                    .set_term(Effect::Alcohol, Theoretical::Known(0.5))
                    .clone()
            )
            .ingredient(
                "B", IngredientUpdate::default()
                    .set_skill("a")
                    .clone()
            )
            .ingredient("ShallRemove", IngredientUpdate::default())
            .clone()
    }
}


pub mod versioned {
    use serde::{Serialize, Deserialize};

    use super::{GrimoireUpdate, GrimoireUpdateCommand};
    use super::character::versioned::CharacterUpdateVersioned;
    use super::skill::versioned::SkillUpdateVersioned;
    use super::ingredient::versioned::IngredientUpdateVersioned;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum GrimoireUpdateVersioned {
        #[serde(rename="0")]
        V0(v0::GrimoireUpdateV0)
    }

    impl From<GrimoireUpdate> for GrimoireUpdateVersioned {
        fn from(value: GrimoireUpdate) -> Self {
            Self::V0(value.into())
        }
    }

    impl From<GrimoireUpdateVersioned> for GrimoireUpdate {
        fn from(value: GrimoireUpdateVersioned) -> Self {
            match value {
                GrimoireUpdateVersioned::V0(x) => x.into()
            }
        }
    }

    pub mod v0 {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GrimoireUpdateV0 {
            commands: Vec<GrimoireUpdateCommandV0>
        }

        impl From<GrimoireUpdate> for GrimoireUpdateV0 {
            fn from(value: GrimoireUpdate) -> Self {
                Self {
                    commands: value.commands.into_iter().map(|x| x.into()).collect(),
                }
            }
        }

        impl From<GrimoireUpdateV0> for GrimoireUpdate {
            fn from(value: GrimoireUpdateV0) -> Self {
                Self {
                    commands: value.commands.into_iter().map(|x| x.into()).collect(),
                }                
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum GrimoireUpdateCommandV0 {
            Character(String, CharacterUpdateVersioned),
            Skill(String, SkillUpdateVersioned),
            Ingredient(String, IngredientUpdateVersioned),
            RemoveCharacter(String),
            RemoveSkill(String),
            RemoveIngredient(String),                   
        }

        impl From<GrimoireUpdateCommand> for GrimoireUpdateCommandV0 {
            fn from(value: GrimoireUpdateCommand) -> Self {
                match value {
                    GrimoireUpdateCommand::Character(n, c) => 
                        GrimoireUpdateCommandV0::Character(n, c.into()),
                    GrimoireUpdateCommand::Skill(n, c) => 
                        GrimoireUpdateCommandV0::Skill(n, c.into()),
                    GrimoireUpdateCommand::Ingredient(n, c) => 
                        GrimoireUpdateCommandV0::Ingredient(n, c.into()),
                    GrimoireUpdateCommand::RemoveCharacter(n) =>
                        GrimoireUpdateCommandV0::RemoveCharacter(n),
                    GrimoireUpdateCommand::RemoveSkill(n) =>
                        GrimoireUpdateCommandV0::RemoveSkill(n),
                    GrimoireUpdateCommand::RemoveIngredient(n) =>
                        GrimoireUpdateCommandV0::RemoveIngredient(n),
                }
            }
        }

        impl From<GrimoireUpdateCommandV0> for GrimoireUpdateCommand {
            fn from(value: GrimoireUpdateCommandV0) -> Self {
                match value {
                    GrimoireUpdateCommandV0::Character(n, c) => 
                        GrimoireUpdateCommand::Character(n, c.into()),
                    GrimoireUpdateCommandV0::Skill(n, c) => 
                        GrimoireUpdateCommand::Skill(n, c.into()),
                    GrimoireUpdateCommandV0::Ingredient(n, c) => 
                        GrimoireUpdateCommand::Ingredient(n, c.into()),
                    GrimoireUpdateCommandV0::RemoveCharacter(n) =>
                        GrimoireUpdateCommand::RemoveCharacter(n),
                    GrimoireUpdateCommandV0::RemoveSkill(n) =>
                        GrimoireUpdateCommand::RemoveSkill(n),
                    GrimoireUpdateCommandV0::RemoveIngredient(n) =>
                        GrimoireUpdateCommand::RemoveIngredient(n),
                }               
            }
        }
    }
}