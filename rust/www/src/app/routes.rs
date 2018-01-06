extern crate router;
extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;

use std::path::Path;
use app::controllers::IndexController;
use app::controllers::ApiController;
use self::router::Router;
use self::iron::{
    Chain
};
use self::mount::Mount;
use self::staticfile::Static;
use self::hbs::{HandlebarsEngine, DirectorySource};

pub fn all() -> Chain {
	let mut router = Router::new();  
	router.post("/", IndexController::create, "index_create");
	router.get("/users/:id", IndexController::read, "index_read");
	router.put("/users/:id", IndexController::update, "index_update");
	router.delete("/users/:id", IndexController::delete, "index_delete");
	router.get("/", IndexController::index, "index");
	router.get("/api", ApiController::index, "api_index");

	let mut mount = Mount::new();
	mount.mount("/assets/", Static::new(Path::new("./src/assets")));
	mount.mount("/", router);
	let mut chain = Chain::new(mount);
	let mut hbse = HandlebarsEngine::new();
	hbse.add(Box::new(
		DirectorySource::new("./src/app/views/", ".hbs")));
	if let Err(r) = hbse.reload() {
		panic!("{}", r);
	}
	chain.link_after(hbse);
	chain
}
