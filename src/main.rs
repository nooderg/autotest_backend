#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate eventbus;
#[macro_use]
extern crate lazy_static;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod schema;

// #[derive(Debug)]
// struct MyEvent {
//     i: i32
// }

// impl Event for MyEvent {}

// fn add_handler(e: &mut MyEvent) {
//     println!("{:?}", e);
// }

// let event_bus = EventBus::new();
// register_hook!(&event_bus, 0, MyEvent, add_handler);

// let mut event = MyEvent { i: 3 };

// post_event!(&event_bus, &mut event, MyEvent);

#[launch]
fn rocket() -> _ {
    infrastructure::event_bus::register::register_events();

    rocket::build()
        .mount("/users/", infrastructure::api::user_routes())
}
