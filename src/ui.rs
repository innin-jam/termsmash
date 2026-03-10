use ratatui::{
    Frame,
    layout::Rect,
    prelude::Buffer,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::app::App;

struct Player {
    x: i32,
    y: i32,
}

struct Platform {
    x: i32,
    y: i32,
    x2: i32,
    y2: i32,
}

impl Widget for &Player {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let sprite = {
            Text::from(vec![
                Line::from("###"),
                Line::from("###"),
                Line::from("###"),
            ])
        };

        let render_x = (self.x + (area.width / 2) as i32) as u16;
        let render_y = (-self.y + (area.height / 2) as i32) as u16;

        let rect = Rect {
            x: render_x,
            y: render_y,
            width: 3,
            height: 3,
        };

        Paragraph::new(sprite).render(rect, buf);
    }
}

impl Widget for &Platform {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let render_x = (self.x + (area.width / 2) as i32) as u16;
        let render_y = (-self.y + (area.height / 2) as i32) as u16;
        let render_width = (self.x2 - self.x) as u16;
        let render_height = (self.y2 - self.y) as u16;

        let rect = Rect {
            x: render_x,
            y: render_y,
            width: render_width,
            height: render_height,
        };

        Block::bordered().render(rect, buf);
    }
}

pub fn ui(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let player = {
        let (x, y) = app.player_xy();
        let x = x;
        let y = y;
        Player { x, y }
    };

    let platform = {
        let [x, y, x2, y2] = app.level()[0];
        Platform { x, y, x2, y2 }
    };

    frame.render_widget(&player, area);
    frame.render_widget(&platform, area);
}
