use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span, Text},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

use crate::items::{fish::Fish, rod::Rod};

pub mod fish;
pub mod rod;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemTypes {
    Fish(Fish),
    Rod(Rod),
}

pub trait Item {
    fn name(&self) -> String;
    fn value(&self) -> i32;
    fn info(&self) -> String;
    fn icon(&self) -> Vec<Span<'_>>;
}

impl<'a> From<&'a ItemTypes> for ListItem<'a> {
    fn from(item: &'a ItemTypes) -> ListItem<'a> {
        let mut text = item.icon();
        text.extend(vec![
            " ".into(),
            item.name().into(),
            " ".into(),
            Span::from(format!("${}", item.value()).fg(Color::Green)),
        ]);
        let line1 = Line::from(text).bold().underlined();
        let line2 = Line::from(item.info());

        ListItem::new(Text::from(vec![line1, line2]))
    }
}

impl Item for ItemTypes {
    fn name(&self) -> String {
        match self {
            ItemTypes::Fish(fish) => fish.name(),
            ItemTypes::Rod(rod) => rod.name(),
        }
    }
    fn icon(&self) -> Vec<Span<'_>> {
        match self {
            ItemTypes::Fish(fish) => fish.icon(),
            ItemTypes::Rod(rod) => rod.icon(),
        }
    }
    fn info(&self) -> String {
        match self {
            ItemTypes::Fish(fish) => fish.info(),
            ItemTypes::Rod(rod) => rod.info(),
        }
    }
    fn value(&self) -> i32 {
        match self {
            ItemTypes::Fish(fish) => fish.value(),
            ItemTypes::Rod(rod) => rod.value(),
        }
    }
}
