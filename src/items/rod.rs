use std::sync::LazyLock;

use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
};
use serde::Deserialize;

use crate::items::Item;

static ROD_ICON: &str = "(";
static ROD_STRING: &str = "`\\";

pub static RODS: LazyLock<Vec<Rod>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("rod.json")).expect("Error deserializing rods!")
});

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Rod {
    pub name: String,
    pub value: i32,
    pub color: Color,
    /// Multiplier used for calculating lure time - the higher this is the faster fish will bite
    pub lure_mult: f32,
    /// Multiplier used for calculating bite time - the higher this is the longer fish will stay on
    /// the hook
    pub hook_strength: f32,
}

impl Item for Rod {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn value(&self) -> i32 {
        self.value
    }

    fn info(&self) -> String {
        format!("Lure: {} | Hook: {}", self.lure_mult, self.hook_strength)
    }

    fn icon(&self) -> Vec<Span<'_>> {
        vec![
            Span::styled(ROD_ICON, Style::new().fg(self.color)),
            ROD_STRING.into(),
        ]
    }
}

impl Rod {
    pub fn equipped_lines(&self) -> Text<'_> {
        Text::from({
            let mut vec = self.icon();
            vec.extend([
                " ".into(),
                Span::styled(self.name.clone(), Style::new().fg(self.color)),
            ]);
            let l1 = Line::from(vec).bold().underlined();

            let l2 = Line::from(format!(
                "Lure: {} | Hook: {}",
                self.lure_mult, self.hook_strength
            ));

            vec![l1, l2]
        })
    }
}
