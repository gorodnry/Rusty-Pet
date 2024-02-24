use crate::cake::Cake;
use rand::{seq::SliceRandom, Rng};

const STRENGTHS: [Strength; 3] = [Strength::Weak, Strength::Normal, Strength::Strong];
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

enum Strength {
    Weak,
    Normal,
    Strong,
}

#[derive(Debug)]
pub struct Pet {
    name: String,
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
        let strength: &Strength = STRENGTHS.choose(&mut rand::thread_rng()).unwrap();
        let max_health: u16;
        let boredom_limit: u8;
        let hunger_rate: u8;

        match strength {
            Strength::Weak => {
                max_health = BASE_MAX_HEALTH / 2;
                boredom_limit = BASE_MAX_BOREDOM - 1;
                hunger_rate = BASE_HUNGER_RATE;
            }
            Strength::Normal => {
                max_health = BASE_MAX_HEALTH;
                boredom_limit = rand::thread_rng().gen_range(50..90);
                hunger_rate = BASE_HUNGER_RATE;
            }
            Strength::Strong => {
                max_health = BASE_MAX_HEALTH * 2;
                boredom_limit = rand::thread_rng().gen_range(50..90);
                hunger_rate = BASE_HUNGER_RATE * 2;
            }
        }

        Pet {
            name: String::new(),
            image_type: IMAGE_TYPES
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            tombstone_type: rand::thread_rng()
                .gen_range(1..=NUMBER_OF_TOMBSTONE_VARIATIONS)
                .to_string(),

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
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_max_boredom(&self) -> u8 {
        self.max_boredom
    }

    pub fn get_max_hunger(&self) -> u8 {
        self.max_hunger
    }

    pub fn get_max_health(&self) -> u16 {
        self.max_health
    }

    pub fn get_reason_for_death(self) -> String {
        if self.health != 0 {
            return String::from("I'm still alive :eye:");
        }

        self.reason_for_death
    }

    pub fn change_name(&mut self, new_name: String) {
        if !new_name.trim().is_empty() {
            self.name = new_name;
        }
    }

    pub fn get_status_report(&self) -> Status {
        let boredom_status_message;
        let hunger_status_message;
        let health_status_message;

        if self.boredom <= self.boredom_limit {
            boredom_status_message = String::from("happy");
        } else if self.boredom < self.anger_limit {
            boredom_status_message = String::from("bored");
        } else {
            boredom_status_message = String::from("angry");
        }

        if self.hunger < self.max_hunger / 2 {
            hunger_status_message = String::from("full");
        } else if self.hunger <= self.hunger_limit {
            hunger_status_message = String::from("hungry");
        } else {
            hunger_status_message = String::from("starving");
        }

        if f32::from(self.health) >= f32::from(self.max_health) * 0.8 {
            health_status_message = String::from("fighting fit");
        } else if f32::from(self.health) >= f32::from(self.max_health) * 0.2 {
            health_status_message = String::from("ok");
        } else if self.health >= 1 {
            health_status_message = String::from("sick");
        } else {
            health_status_message = String::from("dead");
        }

        Status {
            boredom: self.boredom,
            boredom_percentage: (self.boredom / self.max_boredom) * 100,
            boredom_status_message,
            hunger: self.hunger,
            hunger_percentage: (self.hunger / self.max_hunger) * 100,
            hunger_status_message,
            health: self.health,
            health_percentage: u8::try_from((self.health / self.max_health) * 100).unwrap(),
            health_status_message,
        }
    }

    fn get_current_boredom_rate(&self) -> u8 {
        if self.boredom > self.boredom_limit {
            return self.boredom_rate * 2;
        }

        self.boredom_rate
    }

    fn get_current_hunger_rate(&self) -> u8 {
        let mut rate: u8 = self.hunger_rate;

        if self.health <= self.max_health / 4 {
            rate /= 2;
        }

        if self.boredom > self.boredom_limit {
            rate *= 2;

            if self.boredom > self.anger_limit {
                rate *= 2;
            }
        }

        rate
    }

    pub fn eat(&mut self, cake: &Cake) {
        self.hunger = u8::try_from(std::cmp::max(
            0,
            usize::from(self.hunger) - cake.get_hunger_replenished(),
        ))
        .ok()
        .unwrap();
        self.health = u16::try_from(std::cmp::min(
            usize::from(self.max_health),
            usize::from(self.health) + cake.get_health_replenished(),
        ))
        .ok()
        .unwrap();
    }
}

pub struct Status {
    boredom: u8,
    boredom_percentage: u8,
    boredom_status_message: String,
    hunger: u8,
    hunger_percentage: u8,
    hunger_status_message: String,
    health: u16,
    health_percentage: u8,
    health_status_message: String,
}
