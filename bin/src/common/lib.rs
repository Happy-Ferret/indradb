extern crate chrono;
extern crate indradb;
extern crate serde_json;
extern crate uuid;

mod datastore;

pub use datastore::{datastore, ProxyDatastore, ProxyTransaction};
