use std::net::TcpStream;

use uno_lib::models::{self, card::Card};

pub struct Player{
    name: String,
    cards: [Card; 52],
    state: PlayerState,
}

pub enum PlayerState {
    IDLE,
    ACTIVE,
    UNO,
    DONE,
    DISCONNECTED
}