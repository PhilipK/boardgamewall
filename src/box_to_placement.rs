use crate::{svg_generation::BoxDraw, placement::Placement};

impl<'a> Into<BoxDraw<'a>> for Placement<'a> {
    fn into(self) -> BoxDraw<'a> {
        let g = self.game;
        let width = g.width as f32 - (g.margin_right);
        let height = g.height as f32 - (g.margin_top);
        BoxDraw {
            x: self.x as f32,
            y: self.y as f32,
            name: &self.game.name,
            width,
            height,
        }
    }
}
