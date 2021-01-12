use druid::piet::Color;
use druid::{Env, Key};

pub const PAINTR_TOGGLE_ON: Key<Color> = Key::new("paintr_toggle_on");
pub const PAINTR_TOGGLE_OFF: Key<Color> = Key::new("paintr_toggle_off");
pub const PAINTR_TOGGLE_FOREGROND: Key<Color> = Key::new("paintr_toggle_foreground");

pub fn init(env: &mut Env) {
    env.set(PAINTR_TOGGLE_FOREGROND, Color::rgb(0.0, 0.0, 0.0));
    env.set(PAINTR_TOGGLE_ON, Color::rgb(0.3, 0.3, 0.3));
    env.set(PAINTR_TOGGLE_OFF, Color::rgb(0.8, 0.8, 0.8));
}
