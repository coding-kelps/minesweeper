use std::{io, hash::Hash};
use regex::Regex;

struct Listener {
    cmds: Vec<Regex>,
}
fn start_wrapper(cmd: &str) {

}

fn pick_wrapper(cmd: &str) {
}

impl Listener {
    // pub fn new() -> Self {
    //     let cmds = Vec::from([
    //         (Regex::new(r"^START(| +.*)$").unwrap(), start_wrapper),
    //         // (Regex::new(r"^PICK(| +.*)$").unwrap(), pick_wrapper),
    //     ]);
    // }


    pub fn listen(&self) {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}