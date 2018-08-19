extern crate reqwest;
extern crate futures;
extern crate serde;

use serde::Serialize;

use reqwest::Client;
use reqwest::Response;
use reqwest::Error;
use futures::Future;
use futures::IntoFuture;


pub struct SpaceXAPI {
    settings: Settings
}
struct Settings {
    version: String,
    host: String,
    ssl: bool,
    parse_json: bool
}

impl Settings {
    pub fn new(version: String,
               host: String,
               ssl: bool,
               parse_json: bool) -> Settings {
        Settings {
            version,
            host,
            ssl,
            parse_json
        }
    }
}

impl SpaceXAPI {
    pub fn new(host: Option<String>,
               ssl: Option<bool>,
               parse_json: Option<bool>) -> SpaceXAPI {
        let version = "v2";
        let _host = match host {
            Some(h) => h,
            None => "api.spacexdata.com"
        };
        let _ssl = match ssl {
            Some(true) => true,
            Some(false) => false,
            None => false,

        };
        let _parse_json = match parse_json {
            Some(true) => true,
            Some(false) => false,
            None => false,

        };
        let settings = Settings {
            version,
            host : _host,
            ssl: _ssl,
            parse_json: _parse_json
        };

        SpaceXAPI {
            settings
        }
    }
    fn request<T: Serialize + ?Sized>(&self, path: &str, filters: Option<&T>) -> impl Future<Item=Response, Error=Error> {
        let protocol = if self.settings.ssl {
            "https:"
        } else {
            "http:"
        };
        let hostname = format!("{}/{}", self.settings.host, self.settings.version);
        let uri = format!("{}{}{}", protocol, hostname, path);
        let mut req_builder = Client::new()
            .get(uri.as_str());
        let _ = match filters {
            Some(f) => req_builder.query(f)
        };

        req_builder
            .send()
            .into_future()
    }
    fn get<T: Serialize + ?Sized>(&self, path: &str, filters: Option<&T>) -> impl Future<Item=Response, Error=Error> {
        self.request(path, filters)
    }
    pub fn get_company_info(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/info", None)
    }
//    pub fn get_all_rockets() {}
//    pub fn get_rocket() {}
//    pub fn get_all_capsules() {}
//    pub fn get_capsule() {}
//    pub fn get_all_launch_pads() {}
//    pub fn get_launch_pad() {}
//    pub fn get_latest_launch() {}
//    pub fn get_all_launches() {}
//    pub fn get_all_past_launches() {}
//    pub fn get_all_upcoming_launches() {}
//    pub fn get_all_capsule_parts() {}
//    pub fn get_capsule_part() {}
//    pub fn get_all_core_parts() {}
//    pub fn get_core_part() {}
}

#[test]
fn get_company_info(){
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_company_info().wait().map(|b| {
        println!("{:?}", b);
    })
}

