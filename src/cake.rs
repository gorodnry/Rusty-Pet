#[derive(Debug)]
pub struct Cake {
    name: String,
    hunger_replenished: usize,
    health_replenished: usize,
    cost: u8
}

impl Cake {
    pub fn new(name: String, hunger_replenished: usize, health_replenished: usize, cost: u8) -> Cake {
        Cake { name, hunger_replenished, health_replenished, cost }
    }
    
    pub fn get_name(&self) -> &String {
        &self.name
    }
    
    pub fn get_hunger_replenished(&self) -> usize {
        self.hunger_replenished
    }
    
    pub fn get_health_replenished(&self) -> usize {
        self.health_replenished
    }
    
    pub fn get_cost(&self) -> u8 {
        self.cost
    }
}
