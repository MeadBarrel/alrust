use serde::{Serialize, Deserialize};

use grimoire2::modify::skill::SkillUpdate;

use crate::theoretical::TheoreticalWrapper;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SkillUpdateSerializable {
    effectiveness: Option<TheoreticalWrapper>,
    parent: Option<String>,
    parent_2: Option<String>,
    remove_parent: bool,
    remove_parent_2: bool,
}


impl SkillUpdateSerializable {
    pub fn to_update(&self) -> SkillUpdate {
        let mut update = SkillUpdate::default();

        if let Some(x) = self.effectiveness {
            update.set_effectiveness(x.into());
        }

        if let Some(x) = &self.parent {
            update.set_parent(x);
        }

        if let Some(x) = &self.parent_2 {
            update.set_parent2(x);
        }

        if self.remove_parent {
            update.remove_parent();
        }

        if self.remove_parent_2 {
            update.remove_parent_2();
        }

        update
    }

    pub fn from_update(update: &SkillUpdate) -> Self {
        Self {
            effectiveness: update.will_set_effectiveness().map(|x| x.into()),
            parent: match update.will_set_parent() {
                Some(Some(x)) => Some(x),
                Some(None) => None,
                None => None,
            },
            parent_2: match update.will_set_parent_2() {
                Some(Some(x)) => Some(x),
                Some(None) => None,
                None => None,
            },
            remove_parent: match update.will_set_parent() {
                Some(Some(_)) => false,
                Some(None) => true,
                None => false,
            },
            remove_parent_2: match update.will_set_parent_2() {
                Some(Some(_)) => false,
                Some(None) => true,
                None => false,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::SkillUpdateSerializable;
    use crate::theoretical::TheoreticalWrapper;
    use grimoire2::theoretical::Theoretical;
    use grimoire2::modify::skill::SkillUpdate;

    #[test]
    pub fn test_to_update_set() {
        let ser_update = SkillUpdateSerializable {
            effectiveness: Some(TheoreticalWrapper::Known(0.1)),
            parent: Some("a".to_string()),
            parent_2: Some("b".to_string()),
            remove_parent: false,
            remove_parent_2: false,
        };

        let update = ser_update.to_update();

        assert_eq!(update.will_set_effectiveness(), Some(Theoretical::Known(0.1)));
        assert_eq!(update.will_set_parent(), Some(Some("a".to_string())));
        assert_eq!(update.will_set_parent_2(), Some(Some("b".to_string())));
    }

    #[test]
    pub fn test_to_update_no_set() {
        let ser_update = SkillUpdateSerializable::default();
        let update = ser_update.to_update();

        assert!(update.will_set_effectiveness().is_none());
        assert!(update.will_set_parent().is_none());
        assert!(update.will_set_parent_2().is_none());
    }

    #[test]
    pub fn test_to_update_set_none() {
        let ser_update = SkillUpdateSerializable {
            effectiveness: None,
            parent: None,
            parent_2: None,
            remove_parent: true,
            remove_parent_2: true
        };

        let update = ser_update.to_update();

        assert_eq!(update.will_set_effectiveness(), None);
        assert_eq!(update.will_set_parent(), Some(None));
        assert_eq!(update.will_set_parent_2(), Some(None));
    }

    #[test]
    pub fn test_from_update_set() {
        let update = SkillUpdate::default()
            .set_effectiveness(Theoretical::Known(0.1))
            .set_parent("a")
            .set_parent2("b")
            .clone();
        
        let ser_update = SkillUpdateSerializable::from_update(&update);

        assert_eq!(ser_update.effectiveness, Some(TheoreticalWrapper::Known(0.1)));
        assert_eq!(ser_update.parent, Some("a".to_string()));
        assert_eq!(ser_update.parent_2, Some("b".to_string()));
        assert!(!ser_update.remove_parent);
        assert!(!ser_update.remove_parent_2);
    }

    #[test]
    pub fn test_from_update_set_none() {
        let update = SkillUpdate::default()
            .remove_parent()
            .remove_parent_2()
            .clone();
        let ser_update = SkillUpdateSerializable::from_update(&update);

        assert!(ser_update.effectiveness.is_none());
        assert!(ser_update.parent.is_none());
        assert!(ser_update.parent_2.is_none());
        assert!(ser_update.remove_parent);
        assert!(ser_update.remove_parent_2);
    }
}