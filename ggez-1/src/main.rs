use ggez::*;

struct State {
    xpos:      f32,
    direction: i8,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let background = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            // Draw a 999x999 rectangle with top left at (0, 0)
            graphics::Rect::new(0.0, 0.0, 999.9, 999.9),
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &background, graphics::DrawParam::default())?;
        graphics::present(ctx)?;

        if self.xpos < 1000.0 && self.xpos > 0.0 {
            self.xpos += (25.0 * self.direction as f32);
        } else if self.xpos >= 1000.0 {
            self.direction *= -1;
            self.xpos -= 25.0;
        } else if self.xpos <= 0.0 {
            self.direction *= -1;
            self.xpos += 25.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx, // window context
            graphics::DrawMode::fill(), // we want solid colour
            mint::Point2{x: self.xpos, y: 300.0}, // centerpoint
            100.0, // radius
            0.1, // how tightly the edge must adhere to the radius
            graphics::WHITE, // colour
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = &mut State {xpos: 200.0, direction: 1};
    let cb = ggez::ContextBuilder::new("generative_art", "Dante Falzone");
    let (ref mut ctx, ref mut event_loop) = &mut cb.build().unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
