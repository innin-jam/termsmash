use ratatui::{
    Frame,
    layout::Rect,
    prelude::Buffer,
    symbols,
    widgets::{Block, Paragraph, Widget},
};

use crate::app::App;
use crate::app::Hitbox;
use crate::app::entity::Player;

const HITBOX_BORDER: symbols::border::Set = symbols::border::Set {
    top_left: "█",
    top_right: "█",
    bottom_left: "█",
    bottom_right: "█",
    vertical_left: "█",
    vertical_right: "█",
    horizontal_top: "▀",
    horizontal_bottom: "▄",
};

struct ScreenObject {
    pub rect: Rect,
    pub contents: String,
}

const PLAYER_SPRITE: &str = "\
███
███
███
";

const PLAYER_SPRITE_CROUCH: &str = "\
   \n\
███
███
";

fn sprite_dimensions(sprite: &str) -> (u16, u16) {
    let height = sprite.lines().count() as u16;
    let width = sprite.lines().map(|l| l.chars().count()).max().unwrap_or(0) as u16;
    (width, height)
}

impl ScreenObject {
    fn clip_left(content: &str, amount: usize) -> String {
        content
            .lines()
            .map(|line| line.chars().skip(amount).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn clip_top(content: &str, amount: usize) -> String {
        content.lines().skip(amount).collect::<Vec<_>>().join("\n")
    }

    fn from_player(player: &Player, area: Rect) -> Option<Self> {
        let mut sprite = if matches!(player.state(), crate::app::entity::PlayerState::Crouch(_)) {
            PLAYER_SPRITE_CROUCH
        } else {
            PLAYER_SPRITE
        }
        .to_string();
        let (sprite_w, sprite_h) = sprite_dimensions(&sprite);

        let render_x = player.hitbox().x + (area.width / 2) as i32;
        let render_y = -player.hitbox().y + (area.height / 2) as i32;

        if render_x >= area.width as i32
            || render_y >= area.height as i32
            || render_x + sprite_w as i32 <= 0
            || render_y + sprite_h as i32 <= 0
        {
            return None;
        }

        if render_x < 0 {
            sprite = Self::clip_left(&sprite, render_x.unsigned_abs() as usize);
        }
        if render_y < 0 {
            sprite = Self::clip_top(&sprite, render_y.unsigned_abs() as usize);
        }

        Some(ScreenObject {
            rect: Rect::new(
                render_x.max(0) as u16,
                render_y.max(0) as u16,
                sprite_w,
                sprite_h,
            ),
            contents: sprite,
        })
    }
}

impl Widget for ScreenObject {
    fn render(self, _area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(self.contents).render(self.rect, buf);
    }
}

impl Widget for &Hitbox {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let render_x = (self.x + (area.width / 2) as i32) as u16;
        let render_y = (-self.y + (area.height / 2) as i32) as u16;
        let render_width = (self.width) as u16;
        let render_height = (self.height) as u16;

        let rect = Rect {
            x: render_x,
            y: render_y,
            width: render_width,
            height: render_height,
        };

        Block::bordered()
            .border_set(HITBOX_BORDER)
            .render(rect, buf);
    }
}

pub fn ui(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // render hitboxes
    // frame.render_widget(app.player_hitbox(), area);
    for platform in app.level() {
        frame.render_widget(platform, area);
    }

    let mut screen_objects = Vec::<ScreenObject>::new();

    if let Some(player_sprite) = ScreenObject::from_player(&app.player, area) {
        screen_objects.push(player_sprite);
    }

    for o in screen_objects {
        frame.render_widget(o, area);
    }
}
