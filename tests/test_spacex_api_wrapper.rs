extern crate spacex_api_wrapper;
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

use spacex_api_wrapper::SpaceXAPI;

#[test]
fn get_company_info(){
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_company_info()
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}

#[test]
fn get_all_rockets() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_all_rockets()
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_rocket() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_rocket("falconheavy")
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_all_capsules() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_all_capsules()
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_capsule() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_capsule("dragon1")
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_all_launch_pads() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_all_launch_pads()
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_launch_pad() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_launch_pad("ksc_lc_39a")
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_latest_launch() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_latest_launch()
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_all_launches() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_all_launches()
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_all_past_launches() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_all_past_launches()
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_all_upcoming_launches() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_all_upcoming_launches()
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_all_capsule_parts() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_all_capsule_parts()
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_capsule_part() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_capsule_part("C113")
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_all_core_parts() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_all_core_parts()
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
#[test]
fn get_core_part() {
    let spacex_api = SpaceXAPI::new(None, None, None);
    spacex_api.get_core_part("B1041")
        .wait()
        .map(|mut b| {
            assert_eq!(b.status(), reqwest::StatusCode::Ok);
            println!("{:?}", b.text());
        });
}
