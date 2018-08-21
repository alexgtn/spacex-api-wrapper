# SpaceX API wrapper written in Rust

[![GitHub issues](https://img.shields.io/github/issues/AGutan/spacex-api-wrapper.svg)](https://github.com/AGutan/spacex-api-wrapper/issues)
[![GitHub stars](https://img.shields.io/github/stars/AGutan/spacex-api-wrapper.svg)](https://github.com/AGutan/spacex-api-wrapper/stargazers)
[![GitHub license](https://img.shields.io/github/license/AGutan/spacex-api-wrapper.svg)](https://github.com/AGutan/spacex-api-wrapper)

# API Reference
See the full API reference [here](https://github.com/r-spacex/SpaceX-API/blob/master/docs/home.md)

# Installation
Via `cargo`, add this to your project's `Cargo.toml`:
```Rust
[dependencies]
spacex-api-wrapper = "0.2.0"
```

# Usage example
```Rust
extern crate spacex_api_wrapper;
use spacex_api_wrapper::SpaceXAPI;

let spacex_api = SpaceXAPI::default();
spacex_api.get_company_info()
    .wait()
    .map(|mut b| {
        println!("{:?}", b.text());
    });
```
