extern crate iron;

mod routes;
mod controllers;
mod models;
mod helpers;

use self::iron::prelude::Iron;

pub fn run() {
	Iron::new(routes::all()).http("0.0.0.0:80").unwrap();
}
