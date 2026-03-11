use crate::{
    inventory::Inventory,
    items::{Item, ItemTypes},
};

#[derive(Default)]
pub struct Backpack {
    pub items: Vec<ItemTypes>,
}

impl Backpack {
    pub fn search(&self, name: String) -> Vec<ItemTypes> {
        self.items
            .iter()
            .filter(|item| item.name() == name)
            .cloned()
            .collect()
    }

    pub fn get_all(&self) -> Vec<ItemTypes> {
        self.items.clone()
    }
}

impl Inventory for Backpack {
    fn add_item(&mut self, item: ItemTypes) {
        self.items.push(item);
    }

    fn remove_item(&mut self, item: ItemTypes) {
        self.items.retain(|x| *x != item);
    }
}
