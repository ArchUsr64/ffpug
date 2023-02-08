use miniquad::*;

struct Stage {
	pipeline: Pipeline,
	bindings: Bindings,
}

impl Stage {
	pub fn new(ctx: &mut Context) -> Stage {
		let index_buffer = Buffer::immutable::<u16>(ctx, BufferType::IndexBuffer, &[]);

		let bindings = Bindings {
			vertex_buffers: vec![],
			index_buffer,
			images: vec![],
		};

		use std::fs;
		let shader = Shader::new(
			ctx,
			&fs::read_to_string("shaders/vert.glsl").unwrap(),
			&fs::read_to_string("shaders/frag.glsl").unwrap(),
			ShaderMeta {
				images: vec![],
				uniforms: UniformBlockLayout { uniforms: vec![] },
			},
		)
		.unwrap();

		let pipeline = Pipeline::new(ctx, &[BufferLayout::default()], &[], shader);

		Stage { pipeline, bindings }
	}
}

impl EventHandler for Stage {
	fn update(&mut self, _ctx: &mut Context) {}

	fn draw(&mut self, ctx: &mut Context) {
		ctx.begin_default_pass(Default::default());

		ctx.apply_pipeline(&self.pipeline);
		ctx.apply_bindings(&self.bindings);
		ctx.end_render_pass();

		ctx.commit_frame();
	}
}

fn main() {
	miniquad::start(conf::Conf::default(), |ctx| {
		Box::new(Stage::new(ctx))
	});
}
