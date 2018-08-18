extern crate reqwest;
extern crate futures;

use reqwest::Client;
use reqwest::Response;
use reqwest::Error;
use futures::Future;


pub struct SpaceXAPI {
    settings: Settings
}
struct Settings {
    verson: String,
    host: String,
    ssl: bool,
    parseJSON: bool
}

impl Settings {
    pub fn new(verson: String,
               host: String,
               ssl: bool,
               parseJSON: bool) -> Settings {
        Settings {
            verson,
            host,
            ssl,
            parseJSON
        }
    }
}

impl SpaceXAPI {
    pub fn new(host: Option<String>,
               ssl: Option<bool>,
               parseJSON: Option<bool>) -> SpaceXAPI {
        let version = "v2";
        let _host = match host {
            Some(h) => h,
            None => "api.spacexdata.com"
        };
        let _ssl
        let settings = Settings {
            verson,
            host : _host,
            ssl,
            parseJSON
        };

        SpaceXAPI {
            settings
        }
    }
    fn get() -> impl Future<Response, Error> {
        Client::new()
            .get("https://hyper.rs")
            .send()
            .into_future()
    }
    pub fn get_company_info() {}
    pub fn get_all_rockets() {}
    pub fn get_rocket() {}
    pub fn get_all_capsules() {}
    pub fn get_capsule() {}
    pub fn get_all_launch_pads() {}
    pub fn get_launch_pad() {}
    pub fn get_latest_launch() {}
    pub fn get_all_launches() {}
    pub fn get_all_past_launches() {}
    pub fn get_all_upcoming_launches() {}
    pub fn get_all_capsule_parts() {}
    pub fn get_capsule_part() {}
    pub fn get_all_core_parts() {}
    pub fn get_core_part() {}
}

