extern crate reqwest;
extern crate futures;
extern crate serde;

use serde::Serialize;
use reqwest::Client;
use reqwest::Response;
use reqwest::Error;
use futures::Future;
use futures::IntoFuture;
use reqwest::RequestBuilder;

pub struct SpaceXAPI {
    settings: Settings
}

struct Settings {
    version: &'static str,
    host: &'static str,
    ssl: bool,
    parse_json: bool,
}

impl Settings {
    pub fn new(version: &'static str,
               host: &'static str,
               ssl: bool,
               parse_json: bool) -> Settings {
        Settings {
            version,
            host,
            ssl,
            parse_json,
        }
    }
}

impl SpaceXAPI {
    pub fn new(host: Option<&'static str>,
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
            host: _host,
            ssl: _ssl,
            parse_json: _parse_json,
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
        if filters.is_some() {
            req_builder.query(filters.unwrap());
        }

        req_builder
            .send()
            .into_future()
    }

    fn get<T: Serialize + ?Sized>(&self, path: &str, filters: Option<&T>) -> impl Future<Item=Response, Error=Error> {
        self.request(path, filters)
    }

    pub fn get_company_info(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/info", None::<&()>)
    }

    pub fn get_all_rockets(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/rockets", None::<&()>)
    }

    pub fn get_rocket(&self, id: &'static str) -> impl Future<Item=Response, Error=Error> {
        self.get(format!("/rockets/{}", id).as_str(), None::<&()>)
    }

    pub fn get_all_capsules(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/capsules", None::<&()>)
    }

    pub fn get_capsule(&self, id: &'static str) -> impl Future<Item=Response, Error=Error> {
        self.get(format!("/capsules/{}", id).as_str(), None::<&()>)
    }

    pub fn get_all_launch_pads(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/launchpads", None::<&()>)
    }

    pub fn get_launch_pad(&self, id: &'static str) -> impl Future<Item=Response, Error=Error> {
        self.get(format!("/launchpads/{}", id).as_str(), None::<&()>)
    }

    pub fn get_latest_launch(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/launches/latest", None::<&()>)
    }

    pub fn get_all_launches(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/launches/all", None::<&()>)
    }

    pub fn get_all_past_launches(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/launches", None::<&()>)
    }

    pub fn get_all_upcoming_launches(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/launches/upcoming", None::<&()>)
    }

    pub fn get_all_capsule_parts(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/parts/caps", None::<&()>)
    }

    pub fn get_capsule_part(&self, id: &'static str) -> impl Future<Item=Response, Error=Error> {
        self.get(format!("/parts/caps/{}", id).as_str(), None::<&()>)
    }

    pub fn get_all_core_parts(&self) -> impl Future<Item=Response, Error=Error> {
        self.get("/parts/cores", None::<&()>)
    }

    pub fn get_core_part(&self, id: &'static str) -> impl Future<Item=Response, Error=Error> {
        self.get(format!("/parts/cores/{}", id).as_str(), None::<&()>)
    }
}
