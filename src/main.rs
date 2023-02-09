use miniquad::*;

#[repr(C)]
struct Vec2(f32, f32);

fn float_to_4_u8s(data: f32) -> Vec<u8> {
	let data = data.to_bits();
	(0..4)
		.map(|i| ((data >> (8 * i)) % 256) as u8)
		.rev()
		.collect()
}

#[repr(C)]
struct Uniform(f32);

struct Stage {
	pipeline: Pipeline,
	bindings: Bindings,
}

impl Stage {
	pub fn new(ctx: &mut Context) -> Stage {
		let vertices = [Vec2(-1., -1.), Vec2(1., -1.), Vec2(1., 1.), Vec2(-1., 1.)];
		let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
		let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
		let index_buffer = Buffer::immutable::<u16>(ctx, BufferType::IndexBuffer, &indices);

		let bindings = Bindings {
			vertex_buffers: vec![vertex_buffer],
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
				uniforms: UniformBlockLayout {
					uniforms: vec![UniformDesc::new("some_float", UniformType::Float1)],
				},
			},
		)
		.unwrap();

		let pipeline = Pipeline::new(
			ctx,
			&[BufferLayout::default()],
			&[VertexAttribute::new("pos", VertexFormat::Float2)],
			shader,
		);

		Stage { pipeline, bindings }
	}
}

impl EventHandler for Stage {
	fn update(&mut self, _ctx: &mut Context) {}

	fn draw(&mut self, ctx: &mut Context) {
		static mut T: f32 = 0.;
		unsafe {
			T += 0.01;
			if T >= 1. {
				T = 0.
			}
		}
		ctx.begin_default_pass(Default::default());

		ctx.apply_pipeline(&self.pipeline);
		ctx.apply_bindings(&self.bindings);
		unsafe {
			ctx.apply_uniforms(&Uniform(T));
		};
		ctx.draw(0, 6, 1);
		ctx.end_render_pass();

		ctx.commit_frame();
	}
}

fn main() {
	miniquad::start(conf::Conf::default(), |ctx| Box::new(Stage::new(ctx)));
}
