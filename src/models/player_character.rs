use super::{player_character_lore::PlayerCharacterLore, Conn};
use crate::schema::*;
use diesel::{associations::HasTable, prelude::*};
use grimoire2::prelude as g2;

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name=player_characters, primary_key(name))]
pub struct PlayerCharacter {
    pub name: String,
    pub advanced_potion_making: i32,
    pub alvarin_clade: bool,
}

impl PlayerCharacter {
    pub fn load(conn: &mut Conn) -> QueryResult<Vec<PlayerCharacter>> {
        PlayerCharacter::table().load(conn)
    }

    pub fn from_grimoire(name: &str, src: &g2::Character) -> Self {
        Self {
            name: name.to_string(),
            advanced_potion_making: src
                .skills
                .get("Advanced Potion Making")
                .cloned()
                .unwrap_or_default() as i32,
            alvarin_clade: src.clades.contains("Alchemist"),
        }
    }

    pub fn lores_from_grimoire(name: &str, src: &g2::Character) -> Vec<PlayerCharacterLore> {
        src.skills
            .iter()
            .map(|(skill_name, &value)| PlayerCharacterLore {
                character: name.to_string(),
                lore: skill_name.clone(),
                value: value.into(),
            })
            .collect()
    }

    pub fn to_grimoire(&self, lores: &[PlayerCharacterLore]) -> (String, g2::Character) {
        let clades = match self.alvarin_clade {
            true => vec!["Alchemist".to_string()],
            false => vec![],
        }
        .into_iter()
        .collect();
        let character = g2::Character {
            clades,
            skills: lores
                .iter()
                .filter(|x| x.character == self.name)
                .map(|x| (x.lore.clone(), x.value as u8))
                .collect(),
        };
        (self.name.clone(), character)
    }
}