pub mod character;
pub mod skill;
pub mod ingredient;


use crate::grimoire::Grimoire;


#[derive(Debug, Clone)]
enum CharacterUpdateCommand {
    Update(String, character::CharacterUpdate),
    Remove(String),
}


#[derive(Debug, Clone)]
enum SkillUpdateCommand {
    Update(String, skill::SkillUpdate),
    Remove(String)
}


#[derive(Debug, Clone)]
enum IngredientUpdateCommand {
    Update(String, ingredient::IngredientUpdate),
    Remove(String)
}


#[derive(Debug, Clone, Default)]
pub struct GrimoireUpdate {
    characters: Vec<CharacterUpdateCommand>,
    skills: Vec<SkillUpdateCommand>,
    ingredients: Vec<IngredientUpdateCommand>,
}


impl GrimoireUpdate {
    pub fn create(&self) -> Grimoire {
        let mut result = Grimoire::default();
        self.update(&mut result);
        result
    }

    pub fn update(&self, grimoire: &mut Grimoire) {
        self.characters.iter().for_each(|action| {
            match action {
                CharacterUpdateCommand::Update(name, update) => 
                    { 
                        update.update(grimoire.characters.entry(name.clone()).or_default());
                    },
                CharacterUpdateCommand::Remove(name) => 
                    { grimoire.characters.remove(name); }
            }
        });

        self.skills.iter().for_each(|action| {
            match action {
                SkillUpdateCommand::Update(name, update) => 
                    { update.update(grimoire.skills.entry(name.clone()).or_default()); },
                SkillUpdateCommand::Remove(name) => 
                    { grimoire.skills.remove(name); }
            }
        });

        self.ingredients.iter().for_each(|action| {
            match action {
                IngredientUpdateCommand::Update(name, update) => 
                    { update.update(grimoire.ingredients.entry(name.clone()).or_default()); },
                IngredientUpdateCommand::Remove(name) => 
                    { grimoire.ingredients.remove(name); }
            }
        })
    }

    pub fn character(&mut self, name: &str, update: character::CharacterUpdate) -> &mut Self {
        self.characters.push(
            CharacterUpdateCommand::Update(name.to_string(), update)
        );
        self
    }

    pub fn skill(&mut self, name: &str, update: skill::SkillUpdate) -> &mut Self {
        self.skills.push(
            SkillUpdateCommand::Update(name.to_string(), update)
        );
        self
    }

    pub fn ingredient(&mut self, name: &str, update: ingredient::IngredientUpdate) -> &mut Self {
        self.ingredients.push(
            IngredientUpdateCommand::Update(name.to_string(), update)
        );
        self
    }

    pub fn remove_character(&mut self, name: &str) -> &mut Self {
        self.characters.push(
            CharacterUpdateCommand::Remove(name.to_string())
        );
        self
    }

    pub fn remove_skill(&mut self, name: &str) -> &mut Self {
        self.skills.push(
            SkillUpdateCommand::Remove(name.to_string())
        );
        self
    }
    pub fn remove_ingredient(&mut self, name: &str) -> &mut Self {
        self.ingredients.push(
            IngredientUpdateCommand::Remove(name.to_string())
        );
        self    }
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
            .character("ShallRemove", CharacterUpdate::default().clone())
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
            .skill("ShallRemove", SkillUpdate::default().clone())
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
            .ingredient("ShallRemove", IngredientUpdate::default().clone())
            .clone()
    }
}