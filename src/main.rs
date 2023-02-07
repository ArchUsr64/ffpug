use miniquad::{conf, Context, EventHandler};

struct Stage {}

impl Stage {
	fn new() -> Self {
		Stage {}
	}
}

impl EventHandler for Stage {
	fn update(&mut self, _ctx: &mut Context) {}
	fn draw(&mut self, _ctx: &mut Context) {}
}

fn main() {
	miniquad::start(conf::Conf::default(), |_| Box::new(Stage::new()));
}
