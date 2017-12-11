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
use self::serde_json::{Map};

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
		let mut data = Map::new();
		data.insert("year".to_string(), to_json(&"2015".to_owned()));
		response.set_mut(Template::new("index_index", data)).set_mut(status::Ok);
		Ok(response)
	}
}
