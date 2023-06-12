#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types

mod todo;

fn main() {
    // launch the web app
    dioxus_web::launch(todo::app);
}

