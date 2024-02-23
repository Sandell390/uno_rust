use std::net::TcpStream;

use crate::models::player::Player;

use uno_lib::models::{self, card::Card};

pub struct Game{
    pub players: [Player; 10],
    pub player_count: u8,
    pub used_deck: [Card; 52],
    pub deck: [Card; 52],
    pub current_player_index: u8,
    pub winner_index: u8,
    pub reverse: bool,
}


impl Game {
    pub fn client_listen(stream: TcpStream, username: String){
        

        // Contiue to listen if client is sending something
        loop {
            
            

        }
    }
}