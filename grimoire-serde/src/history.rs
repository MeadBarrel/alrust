use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use grimoire2::grimoire::Grimoire;

use crate::modify::GrimoireUpdateSerializable;


const VERSION: u64 = 0;
const SUB_VERSION: u64 = 0;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    created: DateTime<Utc>,    
    content: GrimoireUpdateSerializable,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History {
    version: u64,
    sub_version: u64,
    data_version: u64,
    records: Vec<HistoryRecord>
}


impl History {
    pub fn merge(&self) -> Self {
        let grimoire = self.to_grimoire();
        Self::from_grimoire(&grimoire, self.data_version)
    }

    pub fn combine(&mut self, other: &Self) -> bool {
        if self.data_version != other.data_version - 1 { return false };

        self.data_version = other.data_version;
        self.records.extend(other.records.clone());
        true
    }

    pub fn add(&mut self, content: GrimoireUpdateSerializable) {
        self.records.push(HistoryRecord::new(content))
    }

    pub fn to_grimoire(&self) -> Grimoire {
        let mut grimoire = Grimoire::default();

        self.records.iter().for_each(|x| x.content.to_update().update(&mut grimoire));

        grimoire
    }

    pub fn from_grimoire(grimoire: &Grimoire, data_version: u64) -> Self {
        let content = GrimoireUpdateSerializable::from_grimoire(grimoire);
        Self {
            version: VERSION,
            sub_version: SUB_VERSION,
            data_version,
            records: vec! [              
                HistoryRecord::new(content)
            ]
        }
    }

}


impl HistoryRecord {
    pub fn new(content: GrimoireUpdateSerializable) -> Self {
        Self {
            created: Utc::now(),
            content
        }
    }
}