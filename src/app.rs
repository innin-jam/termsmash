use crate::input::Input;

pub struct App {
    player: Player,
    level: Vec<Platform>,
    should_quit: bool,
}

#[derive(Default)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Default)]
enum Direction {
    Left,
    #[default]
    Right,
}

#[derive(Default)]
struct Player {
    pos: Pos,
    state: PlayerState,
    direction: Direction,
}

#[derive(Default)]
enum PlayerState {
    #[default]
    Still,
    Walk,
    Jump(u16),
}

impl Player {
    fn tick(&mut self, input: Option<Input>, level: &Vec<Platform>) {
        match self.state {
            PlayerState::Still => {
                if let Some(input) = input {
                    match input {
                        Input::Left => {
                            self.direction = Direction::Left;
                            self.state = PlayerState::Walk
                        }
                        Input::Right => {
                            self.direction = Direction::Right;
                            self.state = PlayerState::Walk
                        }
                        Input::Up => self.state = PlayerState::Jump(0),
                        _ => {}
                    }
                }
            }
            PlayerState::Walk => {
                self.pos.x += match self.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                if let Some(input) = input {
                    match input {
                        Input::Down => self.state = PlayerState::Still,
                        Input::Left if matches!(self.direction, Direction::Right) => {
                            self.direction = Direction::Left;
                        }
                        Input::Right if matches!(self.direction, Direction::Left) => {
                            self.direction = Direction::Right;
                        }
                        _ => {}
                    }
                }
            }
            PlayerState::Jump(ref mut f) => {
                *f += 1;
                if *f > 11 {
                    self.state = PlayerState::Still;
                    return;
                }
                self.pos.y += {
                    match f {
                        1 => 2,
                        2 => 2,
                        3 => 1,
                        4 => 1,
                        5 => 0,
                        6 => 0,
                        7 => 0,
                        8 => -1,
                        9 => -1,
                        _ => -2,
                    }
                };
            }
        }
    }
}

struct Platform {
    pub topleft: Pos,
    pub bottomright: Pos,
}

impl App {
    pub fn new() -> Self {
        App {
            player: Player::default(),
            level: vec![Platform {
                topleft: Pos { x: 0, y: 0 },
                bottomright: Pos { x: 50, y: -10 },
            }],
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

    pub fn player_xy(&self) -> (i32, i32) {
        (self.player.pos.x, self.player.pos.y)
    }

    pub fn level(&self) -> Vec<[i32; 4]> {
        let mut out: Vec<[i32; 4]> = vec![];
        for platform in &self.level {
            let (x, y, x2, y2) = (
                platform.topleft.x,
                platform.topleft.y,
                platform.bottomright.x,
                platform.bottomright.y,
            );
            out.push([x, y, x2, y2]);
        }
        out
    }
}
