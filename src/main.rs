use fifty_tone::generate_random_sequence;
use std::collections::HashMap;

fn main() {
    let fifty_tones = vec!["あ", "い", "う", "え", "お"];
    let keys_tones: HashMap<usize, &&str> = (0..fifty_tones.len()).zip(fifty_tones.iter()).collect();
    generate_random_sequence(&keys_tones);
}

