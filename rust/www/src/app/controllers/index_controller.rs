extern crate iron;
extern crate router;
extern crate serde_json;

use app::models::Index;
use app::helpers::IndexHelper;
use self::iron::prelude::*;
use self::router::Router;

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
	    Ok(Response::with((iron::status::Ok)))
	}
}
