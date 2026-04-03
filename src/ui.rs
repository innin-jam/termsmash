use ratatui::{
    Frame,
    layout::Rect,
    prelude::Buffer,
    symbols,
    widgets::{Block, Widget},
};

use crate::app::App;
use crate::app::Hitbox;

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

// impl Widget for &Player {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         let sprite = {
//             Text::from(vec![
//                 Line::from("###"),
//                 Line::from("###"),
//                 Line::from("###"),
//             ])
//         };

//         let render_x = (self.x + (area.width / 2) as i32) as u16;
//         let render_y = (-self.y + (area.height / 2) as i32) as u16;

//         let rect = Rect {
//             x: render_x,
//             y: render_y,
//             width: 3,
//             height: 3,
//         };

//         Paragraph::new(sprite).render(rect, buf);
//     }
// }

impl Widget for &Hitbox {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

    frame.render_widget(app.player_hitbox(), area);
    for platform in app.level() {
        frame.render_widget(platform, area);
    }
}
