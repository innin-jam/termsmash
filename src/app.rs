use crate::input::Input;

pub struct App {
    player: Player,
    level: Vec<Hitbox>,
    should_quit: bool,
}

#[derive(Default)]
pub struct Hitbox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Default)]
enum Direction {
    Left,
    #[default]
    Right,
}

#[derive(Default)]
struct Player {
    hitbox: Hitbox,
    state: PlayerState,
    direction: Direction,
}

#[derive(Default)]
enum PlayerState {
    #[default]
    Idle,
    Walk,
    Jump(u16),
    Fall(u16),
    JumpForwards(u16),
    FallForwards(u16),
}

// TODO extract into private model, publicly reexport Hitbox
impl Hitbox {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn new_extend_upwards(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::new(x, y + height, width, height)
    }

    fn move_x(&mut self, dx: i32, level: &[Hitbox]) -> i32 {
        if dx == 0 {
            return dx;
        }

        let potential_obstacles = level
            .iter()
            .filter(|o| self.y - self.height < o.y && o.y - o.height < self.y);

        let mut max_move = dx;
        for obstacle in potential_obstacles {
            if dx > 0 {
                if self.x + self.width <= obstacle.x {
                    let gap = obstacle.x - (self.x + self.width);
                    if gap < max_move {
                        max_move = gap;
                    }
                }
            } else {
                if obstacle.x + obstacle.width <= self.x {
                    let gap = (obstacle.x + obstacle.width) - self.x;
                    if gap > max_move {
                        max_move = gap;
                    }
                }
            }
        }
        self.x += max_move;
        max_move
    }

    fn move_y(&mut self, dy: i32, level: &[Hitbox]) -> i32 {
        if dy == 0 {
            return dy;
        }

        let potential_obstacles = level
            .iter()
            .filter(|o| o.x < self.x + self.width && self.x < o.x + o.width);

        let mut max_move = dy;
        for obstacle in potential_obstacles {
            if dy > 0 {
                if self.y <= obstacle.y - obstacle.height {
                    let gap = (obstacle.y - obstacle.height) - self.y;
                    if gap < max_move {
                        max_move = gap;
                    }
                }
            } else {
                if obstacle.y <= self.y - self.height {
                    let gap = obstacle.y - (self.y - self.height);
                    if gap > max_move {
                        max_move = gap;
                    }
                }
            }
        }
        self.y += max_move;
        max_move
    }

    fn touching_below(&self, level: &[Hitbox]) -> bool {
        let potential_obstacles = level
            .iter()
            .filter(|o| o.x < self.x + self.width && self.x < o.x + o.width);

        for obstacle in potential_obstacles {
            if self.y - self.height == obstacle.y {
                return true;
            }
        }
        false
    }
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        let width = 3;
        let height = 3;
        Self {
            hitbox: Hitbox::new_extend_upwards(x, y, width, height),
            direction: Direction::Right,
            state: PlayerState::Idle,
        }
    }

    fn tick(&mut self, input: Option<Input>, level: &[Hitbox]) {
        match self.state {
            PlayerState::Idle => {
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
                let dx = match self.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                if self.hitbox.move_x(dx, level) != dx {
                    self.state = PlayerState::Idle
                };
                if !self.hitbox.touching_below(level) {
                    self.state = PlayerState::FallForwards(0);
                    return;
                }
                if let Some(input) = input {
                    match input {
                        Input::Left if matches!(self.direction, Direction::Right) => {
                            self.direction = Direction::Left;
                        }
                        Input::Right if matches!(self.direction, Direction::Left) => {
                            self.direction = Direction::Right;
                        }
                        Input::Up => self.state = PlayerState::JumpForwards(0),
                        Input::Down => self.state = PlayerState::Idle,
                        _ => {}
                    }
                }
            }
            PlayerState::Jump(ref mut f) => {
                *f += 1;
                let dy = match f {
                    1 => 2,
                    2 => 2,
                    3 => 1,
                    4 => 1,
                    5 => 1,
                    6 => 1,
                    7 => 0,
                    8 => 0,
                    9 => 0,
                    _ => {
                        self.state = PlayerState::Fall(0);
                        return;
                    }
                };
                if self.hitbox.move_y(dy, level) != dy {
                    self.state = PlayerState::Fall(0)
                };
            }
            PlayerState::Fall(ref mut f) => {
                *f += 1;
                let dy = match *f {
                    1..=3 => -1,
                    _ => -2,
                };
                if self.hitbox.move_y(dy, level) != dy {
                    self.state = PlayerState::Idle;
                }
            }
            PlayerState::JumpForwards(ref mut f) => {
                *f += 1;
                let dx = match self.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                let dy = match f {
                    1 => 2,
                    2 => 2,
                    3 => 1,
                    4 => 1,
                    5 => 1,
                    6 => 1,
                    7 => 0,
                    8 => 0,
                    9 => 0,
                    _ => {
                        self.state = PlayerState::FallForwards(0);
                        return;
                    }
                };
                match (
                    self.hitbox.move_x(dx, level) != dx,
                    self.hitbox.move_y(dy, level) != dy,
                ) {
                    (true, false) => self.state = PlayerState::Jump(*f),
                    (false, true) => self.state = PlayerState::FallForwards(0),
                    (true, true) => self.state = PlayerState::Fall(0),
                    _ => {}
                }
            }
            PlayerState::FallForwards(ref mut f) => {
                *f += 1;
                let dx = match self.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                let dy = match *f {
                    1..=3 => -1,
                    _ => -2,
                };
                match (
                    self.hitbox.move_x(dx, level) != dx,
                    self.hitbox.move_y(dy, level) != dy,
                ) {
                    (true, false) => self.state = PlayerState::Fall(*f),
                    (false, true) => self.state = PlayerState::Walk,
                    (true, true) => self.state = PlayerState::Idle,
                    _ => {}
                }
            }
        }
    }
}

impl App {
    pub fn new() -> Self {
        App {
            player: Player::new(0, 0),
            level: vec![
                Hitbox {
                    x: -7,
                    y: 0,
                    width: 60,
                    height: 10,
                },
                Hitbox {
                    x: -19,
                    y: 0,
                    width: 10,
                    height: 10,
                },
                Hitbox {
                    x: -10,
                    y: 8,
                    width: 30,
                    height: 3,
                },
                Hitbox {
                    x: -30,
                    y: 1,
                    width: 10,
                    height: 10,
                },
            ],
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
        &self.player.hitbox
    }

    pub fn level(&self) -> &Vec<Hitbox> {
        &self.level
    }
}
