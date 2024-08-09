use std::time::Duration;
use slint::Timer;

slint::include_modules!();

const ANIMATION_MS: u64 = 1000;

fn main() {
	let window = Root::new().unwrap();
	window.set_animation_len(ANIMATION_MS as i64);

	let window_weak = window.as_weak();
	let sleep_duration = Duration::from_millis(ANIMATION_MS);
	window.on_stack_clicked(move || {
		let interface = window_weak.unwrap();
		interface.set_show_animation(true);
		interface.set_animation_on_target(true);

		Timer::single_shot(sleep_duration, move || {
			interface.set_show_animation(false);
			interface.set_animation_on_target(false);
		});
	});

	window.run().unwrap();
}
