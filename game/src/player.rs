use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::audio::{Source};
use ggez::input::keyboard::{KeyCode, is_key_pressed};

pub struct Player {
	pub x_pos: f32,
    pub y_pos: f32,
    pub width: f32,
    pub height: f32,
	pub speed: f32,
	pub sound: Source
}

impl Player {
    // Draws a rectangle.
	pub fn rect(&self, ctx: &mut Context) -> GameResult {
		let rect = graphics::Rect::new(self.x_pos, self.y_pos, self.width, self.height);
		let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
		graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())
    }
    
    // Binds keypressess to movement.
    pub fn movement(&mut self, ctx: &mut Context, up: KeyCode, down: KeyCode, left: KeyCode, right: KeyCode) {
        if is_key_pressed(ctx, right) {
			self.x_pos += self.speed;
		}
		if is_key_pressed(ctx, left) {
			self.x_pos -= self.speed;
		}
		if is_key_pressed(ctx, up) {
			self.y_pos -= self.speed;
		}
		if is_key_pressed(ctx, down) {
			self.y_pos += self.speed;
		}
    }

    pub fn shoot(&self, ctx: &mut Context) {
        // for loop to move the bullet.
        let mut x = self.x_pos;
        let y = self.y_pos;

        for _ in 0..10 {
            let rect = graphics::Rect::new(x, y, 10.0, 10.0);
            let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE).unwrap();
            graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default()).unwrap();
            graphics::present(ctx).unwrap();

            x += 20.0;
        }
    }
}