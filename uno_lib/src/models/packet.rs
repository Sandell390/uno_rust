use std::io::Read;

use super::card::Card;

pub struct Packet{
    pub action: Action,
    pub payload: PacketPayload,
} 

impl Packet {
    pub fn new(bytes: &[u8]) -> Result<Packet, &'static str> {

        if bytes.len() < 7 {
            return Err("Packet is too short to be valid");
        }

        let action = Action::try_from(bytes[0])?;
        let mut length_bytes: [u8; 4] = [0; 4];
        length_bytes.copy_from_slice(&bytes[1..5]); // Adjust indices if necessary
        let length = u32::from_be_bytes(length_bytes);

        // Validate the length given is not larger than the bytes slice
        if (5 + length as usize) > bytes.len() {
            return Err("Packet length is larger than byte slice");
        }

        // Get Payload, assuming the length includes the header bytes
        

        Ok(Packet {
            action,
            payload,
        })
    }
}

fn parse_card(payload_bytes: &[u8]) -> Result<Card, &'static str>{

    // Finds the card bytes
    let mut card_bytes: [u8; 3] = [0; 3];
    card_bytes.copy_from_slice(&payload_bytes[0..]);

    Ok(Card::convert_from_arr(card_bytes)?)
}

fn parse_deck(payload_bytes: &[u8]) -> Result<(Vec<Card>, usize), &'static str>{
    
    // The index where we have read to in the payload_bytes
    let mut current_index: u16  = 0;

    // The list of cards
    let mut deck: Vec<Card> = Vec::new();

    // Finds the card count
    let mut card_count: u16 = 0;
    let mut card_count_bytes: [u8; 2] = [0; 2];
    card_count_bytes.copy_from_slice(&payload_bytes[0..1]);
    card_count = u16::from_be_bytes(card_count_bytes);
    current_index += 2;

    // Loops through all the cards in the payload_bytes
    for mut i in 2..=card_count{
        parse_card(&payload_bytes[(current_index + i) as usize..((current_index + i) + 3) as usize]);
        current_index += 3;
    };

    Ok((deck,usize::from(current_index)))
}

fn parse_multiple_decks(payload_bytes: &[u8]) -> Result<Vec<Vec<Card>>, &'static str>{
    // The index where we have read to in the payload_bytes
    let mut current_index: u16  = 0;

    // The list of decks
    let mut decks: Vec<Vec<Card>> = Vec::new();

    // Finds the card count
    let mut deck_count: u16 = 0;
    let mut deck_count_bytes: [u8; 2] = [0; 2];
    deck_count_bytes.copy_from_slice(&payload_bytes[0..1]);
    deck_count = u16::from_be_bytes(deck_count_bytes);
    current_index += 2;

    // Loops through all the cards in the payload_bytes
    for mut i in 2..=deck_count{
        parse_deck(&payload_bytes[(current_index + i) as usize..((current_index + i) + 3) as usize]);
        current_index += 3;
    };

    Ok((decks))
}

fn parse_bool(payload_bytes: &[u8]) -> Result<bool, &'static str>{

}

fn parse_u8(payload_bytes: &[u8]) -> Result<u8, &'static str>{

}

fn parse_string(payload_bytes: &[u8]) -> Result<String, &'static str>{

}

fn parse_multiple_string(payload_bytes: &[u8]) -> Result<Vec<String>, &'static str>{

}
pub enum  PacketPayload{
    // Server Actions
    PlayerDeck(Vec<Card>), // Sends the current player cards
    UnknowDeck(Vec<Vec<Card>>), // Sends to the enemy cards with 
    SpectateDeck(Vec<Vec<Card>>), // Sends all the cards
    PlayerTurn(bool), // Tells the player that it is their turn
    PlayerWait(bool), // Tells the player to wait and which player that has the turn
    ChooseError, // Tells the player that they did something wrong 

    // Two way action
    CardAction(Card), // Have a card action attach
    StartGame, // Tells the server that the player wants to start the game || Tells the client that they can start the start the game


    // Player Actions
    CallUno, // Tells the server that the player calls 
    ChooseCard(Card), // Tells the server which card the player have choosen
    PickUp, // Tells the server that the player wants to pick up a card 
    EndRound, // Tells the server that the player wants to end the round

    // Server Pre Game Actions
    ShowGames(Vec<String>), // Sends the list of current games
    GameLobby, // Puts the player into a game lobby


    // Player Pre Game Actions
    CreateGame(String), // Tells the server that the player wants to create a game
    JoinGame(u8), // Tells the server that the player wants to join a game
    SpectateGame(u8), // Tells the server that the player wants to spectate a game

    // Player Events
    PlayerLeave(String), // Notify all players that a player have left
    PlayerJoin(String), // Notify all players that a playeer have joined
    PlayerDone(String) // Nofity all players when a player is done playing
}

pub enum Action{
    // Server Actions
    PlayerDeck,
    UnknowDeck,
    SpectateDeck,
    PlayerTurn,
    PlayerWait,
    ChooseError,

    // Two way action
    CardAction,
    StartGame,

    // Player Actions
    CallUno,
    ChooseCard,
    PickUp,
    EndRound,

    // Server Pre Game Actions
    ShowGames,
    GameLobby,


    // Player Pre Game Actions
    CreateGame,
    JoinGame,
    SpectateGame,

    // Player Events
    PlayerLeave,
    PlayerJoin,
    PlayerDone
}

impl TryFrom<u8> for Action {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {

        // Converts u8 to Action
        match value {
            0 => Ok(Action::PlayerDeck),
            1 => Ok(Action::UnknowDeck),
            2 => Ok(Action::SpectateDeck),
            3 => Ok(Action::PlayerTurn),
            4 => Ok(Action::PlayerWait),
            5 => Ok(Action::ChooseError),
            6 => Ok(Action::CardAction),
            7 => Ok(Action::StartGame),
            8 => Ok(Action::CallUno),
            9 => Ok(Action::ChooseCard),
            10 => Ok(Action::PickUp),
            11 => Ok(Action::EndRound),
            12=> Ok(Action::ShowGames),
            13=> Ok(Action::GameLobby),
            14=> Ok(Action::CreateGame),
            15=> Ok(Action::JoinGame),
            16=> Ok(Action::SpectateGame),
            17=> Ok(Action::PlayerLeave),
            18=> Ok(Action::PlayerJoin),
            19=> Ok(Action::PlayerDone),
            _ => Err("Could not convert u8 to Action: Out of range")
        }
    }
}