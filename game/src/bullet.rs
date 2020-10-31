use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::audio::{Source};
use ggez::input::keyboard::{KeyCode, is_key_pressed};

pub struct Bullet {
	pub x_pos: f32,
    pub y_pos: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub life: f32
}

impl Bullet {
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
		let rect = graphics::Rect::new(self.x_pos, self.y_pos, self.width, self.height);
		let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
		graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())
    }
}