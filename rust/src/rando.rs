extern crate rand as _rand;
use rando::_rand::Rng;
use rando::_rand::seq::SliceRandom;

pub fn rand() -> f32 {
    _rand::thread_rng().gen_range(0.0..1.0)
}

pub fn randu8() -> u8 {
    _rand::thread_rng().gen_range(0..255)
}

pub fn randu8f() -> f32 {
    _rand::thread_rng().gen_range(0..255) as f32
}

pub fn rand_color_adjust(c: f32, range: f32) -> f32 {
    (c + ((rand() - 0.5) * 256.0 * range)).min(255.0).max(0.0)
}

pub fn rand_adjust(p: f32, range: f32, min: f32, max: f32) -> f32 {
    (p + ((rand() - 0.5) * range)).min(max).max(min)
}

pub fn choose<T>(v: &mut Vec<T>) -> Option<&mut T> {
    let mut rng = _rand::thread_rng();
    v.choose_mut(&mut rng)
}