use std::collections::{ HashMap };

use super::{
    SyncSystem,
};



#[derive(Clone)]
#[derive(Debug)]
pub struct System {
    name: String,
    distance_table: HashMap<String, u64>,
}

impl System {
    pub fn new<S: AsRef<str>>(name: S) -> System {
        System {
            name: name.as_ref().to_uppercase(),
            distance_table: HashMap::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_distance_to(&mut self, other: &SyncSystem, distance: u64) -> eyre::Result<()> {
        match self.get_distance_to(other) {
            None => {
                self.distance_table.insert(other.read().unwrap().name().to_string(), distance);
                Ok(())
            },
            Some(_) => Err(eyre::eyre!("distance already set")),
        }
    }

    pub fn get_distance_to(&self, other: &SyncSystem) -> Option<u64> {
        self.distance_table.get(other.read().unwrap().name()).copied()
    }
}

impl std::str::FromStr for System {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(System::new(s))
    }
}

impl PartialEq for System {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for System {}

impl std::hash::Hash for System {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
