use crate::cake::Cake;
use rand::{seq::SliceRandom, Rng};

const STRENGTHS: [&str; 3] = ["weak", "normal", "strong"];
const IMAGE_TYPES: [&str; 4] = ["dinosaur", "dog", "pixel_dog-ish", "squid"];
const NUMBER_OF_TOMBSTONE_VARIATIONS: i8 = 3;

const BASE_MAX_BOREDOM: u8 = 100;
const BASE_MAX_HUNGER: u8 = 100;
const BASE_MAX_HEALTH: u16 = 100;
const BASE_MAX_SOUNDS: u8 = 5;

// Note that the boredom limit is determined randomly in the default implementation of Pet.
const BASE_HUNGER_LIMIT: u8 = 80;
const BASE_ANGER_LIMIT: u8 = 90;

const BASE_BOREDOM_RATE: u8 = 4;
const BASE_HUNGER_RATE: u8 = 4;

#[derive(Debug)]
pub struct Pet {
    name: String,
    strength: String,
    image_type: String,
    tombstone_type: String,

    max_boredom: u8,
    max_hunger: u8,
    max_health: u16,
    max_sounds: u8,

    boredom: u8,
    hunger: u8,
    health: u16,
    sounds: Vec<String>,

    boredom_limit: u8,
    hunger_limit: u8,
    anger_limit: u8,

    boredom_rate: u8,
    hunger_rate: u8,

    ticks_survived: u16,
    reason_for_death: String,
}

impl Default for Pet {
    fn default() -> Self {
        let strength: String = STRENGTHS.choose(&mut rand::thread_rng()).unwrap().to_string();
        let max_health: u16;
        let boredom_limit: u8;
        let hunger_rate: u8;

        match strength.as_str() {
            "weak" => {
                max_health = BASE_MAX_HEALTH / 2;
                boredom_limit = BASE_MAX_BOREDOM - 1;
                hunger_rate = BASE_HUNGER_RATE;
            }
            "strong" => {
                max_health = BASE_MAX_HEALTH * 2;
                boredom_limit = rand::thread_rng().gen_range(50..90);
                hunger_rate = BASE_HUNGER_RATE * 2;
            }
            _ => {
                max_health = BASE_MAX_HEALTH;
                boredom_limit = rand::thread_rng().gen_range(50..90);
                hunger_rate = BASE_HUNGER_RATE;
            }
        }

        Pet {
            name: String::new(),
            strength,
            image_type: IMAGE_TYPES.choose(&mut rand::thread_rng()).unwrap().to_string(),
            tombstone_type: rand::thread_rng().gen_range(1..=NUMBER_OF_TOMBSTONE_VARIATIONS).to_string(),

            max_boredom: BASE_MAX_BOREDOM,
            max_hunger: BASE_MAX_HUNGER,
            max_health,
            max_sounds: BASE_MAX_SOUNDS,

            boredom: 0,
            hunger: 0,
            health: max_health,
            sounds: Vec::new(),

            boredom_limit,
            hunger_limit: BASE_HUNGER_LIMIT,
            anger_limit: BASE_ANGER_LIMIT,

            boredom_rate: BASE_BOREDOM_RATE,
            hunger_rate,

            ticks_survived: 0,
            reason_for_death: String::new(),
        }
    }
}

impl Pet {
    pub fn eat(&mut self, cake: &Cake) {
        self.hunger = u8::try_from(std::cmp::max(0, usize::from(self.hunger) - cake.get_hunger_replenished())).ok().unwrap();
        self.health = u16::try_from(std::cmp::min(usize::from(self.max_health), usize::from(self.health) + cake.get_health_replenished())).ok().unwrap();
    }
}