extern crate router;
extern crate iron;
extern crate mount;
extern crate staticfile;
use std::path::Path;

use app::controllers::IndexController;
use self::router::Router;
use self::iron::{
    Chain
};
use self::mount::Mount;
use self::staticfile::Static;

pub fn all() -> Chain {
	let mut router = Router::new();  
    router.post("/", IndexController::create, "index_create");
    router.get("/users/:id", IndexController::read, "index_read");
    router.put("/users/:id", IndexController::update, "index_update");
    router.delete("/users/:id", IndexController::delete, "index_delete");
    router.get("/", IndexController::index, "index");

    let mut mount = Mount::new();
    mount.mount("/assets/", Static::new(Path::new("./src/assets")));
    mount.mount("/", router);
    let chain = Chain::new(mount);
    chain
}
