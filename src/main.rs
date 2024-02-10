mod controllers;
mod services;

use controllers::hello_controller;
fn main() {
    _ = hello_controller::hello();
}
