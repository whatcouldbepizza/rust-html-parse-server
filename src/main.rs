pub mod models;
pub mod schema;
pub mod ps_adapter;
pub mod server;

use futures::executor::block_on;

#[macro_use]
extern crate diesel;

fn main() {
    std::thread::spawn(|| {
        println!("starting updater");
        let future_handler = ps_adapter::update_statuses();
        block_on(future_handler)
    });
    server::start_api().expect("Server error");
}
