pub(crate) const TF2_IDENTIFIER: u32 = 440;
pub(crate) const DOTA2_IDENTIFIER: u32 = 570;
pub(crate) const CSGO_IDENTIFIER: u32 = 730;
pub(crate) const RUST_IDENTIFIER: u32 = 252490;

pub enum Game {
    TF2,
    DOTA2,
    CSGO,
    RUST,
}

impl Game {
    pub fn identifier(&self) -> u32 {
        match self {
            Game::TF2 => TF2_IDENTIFIER,
            Game::DOTA2 => DOTA2_IDENTIFIER,
            Game::CSGO => CSGO_IDENTIFIER,
            Game::RUST => RUST_IDENTIFIER,
        }
    }
}
