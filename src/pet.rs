use rand::{seq::SliceRandom, Rng};

const STRENGTHS: [&str; 3] = ["weak", "normal", "strong"];
const IMAGE_TYPES: [&str; 4] = ["dinosaur", "dog", "pixel_dog-ish", "squid"];
const NUMBER_OF_TOMBSTONE_VARIATIONS: i8 = 3;

const BASE_MAX_BOREDOM: i8 = 100;
const BASE_MAX_HUNGER: i8 = 100;
const BASE_MAX_HEALTH: i16 = 100;
const BASE_MAX_SOUNDS: i8 = 5;

// Note that the boredom limit is determined randomly in the default implementation of Pet.
const BASE_HUNGER_LIMIT: i8 = 80;
const BASE_ANGER_LIMIT: i8 = 90;

const BASE_BOREDOM_RATE: i8 = 4;
const BASE_HUNGER_RATE: i8 = 4;

#[derive(Debug)]
pub struct Pet {
    name: String,
    strength: String,
    image_type: String,
    tombstone_type: String,

    max_boredom: i8,
    max_hunger: i8,
    max_health: i16,
    max_sounds: i8,

    boredom: i8,
    hunger: i8,
    health: i16,
    sounds: Vec<String>,

    boredom_limit: i8,
    hunger_limit: i8,
    anger_limit: i8,

    boredom_rate: i8,
    hunger_rate: i8,

    ticks_survived: i16,
    reason_for_death: String,
}

impl Default for Pet {
    fn default() -> Self {
        let strength: String = STRENGTHS.choose(&mut rand::thread_rng()).unwrap().to_string();
        let max_health: i16;
        let boredom_limit: i8;
        let hunger_rate: i8;

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
