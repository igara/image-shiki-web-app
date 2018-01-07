extern crate iron;
extern crate router;
extern crate params;
extern crate serde_json;
extern crate handlebars_iron as hbs;
extern crate cpython;

use self::iron::prelude::*;
use self::iron::status;
use self::params::{Params, Value};
use self::hbs::{Template};
use self::hbs::handlebars::to_json;
use self::serde_json::{Map};
use std::fs::{File};
use std::fs;
use std::io::{BufReader, Read, BufWriter};
use std::io::Write;
use std::io::Result;

pub struct ApiController;
impl ApiController {
	pub fn index(_: &mut Request) -> IronResult<Response> {
		let mut response = Response::new();
		let mut data = Map::new();
		data.insert("year".to_string(), to_json(&"20".to_owned()));
		response.set_mut(Template::new("api_index", data)).set_mut(status::Ok);
		Ok(response)
	}

	pub fn upload_file(request: &mut Request) -> IronResult<Response> {
		match request.get_ref::<Params>().unwrap().find(&["file"]) {
			Some(&Value::File(ref file)) => {
				let filename = format!("{:?}", file.path);
				println!("{:?}", file);
				let mut reader = BufReader::new(File::open(filename).unwrap());
				let mut read_str = String::new();
				reader.read_to_string(&mut read_str).unwrap();

				// let mut writter = BufWriter::new(File::create("./src/cache/image/aaa.png").unwrap());
				// writter.write(read_str.as_bytes()).unwrap();
				Ok(Response::with((status::Ok, "Welcome back, Marie!")))
			},
			_ => Ok(Response::with(status::NotFound)),
		}
	}
}
