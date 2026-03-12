use rand::RngExt;
use serde::{Deserialize, Serialize};

use crate::inventory::Inventory;
use crate::inventory::{backpack::Backpack, dex::Dex};
use crate::items::{
    Item, ItemTypes,
    fish::Fish,
    rod::{RODS, Rod},
};

#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum FishingState {
    #[default]
    Idle,
    Biting,
    Catching,
    Caught,
}

#[derive(Deserialize, Serialize)]
pub struct Player {
    pub name: String,
    pub backpack: Backpack,
    pub dex: Dex,
    pub money: i32,

    // fish catching variables
    pub equipped_rod: Rod,
    #[serde(skip)]
    pub fishing_state: FishingState,

    #[serde(skip)]
    pub ticks_until_next_bite: u32,
    #[serde(skip)]
    pub ticks_left_in_current_bite: u32,
    #[serde(skip)]
    pub catch_anim_timer: u32,
}

impl Player {
    pub fn catch_fish(&mut self) -> Fish {
        let fish = Fish::generate();

        self.add_item(ItemTypes::Fish(fish.clone()));

        // 60 tick (2 seconds) timer for the animation to play
        self.fishing_state = FishingState::Catching;
        self.catch_anim_timer = 25;

        fish
    }

    // called after the animation timer for the catch has ended to play the second animation
    pub fn post_catch(&mut self) {
        self.fishing_state = FishingState::Caught;
        self.catch_anim_timer = 60;
    }

    /// Adds an item to the inventory and registers it to the dex
    pub fn add_item(&mut self, item: ItemTypes) {
        self.backpack.add_item(item.clone());
        self.dex.add_item(item);
    }

    /// Updates any relevant counters the player struct uses
    pub fn tick(&mut self) {
        match self.fishing_state {
            FishingState::Idle => {
                if self.ticks_until_next_bite <= 0 {
                    self.bite();
                } else {
                    self.ticks_until_next_bite -= 1;
                }
            }
            FishingState::Biting => {
                // fail fishing if the player missed the time window
                if self.ticks_left_in_current_bite <= 0 {
                    self.cast_rod();
                } else {
                    self.ticks_left_in_current_bite -= 1;
                }
            }
            FishingState::Catching => {
                if self.catch_anim_timer <= 0 {
                    self.post_catch();
                } else {
                    self.catch_anim_timer -= 1;
                }
            }
            FishingState::Caught => {
                if self.catch_anim_timer <= 0 {
                    // cast the rod again after completing the animation
                    self.cast_rod();
                } else {
                    self.catch_anim_timer -= 1;
                }
            }
        }
    }

    /// Called to 'cast' the player's fishing rod
    pub fn cast_rod(&mut self) {
        let mut rng = rand::rng();
        self.fishing_state = FishingState::Idle;

        // 2 seconds <-> 10 seconds base rate
        self.ticks_until_next_bite =
            (rng.random_range(60.0..300.0) * (100.0 / self.equipped_rod.lure_mult)) as u32;
    }

    /// Called to update the player to have a fish biting
    pub fn bite(&mut self) {
        let mut rng = rand::rng();
        self.fishing_state = FishingState::Biting;

        // 2 seconds <-> 5 seconds base rate
        self.ticks_left_in_current_bite =
            (rng.random_range(60.0..150.0) * (self.equipped_rod.hook_strength / 100.0)) as u32;
    }

    pub fn equip(&mut self, rod: Rod) {
        self.equipped_rod = rod;
    }

    pub fn sell(&mut self, index: usize) {
        match &self.backpack.items[index] {
            ItemTypes::Rod(_) => {
                // disallow selling rods
                return;
            }
            _ => {}
        }
        self.money += self.backpack.items[index].value();
        self.backpack.items.remove(index);
    }
}

impl Default for Player {
    fn default() -> Self {
        let mut backpack = Backpack::default();
        let rod = &RODS[0];
        // Add any items the player should start with here
        backpack.add_item(ItemTypes::Rod(rod.clone()));
        Self {
            name: "".to_string(),
            backpack,
            dex: Dex::default(),
            money: 0,
            equipped_rod: rod.clone(),
            fishing_state: FishingState::default(),
            ticks_until_next_bite: 240,
            ticks_left_in_current_bite: 0,
            catch_anim_timer: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catch_fish() {
        let mut p = Player::default();
        p.catch_fish();
        // should be two, as by default the player will have one item (the average rod)
        assert_eq!(p.backpack.get_all().len(), 2);
    }
}
