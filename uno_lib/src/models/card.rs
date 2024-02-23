pub struct Card{
    pub color: Color,
    pub card_type: Type,
    pub value: u8,
}

impl Card {
    pub fn new(color: Color, card_type: Type, value: u8) -> Result<Card, &'static str>{


        // Checks if value is inside the range
        if value > 9 || value < 0{
            return Err("Value can not be higher than 9 or lower than 0");
        }

        Ok(Card {
            color,
            card_type,
            value
        })
    }


    pub fn convert_to_arr(&self) -> [u8; 3]{
        [self.color as u8, self.card_type as u8, self.value]
    }

    pub fn convert_from_arr(bytes: [u8; 3]) -> Result<Card, &'static str>{
        Ok(Card {
            color: Color::try_from(bytes[0])?,
            card_type: Type::try_from(bytes[1])?,
            value: bytes[2]
        })
    }
}

pub enum Type{
    Number,
    Plus,
    Reverse,
    Skip,
    Color,
    None
}

impl TryFrom<u8> for Type {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {

        // Converts u8 to Type
        match value {
            0 => Ok(Type::Number),
            1 => Ok(Type::Plus),
            2 => Ok(Type::Reverse),
            3 => Ok(Type::Skip),
            4 => Ok(Type::Color),
            5 => Ok(Type::None),
            _ => Err("Could not convert u8 to Type: Out of range")
        }
    }
}

pub enum Color{
    None,
    Blue,
    Red,
    Green,
    Yellow
}

impl TryFrom<u8> for Color {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {

        // Converts u8 to Color
        match value {
            0 => Ok(Color::None),
            1 => Ok(Color::Blue),
            2 => Ok(Color::Red),
            3 => Ok(Color::Green),
            4 => Ok(Color::Yellow),
            _ => Err("Could not convert u8 to Color: Out of range")
        }
    }
}