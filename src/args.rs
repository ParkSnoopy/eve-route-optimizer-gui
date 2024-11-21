use crate::{
    config,
    trace,
    route::{ Route, RouteOption },
    system::System,
};



#[derive(Debug)]
pub struct Args {
    // specific separator (,:) separated system names 
    // ex: Jita,Amarr,Hek,BKG-Q2,SI-I89
    pub route: Route,

    // system to start route
    pub start: System,

    // system to end route
    pub end: Option<System>,

    // route option (one of `fastest` `highsec` `low-null`)
    pub route_option: RouteOption,

    // concurrent fetches (too high may blocked by DOTLAN)
    pub concurrent: usize,
}

impl Args {
    pub fn builder() -> ArgBuilder {
        ArgBuilder::new()
    }
}

pub struct ArgBuilder {
    route: Option<Route>,
    start: Option<System>,
    end: Option<System>,
    route_option: Option<RouteOption>,
    concurrent: Option<usize>,
}

impl ArgBuilder {
    fn new() -> Self {
        Self {
            route: None,
            start: None,
            end: None,
            route_option: None,
            concurrent: None,
        }
    }

    pub fn build(self) -> Result<Args, String> {
        if self.route.is_none() {
            return Err(trace::string::error("While building 'Args': 'route' must be set"));
        }
        if self.start.is_none() {
            return Err(trace::string::error("While building 'Args': 'start' must be set"));
        }
        let args = Args {
            route: self.route.unwrap(),
            start: self.start.unwrap(),

            end: self.end,

            route_option: self.route_option.unwrap_or(RouteOption::Fastest),
            concurrent: self.concurrent.unwrap_or(config::DEFAULT_PARAREL_REQUEST),
        };
        Ok(args)
    }

    pub fn set_route<S: AsRef<str>>(mut self, str_route: S) -> Self {
        if str_route.as_ref().len() > 0 {
            self.route = Some(Route::new(str_route));
        }
        self
    }

    pub fn set_start<S: AsRef<str>>(mut self, str_start: S) -> Self {
        if str_start.as_ref().len() > 0 {
            self.start = Some(System::new(str_start));
        }
        self
    }

    pub fn set_end<S: AsRef<str>>(mut self, str_end: S) -> Self {
        if str_end.as_ref().len() > 0 {
            self.end = Some(System::new(str_end));
        }
        self
    }

    pub fn set_route_option(mut self, route_option: &RouteOption) -> Self {
        self.route_option = Some(route_option.clone());
        self
    }

    pub fn set_concurrent(mut self, concurrent: usize) -> Self {
        self.concurrent = Some(concurrent);
        self
    }
}
