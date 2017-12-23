extern crate iron;
extern crate router;
extern crate serde_json;
extern crate handlebars_iron as hbs;

use app::models::Index;
use app::helpers::IndexHelper;
use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use self::hbs::{Template};
use self::hbs::handlebars::to_json;
use self::serde_json::{Map, Value};
use std::fs::{File};
use std::io::{BufReader, Read};
use std::str::from_utf8;

pub struct IndexController;
impl IndexController {
	pub fn create(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
	pub fn read(req: &mut Request) -> IronResult<Response> {
		let id = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i64>().unwrap();
		let index = Index::get_by_id(id);
		let encoded_index = serde_json::to_string(&index).unwrap();
	    Ok(Response::with((iron::status::Ok, encoded_index)))
	}
	pub fn update(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
	pub fn delete(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
	pub fn index(_: &mut Request) -> IronResult<Response> {
	    let mut response = Response::new();

		let mut reader = BufReader::new(File::open("./src/assets/image_shiki.json").unwrap());
		let mut read_str = String::new();
        reader.read_to_string(&mut read_str).unwrap();
		let static_json: Value = serde_json::from_str(&read_str).unwrap();

		let mut data = Map::new();
		data.insert("static_json".to_string(), to_json(&static_json.to_owned()));
		data.insert("year".to_string(), to_json(&"2015".to_owned()));
		response.set_mut(Template::new("index_index", data)).set_mut(status::Ok);
		Ok(response)
	}
}
