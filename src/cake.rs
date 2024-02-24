use infinitable::Infinitable;

#[derive(Debug)]
pub struct Cake {
    name: String,
    hunger_replenished: Infinitable<u8>,
    health_replenished: Infinitable<u16>,
    cost: u16,
}

impl Cake {
    pub fn new(
        name: String,
        hunger_replenished: Infinitable<u8>,
        health_replenished: Infinitable<u16>,
        cost: u16,
    ) -> Cake {
        Cake {
            name,
            hunger_replenished,
            health_replenished,
            cost,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_hunger_replenished(&self) -> Infinitable<u8> {
        self.hunger_replenished
    }

    pub fn get_health_replenished(&self) -> Infinitable<u16> {
        self.health_replenished
    }

    pub fn get_cost(&self) -> u16 {
        self.cost
    }
}
