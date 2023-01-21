#![allow(unused)]
use rand::{self, Rng};

fn main() {
    let unsigned: u32 = 500;
    let signed: i8 = -4;
    let float: f32 = 3.1415;
    println!("Hello, world!");
    let num = random_number(100.0, 0.0);
    println!("{}", num);
}

enum State {
    Init,
    Error(Result<(), u32>),
    Read((u8, u32)),
    Write(Vec<u8>),
    Close,
}

fn match_test(state: State) {
    match state {
        State::Init => todo!(),
        State::Error(error) => todo!(),
        State::Read((channel, samples)) => todo!(),
        State::Write(bytes) => todo!(),
        State::Close => todo!(),
    }
}

/// Generate a random number between upper and lower bounds
fn random_number(upper_bound: f32, lower_bound: f32) -> f32 {
    rand::thread_rng().gen_range(lower_bound..=upper_bound)
}
