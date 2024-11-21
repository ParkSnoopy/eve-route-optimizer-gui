use derive_more::IntoIterator;
use factorial::Factorial;
use itertools::Itertools;
use rayon::prelude::*;

use std::collections::{ HashMap };
use std::sync::{ Arc, RwLock };

use crate::{
    args::Args,
    route::Route,
    system::ArcRwLock,
    progress::ProgressHolder,
};
use super::{
    SyncSystem,
    System,
    SystemPair,
    CurrentShortest,
};



#[derive(IntoIterator)]
pub struct SystemHolder {
    #[into_iterator(owned, ref, ref_mut)]
    inner: HashMap<String, SyncSystem>,
}

impl SystemHolder {
    pub fn new() -> SystemHolder {
        SystemHolder {
            inner: HashMap::new(),
        }
    }

    pub fn get<S: AsRef<str>>(&self, system_name: S) -> &SyncSystem {
        self.inner.get(system_name.as_ref()).unwrap()
    }

    pub fn all_inter_systems_iter(&self) -> impl Iterator<Item=SystemPair> {
        self.inner.clone().into_values().combinations(2).map(SystemPair::from_vec)
    }

    pub fn register_system(&mut self, system: &System) {
        self.inner.insert(
            system.name().to_string(), 
            Arc::new(RwLock::new(system.clone()))
        );
    }

    pub fn register_route(&mut self, route: &Route) {
        for system in route {
            self.register_system(system);
        }
    }
}

impl SystemHolder {
    pub fn permutation_size_hint(&self) -> Option<u128> {
        ((self.inner.len()-1) as u128).checked_factorial()
    }

    pub fn build_shortest_path(&self, cli_args: &Args, progress_holder: &RwLock<ProgressHolder>, feedback_step: usize) -> ArcRwLock<CurrentShortest> {
        let system_from: &SyncSystem = self.get(
            cli_args.start.name()
        );
        let system_to: Option<&SyncSystem> = match &cli_args.end {
            Some(system) => Some(self.get(system.name())),
            None => None,
        };

        let mut systems: Vec<SyncSystem> = self.inner.clone().into_values().collect();

        let system_from_index = systems
            .iter()
            .position(|ss| ss.read().unwrap().name() == system_from.read().unwrap().name())
            .unwrap();
        systems.remove(system_from_index);

        match system_to {
            Some(system_to) => {
                if system_to.read().unwrap().name() != system_from.read().unwrap().name() {
                    let system_from_index = systems
                        .iter()
                        .position(|ss| ss.read().unwrap().name() == system_to.read().unwrap().name())
                        .unwrap();
                    systems.remove(system_from_index);
                }
            },
            None => {},
        };

        let current_shortest: ArcRwLock<CurrentShortest> = Arc::new(RwLock::new(CurrentShortest::new()));

        systems.clone().into_iter().permutations(systems.len()).enumerate().par_bridge().for_each(|(idx, sync_route)| {
            if idx.wrapping_rem(feedback_step) == 0 {
                progress_holder.write().unwrap().feedback(idx as u128);
            }

            let system_from_rlock = system_from.read().unwrap();
            let mut route_length: u64 = /*match*/ system_from_rlock.get_distance_to(&sync_route[0]).unwrap(); /*{
                Some(step) => step,
                None => {
                    progress_holder.write().unwrap().feedback_on_err(
                        format!("Distance from '{}' to '{}' not set",
                            system_from_rlock.name(),
                            sync_route[0].read().unwrap().name(),
                        )
                    );
                    config::ROUTE_LENGTH_STEP_ON_ERR
                },
            };*/
            match &cli_args.end {
                Some(system) => {
                    route_length += sync_route[sync_route.len()-1]
                        .read().unwrap()
                        .get_distance_to(
                            self.get(system.name())
                        ).unwrap();
                },
                None => {},
            }

            sync_route.windows(2).for_each(
                |window| {
                    let prev_rlock = window[0].read().unwrap();

                    let length_step: u64 = /*match*/ prev_rlock.get_distance_to(&window[1]).unwrap(); /*{
                        Some(step) => step,
                        None => {
                            progress_holder.write().unwrap().feedback_on_err(
                                format!("Distance from '{}' to '{}' not set",
                                    prev_rlock.name(),
                                    window[1].read().unwrap().name(),
                                )
                            );

                            config::ROUTE_LENGTH_STEP_ON_ERR
                        },
                    };*/

                    route_length += length_step;
                }
            );

            current_shortest.write().unwrap().register(&sync_route, route_length);
        });

        // last report on 100%
        progress_holder.write().unwrap().feedback(self.permutation_size_hint().unwrap_or(u128::MAX));

        current_shortest
    }
}
