use super::{
    SyncRoute,
};



#[derive(Debug)]
pub struct CurrentShortest {
    routes: Vec<SyncRoute>,
    length: u64,
}

impl CurrentShortest {
    pub fn new() -> CurrentShortest {
        CurrentShortest {
            routes: Vec::new(),
            length: u64::MAX,
        }
    }

    pub fn inner(&self) -> (u64, Vec<SyncRoute>) {
        (self.length, self.routes.clone())
    }

    pub fn register(&mut self, sync_route: &SyncRoute, length: u64) {
        if length < self.length {
            self.routes.clear();
            self.routes.push(sync_route.clone());
            self.length = length;
        } else if length == self.length {
            self.routes.push(sync_route.clone());
        }
    }
}
