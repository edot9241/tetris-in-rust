#![allow(dead_code, unused_imports, unused_variables)]
//WARN:Remove when no longer necessary!
mod engine;
mod logic;
mod ui;

fn main() {
    engine::window();
    let game = logic::Game::new();
}

// let tetrominoes = [
//     ('I', "Cyan"),
//     ('O', "Yellow"),
//     ('T', "Purple"),
//     ('S', "Green"),
//     ('Z', "Red"),
//     ('J', "Blue"),
//     ('L', "Orange")
// ];
//TODO: Korobeiniki song mandatory, Katyusha and Kalinka optional
