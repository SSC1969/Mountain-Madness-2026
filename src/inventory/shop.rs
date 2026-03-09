use std::collections::HashMap;

use ratatui::widgets::ListState;

use crate::{
    inventory::Inventory,
    items::{Item, ItemTypes, rod::RODS},
};

pub struct Shop {
    pub available_inventory: Vec<ItemTypes>,
    pub state: ListState,
    pub ui_index_map: HashMap<usize, usize>,
    // eventually add a hashmap here of conditions to items
    // (or vecs of items) to allow for unlocking new shop stock
    // based on progression
}

impl Default for Shop {
    fn default() -> Self {
        let available = vec![
            ItemTypes::Rod(RODS[1].clone()),
            ItemTypes::Rod(RODS[2].clone()),
        ];
        Self {
            available_inventory: available,
            state: ListState::default(),
            ui_index_map: HashMap::default(),
        }
    }
}

impl Shop {
    pub fn get_available(&self) -> Vec<ItemTypes> {
        self.available_inventory.clone()
    }

    pub fn sell_item(&mut self, index: usize, current_balance: i32) -> Option<ItemTypes> {
        if current_balance > self.available_inventory[index].value() {
            Some(self.available_inventory.remove(index))
        } else {
            None
        }
    }
}

impl Inventory for Shop {
    fn add_item(&mut self, item: ItemTypes) {
        self.available_inventory.push(item);
    }

    fn remove_item(&mut self, item: ItemTypes) {
        self.available_inventory.retain(|x| *x != item);
    }
}
