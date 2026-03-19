#[derive(Default)]
pub struct Hitbox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Hitbox {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn new_extend_upwards(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::new(x, y + height, width, height)
    }

    pub fn move_x(&mut self, dx: i32, level: &[Hitbox]) -> i32 {
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

    pub fn move_y(&mut self, dy: i32, level: &[Hitbox]) -> i32 {
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

    pub fn touching_below<'a>(&self, level: &'a [Hitbox]) -> Option<&'a Hitbox> {
        let potential_obstacles = level
            .iter()
            .filter(|o| o.x < self.x + self.width && self.x < o.x + o.width);

        for obstacle in potential_obstacles {
            if self.y - self.height == obstacle.y {
                return Some(&obstacle);
            }
        }
        None
    }
}
