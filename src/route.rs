use derive_more::IntoIterator;

use std::collections::HashSet;

use crate::{
    config,
    trace,
    system::System,
};



#[derive(Clone, IntoIterator)]
#[derive(Debug)]
pub struct Route {
    #[into_iterator(owned, ref, ref_mut)]
    inner: HashSet<System>,
}

#[derive(Clone, PartialEq)]
#[derive(Debug)]
pub enum RouteOption {
    Fastest,
    Highsec,
    LowNull,
}

impl Route {
    pub fn new<S: AsRef<str>>(str_route: S) -> Route {
        let mut route = HashSet::new();
        for system in str_route.as_ref().split(config::ROUTE_SPLIT_CHAR).map(System::new) {
            route.insert(system);
        }

        Route {
            inner: route
        }
    }
}

impl std::str::FromStr for Route {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Route::new(s))
    }
}

impl RouteOption {
    pub fn new<S: AsRef<str>>(str_route_option: S) -> eyre::Result<RouteOption> {
        match str_route_option.as_ref() {
            "fastest"  => Ok(RouteOption::Fastest),
            "highsec"  => Ok(RouteOption::Highsec),
            "low-null" => Ok(RouteOption::LowNull),
            _ => Err(eyre::eyre!(trace::string::error(format!("given string '{}' cannot parsed into 'RouteOption'", str_route_option.as_ref())))),
        }
    }

    pub fn as_url(&self) -> &str {
        match self {
            RouteOption::Fastest => "1:",
            RouteOption::Highsec => "2:",
            RouteOption::LowNull => "3:",
        }
    }
}

impl std::default::Default for RouteOption {
    fn default() -> Self {
        RouteOption::Fastest
    }
}

impl std::fmt::Display for RouteOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", match self {
            RouteOption::Fastest => "Fastest Route",
            RouteOption::Highsec => "Highsec Route",
            RouteOption::LowNull => "Lowsec and Nullsec Route",
        })
    }
}
