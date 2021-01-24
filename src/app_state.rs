use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
	static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub struct AppState {
	pub canvas_width: f32,
	pub canvas_height: f32,
	pub frame_count: u128,
}

impl AppState {
	pub fn new() -> Self {
		AppState {
			canvas_width: 0.0,
			canvas_height: 0.0,
			frame_count: 0,
		}
	}
}

pub fn get_curr_state() -> Arc<AppState> {
	APP_STATE.lock().unwrap().clone()
}

pub fn update_app_state(canvas_width: f32, canvas_height: f32) {
	let mut data = APP_STATE.lock().unwrap();
	let frame_count = data.frame_count + 1;

	*data = Arc::new(AppState {
		canvas_width,
		canvas_height,
		frame_count,

		..*data.clone()
	});
}
