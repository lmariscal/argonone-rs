mod fan_controller;

// use std::thread;

fn main() {
    // let fan_handle = thread::spawn(fan_controller::controller);
    // fan_handle.join().unwrap();
    fan_controller::controller();
}
