use miniquad::*;
mod matrix;

#[repr(C)]
struct Vec2(f32, f32);

fn float_to_4_u8s(data: f32) -> Vec<u8> {
	let data = data.to_bits();
	(0..4)
		.map(|i| ((data >> (8 * i)) % 256) as u8)
		.rev()
		.collect()
}

fn verify() -> Vec<f32> {
	(0..(TEST_MATRIX.len() / TEST_INPUT.len()))
		.map(|i| {
			(0..TEST_INPUT.len())
				.map(|j| TEST_INPUT[j] * TEST_MATRIX[i * TEST_INPUT.len() + j])
				.sum()
		})
		.collect()
}

#[allow(clippy::excessive_precision)]
const TEST_INPUT: [f32; 16] = [
	0.08822412071490415,
	0.19363814098049603,
	0.4671973140208562,
	0.6046823830357189,
	0.3504339708582983,
	0.11242470296348483,
	0.9610872514505454,
	0.866937611907573,
	0.08822412071490415,
	0.6838678089284516,
	0.19363814098049603,
	0.4671973140208562,
	0.6046823830357189,
	0.3504339708582983,
	0.11242470296348483,
	0.9133379199459934,
];

#[allow(clippy::excessive_precision)]
const TEST_MATRIX: [f32; 64] = [
	0.08822412071490415,
	0.4279725868729074,
	0.3858888181523724,
	0.42543534515580705,
	0.3733061621359317,
	0.08003915241373138,
	0.6838678089284516,
	0.6351788270245093,
	0.09029006806890472,
	0.4711211727115543,
	0.14455617997093462,
	0.8857592877215551,
	0.19363814098049603,
	0.4671973140208562,
	0.7816316283967847,
	0.5544995005161097,
	0.5202063451800572,
	0.09182578265329489,
	0.31537650192800437,
	0.3004754865902629,
	0.6263231852174785,
	0.6017543004252598,
	0.453683483584456,
	0.4279725868729074,
	0.3858888181523724,
	0.42543534515580705,
	0.3733061621359317,
	0.08003915241373138,
	0.6838678089284516,
	0.6351788270245093,
	0.09029006806890472,
	0.4711211727115543,
	0.14455617997093462,
	0.8857592877215551,
	0.19363814098049603,
	0.4671973140208562,
	0.7816316283967847,
	0.5544995005161097,
	0.7351642433952214,
	0.37382023409928067,
	0.3534232594670118,
	0.7507014747787965,
	0.5131125669181578,
	0.6499617065018768,
	0.1707236320124339,
	0.6838678089284516,
	0.6351788270245093,
	0.09029006806890472,
	0.4711211727115543,
	0.14455617997093462,
	0.8857592877215551,
	0.3811522342570507,
	0.7092751335472234,
	0.48838757841796465,
	0.09919861095497307,
	0.4757039943579817,
	0.9762177754226935,
	0.4248474731773134,
	0.6046823830357189,
	0.3504339708582983,
	0.11242470296348483,
	0.9610872514505454,
	0.866937611907573,
	0.9133379199459934,
];

#[repr(C)]
struct Uniform {
	input_neuron_count: u32,
	output_neuron_count: u32,
}

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

		let mut pixels = Vec::<u8>::new();
		TEST_MATRIX.map(|x| {
			let parts = float_to_4_u8s(x);
			parts.iter().for_each(|x| pixels.push(*x))
		});
		let test_matrix = Texture::from_data_and_format(
			ctx,
			&pixels,
			TextureParams {
				width: TEST_INPUT.len() as _,
				height: (TEST_MATRIX.len() / TEST_INPUT.len()) as _,
				filter: FilterMode::Nearest,
				..TextureParams::default()
			},
		);

		let mut pixels = Vec::<u8>::new();
		TEST_INPUT.map(|x| {
			let parts = float_to_4_u8s(x);
			parts.iter().for_each(|x| pixels.push(*x))
		});
		let test_input = Texture::from_data_and_format(
			ctx,
			&pixels,
			TextureParams {
				width: TEST_INPUT.len() as _,
				height: 1,
				filter: FilterMode::Nearest,
				..TextureParams::default()
			},
		);

		let bindings = Bindings {
			vertex_buffers: vec![vertex_buffer],
			index_buffer,
			images: vec![test_input, test_matrix],
		};

		use std::fs;
		let shader = Shader::new(
			ctx,
			&fs::read_to_string("shaders/vert.glsl").unwrap(),
			&fs::read_to_string("shaders/frag.glsl").unwrap(),
			ShaderMeta {
				images: vec!["input_data".to_string(), "weights".to_string()],
				uniforms: UniformBlockLayout {
					uniforms: vec![
						UniformDesc::new("input_neuron_count", UniformType::Int1),
						UniformDesc::new("output_neuron_count", UniformType::Int1),
					],
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
		ctx.begin_default_pass(Default::default());

		ctx.apply_pipeline(&self.pipeline);
		ctx.apply_bindings(&self.bindings);
		ctx.apply_uniforms(&Uniform {
			input_neuron_count: TEST_INPUT.len() as _,
			output_neuron_count: (TEST_MATRIX.len() / TEST_INPUT.len()) as _,
		});
		ctx.draw(0, 6, 1);
		ctx.end_render_pass();

		ctx.commit_frame();
	}
}

fn main() {
	println!("{:?}", verify());
	miniquad::start(conf::Conf::default(), |ctx| Box::new(Stage::new(ctx)));
}
