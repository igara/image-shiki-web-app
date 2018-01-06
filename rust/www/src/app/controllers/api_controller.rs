extern crate iron;
extern crate router;
extern crate serde_json;
extern crate handlebars_iron as hbs;
extern crate cpython;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use self::hbs::{Template};
use self::hbs::handlebars::to_json;
use self::serde_json::{Map};
use self::cpython::{PyResult, Python, PyObject};

pub struct ApiController;
impl ApiController {
	pub fn index(_: &mut Request) -> IronResult<Response> {
		let mut response = Response::new();
		let mut data = Map::new();
		data.insert("year".to_string(), to_json(&"20".to_owned()));
		response.set_mut(Template::new("api_index", data)).set_mut(status::Ok);
		Ok(response)
	}
}
