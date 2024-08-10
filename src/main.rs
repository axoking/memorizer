use std::time::Duration;
use slint::Timer;
use std::iter::repeat;

const ANIMATION_MS: u64 = 1000;
const CARD_TYPES: u8 = 32;
const CARD_COPIES: usize = 1;

slint::include_modules!();

struct CardStack {
	cards: Vec<u8>,
	count: usize
}

impl CardStack {
	fn new() -> Self {
		let init: Vec<u8> = (0u8..CARD_TYPES)
		.flat_map(|n| repeat(n).take(CARD_COPIES))
		.collect();

		Self {
			cards: init,
			count: CARD_COPIES * (CARD_TYPES as usize)
		}
	}


	fn draw(&mut self, rng: &mut impl rand::Rng) -> u8 {
		let index = rng.gen_range(0..self.count);
		self.count -= 1;
		self.cards.remove(index)
	}
}

fn draw_card(window: Root, cards: &mut CardStack, rng: &mut impl rand::Rng) {
	println!("Drew card {}", cards.draw(rng));
	window.set_show_animation(true);
	window.set_animation_on_target(true);

	let duration = Duration::from_millis(ANIMATION_MS);
	Timer::single_shot(duration, move || {
		window.set_show_animation(false);
		window.set_animation_on_target(false);
	});
}

fn main() {
	let mut cards = CardStack::new();
	let mut rng = rand::thread_rng();

	let window = Root::new().unwrap();
	window.set_animation_len(ANIMATION_MS as i64);

	let window_weak = window.as_weak();
	window.on_stack_clicked(move || {
		draw_card(window_weak.unwrap(), &mut cards, &mut rng);
	});

	window.run().unwrap();
}
