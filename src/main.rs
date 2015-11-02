#![feature(custom_derive)]
#![feature(fmt_flags)]

extern crate hyper;
extern crate rustc_serialize;


use std::fmt;
use std::io::Read;
use std::collections::HashSet;

use hyper::client::{Client, Response, RequestBuilder};
use hyper::header::{Accept, Connection, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};

use rustc_serialize::json;

struct MyClient {
    client : Client,
    app_id : String,
    app_key : String,
}

#[derive(Hash, Debug, RustcEncodeable, RustcDecodable)]
struct Line {
    id : String,
    name : String,
    modeName : String,
    routeSections : Vec<RouteSection>
}

#[derive(Hash, Debug, RustcEncodeable, RustcDecodable)]
struct RouteSection {
    name : String,
    direction : String,
    originator : String,
    destination : String,
}

impl fmt::Display for Line {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.id)
    }
}

impl MyClient {
    fn new() -> MyClient {
        return MyClient{
            client : Client::new(),
            app_id : String::new(),
            app_key : String::new(),
        }
    }

    fn get(&mut self, endpoint : &str) -> Response {
        self.client.get("https://api.tfl.gov.uk/line/route?app_id=&app_key=")
            .header(Connection::close())
            .header(Accept(vec![
                qitem(Mime(TopLevel::Application,
                    SubLevel::Ext("json".to_owned()), vec![])),
            ]))
            .send().unwrap()
    }
}

fn main() {
    let mut line_names : HashSet<String> = HashSet::new();
    let mut route_section_names : HashSet<String> = HashSet::new();
    let mut client = MyClient::new();
    let mut resp = client.get("/line/route");
    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();
    let lines : Vec<Line> = json::decode(&body).unwrap();
    for line in lines {
        let line_known = line_names.contains(&line.name);
        println!("{}, Duplicate: {}:", line.name, line_known);
        line_names.insert(line.name.clone());
        for routeSection in line.routeSections {
            let route_section_known = route_section_names.contains(&routeSection.name);
            println!("\t{}, Duplicate: {}", routeSection.name, route_section_known);
            route_section_names.insert(routeSection.name.clone());
        }
    }
}