use std::sync::{ Arc, RwLock };

type ArcRwLock<T> = Arc<RwLock<T>>;
type SyncSystem = ArcRwLock<System>;
type SyncRoute = Vec<SyncSystem>;

mod system;
mod pair;
mod holder;

mod shortest;

pub use shortest::CurrentShortest;
pub use system::System;
pub use pair::SystemPair;
pub use holder::SystemHolder;
