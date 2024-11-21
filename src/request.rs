use futures::stream::{ StreamExt as _ };

use scraper::{ Html, Selector };
use reqwest::Client;
use std::sync::LazyLock;

use crate::{
    config,
    trace,
    args::Args,
    route::RouteOption,
    system::{ SystemPair, SystemHolder },
};

static CLIENT: LazyLock<Client> = LazyLock::new(|| 
    Client::builder()
        .user_agent(config::USER_AGENT)
        .build()
        .unwrap()
);

static SEL_0: LazyLock<Selector> = LazyLock::new(|| Selector::parse(r#"div[id="navtools"]"#).unwrap());
static SEL_1: LazyLock<Selector> = LazyLock::new(|| Selector::parse(r#"table[class="tablelist table-tooltip"]"#).unwrap());
static SEL_2: LazyLock<Selector> = LazyLock::new(|| Selector::parse(r#"tr"#).unwrap());
static SEL_3: LazyLock<Selector> = LazyLock::new(|| Selector::parse(r#"td"#).unwrap());



pub async fn make_requests(args: &Args, system_holder: &SystemHolder) {

    let bodies = futures::stream::iter(
        system_holder.all_inter_systems_iter()
    )
        .map(|system_pair| {
            async move {
                let url = make_url(&args.route_option, &system_pair);
                let resp = CLIENT.get(url).send().await;
                
                (system_pair, resp)
            }
        })
        .buffer_unordered(args.concurrent);

    bodies
        .for_each(|(system_pair, resp)| {
            async move {
                match resp {
                    Ok(resp) => {
                        let distance = parse_text_into_length(resp.text().await.unwrap());
                        system_pair.set_distance(distance).unwrap();
                    },
                    Err(err) => {
                        eprintln!("{:#?}", err);
                    },
                };
            }
        }).await;
}

fn make_url(route_option: &RouteOption, system_pair: &SystemPair) -> String {
    format!("{}{}{}:{}{}",
        config::ROUTE_SEARCH_URL_PREFIX,
        route_option.as_url(),
        system_pair.left().read().unwrap().name(),
        system_pair.right().read().unwrap().name(),
        config::ROUTE_SEARCH_URL_POSTFIX,
    )
}

/*fn rewind_url<S: AsRef<str>>(url: S, system_holder: &SystemHolder) -> SystemPair {
    let str_systems: Vec<&str> = url
        .as_ref()
        .trim_start_matches(config::ROUTE_SEARCH_URL_PREFIX)
        .trim_end_matches(config::ROUTE_SEARCH_URL_POSTFIX)
        .split(':')
        .skip(1)/*first must be one of '1:' '2:' '3:'*/
        .collect();
    // from version 0.2.0, all URL made is for 'SystemPair'
    assert_eq!(str_systems.len(), 2, "{}", &trace::string::error(format!("failed to revert URL '{}' into 'SystemPair'", url.as_ref())));

    SystemPair::from_ref(
        system_holder.get(str_systems[0]),
        system_holder.get(str_systems[1]),
    )
}*/

fn parse_text_into_length<S>(text: S) -> u64 
where
    S: AsRef<str>,
{
    let distance: u64 = Html::parse_document(text.as_ref())
        .select(&SEL_0)
        .next()
        .expect(&trace::string::error("Unexpected response format"))
        .select(&SEL_1)
        .next()
        .expect(&trace::string::error("System Name Invalid"))
        .select(&SEL_2)
        .last()
        .unwrap()
        .select(&SEL_3)
        .next()
        .unwrap()
        .inner_html()
        .replace('.', "")
        .trim()
        .parse()
        .expect(&trace::string::error("Failed to parse route length"));

    distance - 1 // route start from self
}
