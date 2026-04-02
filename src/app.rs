use crate::app::entity::Player;
use crate::input::Input;
pub use hitbox::Hitbox;

mod hitbox;

mod entity;

pub struct App {
    player: Player,
    level: Vec<Hitbox>,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            player: Player::new(0, 0),
            #[rustfmt::skip]
            level: vec![ Hitbox { x: -90, y: -20, width: 4, height: 40, }, Hitbox { x: 86, y: -20, width: 4, height: 40, }, Hitbox { x: -84, y: -17, width: 40, height: 3, }, Hitbox { x: -35, y: -17, width: 50, height: 3, }, Hitbox { x: 28, y: -17, width: 58, height: 3, }, Hitbox { x: -80, y: -8, width: 18, height: 3, }, Hitbox { x: -55, y: 0, width: 20, height: 3, }, Hitbox { x: -20, y: 8, width: 22, height: 3, }, Hitbox { x: 10, y: 14, width: 20, height: 3, }, Hitbox { x: 40, y: 5, width: 18, height: 3, }, Hitbox { x: 62, y: -5, width: 18, height: 3, }, ],
            should_quit: false,
        }
    }

    pub fn tick(&mut self, input: Option<Input>) {
        if let Some(ref input) = input {
            if matches!(input, Input::Quit) {
                self.should_quit = true;
            }
        }

        self.player.tick(input, &self.level);
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn player_hitbox(&self) -> &Hitbox {
        &self.player.hitbox()
    }

    pub fn level(&self) -> &Vec<Hitbox> {
        &self.level
    }
}
