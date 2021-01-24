extern crate wasm_bindgen;
extern crate web_sys;

#[macro_use]
extern crate lazy_static;

use std::cmp;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod app_state;
mod constants;

use constants::*;
use std::f64::consts::TAU;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

#[wasm_bindgen]
pub struct AppClient {
	canvas: HtmlCanvasElement,
	ctx: CanvasRenderingContext2d,

	circle_center_x: f64,
	circle_center_y: f64,
	circle_radius: f64,

	multiplicator: f64,
	number_of_points: u32,
}

#[wasm_bindgen]
impl AppClient {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		console_error_panic_hook::set_once();

		let canvas: HtmlCanvasElement = web_sys::window()
			.unwrap()
			.document()
			.unwrap()
			.query_selector("#app-canvas")
			.unwrap()
			.unwrap()
			.dyn_into()
			.unwrap();

		let ctx: CanvasRenderingContext2d = canvas
			.get_context("2d")
			.unwrap()
			.unwrap()
			.dyn_into()
			.unwrap();

		Self {
			canvas,
			ctx,
			circle_center_x: 0.0,
			circle_center_y: 0.0,
			circle_radius: 0.0,
			multiplicator: 3.,
			number_of_points: 60,
		}
	}

	#[wasm_bindgen]
	pub fn setup(&mut self) -> Result<(), JsValue> {
		Ok(())
	}

	pub fn update(
		&mut self,
		dt: f64,
		canvas_width: f32,
		canvas_height: f32,
	) -> Result<(), JsValue> {
		// log_str(format!("Elapsed time : {}", dt));

		// app_state::update_app_state(canvas_width, canvas_height);

		let circle_radius = canvas_width.min(canvas_height) / 2. - CIRCLE_PADDING;
		let circle_center_x = canvas_width / 2.;
		let circle_center_y = canvas_height / 2.;

		self.circle_center_x = circle_center_x as f64;
		self.circle_center_y = circle_center_y as f64;
		self.circle_radius = circle_radius as f64;

		self.multiplicator += 0.1 * dt;

		Ok(())
	}

	pub fn render(&mut self) -> Result<(), JsValue> {
		self.background("#000000");

		self.ctx.set_stroke_style(&JsValue::from("#FFFFFF"));
		self.ctx.set_line_width(2.);
		self.circle_stroke(
			self.circle_center_x,
			self.circle_center_y,
			self.circle_radius,
		)?;

		for i in 0..self.number_of_points {
			let angle = (i as f64) * (TAU / (self.number_of_points as f64));
			let x1 = self.circle_radius * angle.cos() + self.circle_center_x;
			let y1 = self.circle_radius * angle.sin() + self.circle_center_y;

			let angle = (((i as f64) * self.multiplicator) % (self.number_of_points as f64))
				* (TAU / (self.number_of_points as f64));
			let x2 = self.circle_radius * angle.cos() + self.circle_center_x;
			let y2 = self.circle_radius * angle.sin() + self.circle_center_y;

			self.ctx.begin_path();
			self.ctx.move_to(x1, y1);
			self.ctx.line_to(x2, y2);
			self.ctx.stroke();
		}

		Ok(())
	}

	pub fn background(&mut self, color: &str) {
		self.ctx.set_fill_style(&JsValue::from(color));
		self.ctx.fill_rect(
			0.0,
			0.0,
			self.canvas.width() as f64,
			self.canvas.height() as f64,
		);
	}

	pub fn circle_stroke(&mut self, cx: f64, cy: f64, r: f64) -> Result<(), JsValue> {
		self.ctx.begin_path();
		self.ctx.arc(cx, cy, r, 0., TAU)?;
		self.ctx.stroke();

		Ok(())
	}
}
