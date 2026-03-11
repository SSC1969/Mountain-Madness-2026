static DEFAULT_FRAME: &str = r#"

　　　　　　　　　　　　　　　　　　　 ,
　　　　　　　　　　　　　　 　 　　,／ヽ
　　　　　　　　　　　　　 　 　 ,／　　　ヽ
　　　　 　　　　　  ∧＿∧　　 ,／　　　　　　ヽ
　　　　　 　　　　（ ´w｀）,／　 　　　　　　 ヽ
　　　　　　　 　　（　　つつ@　 　　　　 　　　　ヽ
　　　 　 　＿＿   ｜ ｜ |　　　 　　　　 　　　　  ヽ
　　　　　　|――|　（_＿）＿）　　　　 　　 　　　　　　ヽ
￣￣￣￣￣￣￣￣￣￣￣￣|　　　　　　　　 　　　 　   　o
／⌒＼／⌒＼／⌒＼／⌒＼|彡~ﾟ　゜~ ~。゜　~ ~　~ ~~　~ ~ ~ ~　~ ~　~ ~~　~゜~ ~。゜　~ ~　~ ~~　~゜~ ~。゜
"#;

static BITE_FRAME: &str = r#"

　　　　　　　　　　　　　　　　　　　 ,
　　　　　　　　　　　　　　 　 　　 ,／ヽ
　　　　　　　　　　　　　 　 　  ,／　　\
　　　　 　　　　　  ∧＿∧　　 ,／　　　　ヽ        got a bite!
　　　　　 　　　　（ `o｀）,／　 　　　　ヽ
　　　　　　　 　　（　　つつ@　 　　　　 　ヽ
　　　 　 　＿＿   ｜ ｜ |　　　 　　　　 　　ヽ
　　　　　　|――|　（_＿）＿）　　　　 　　 　　ヽ
￣￣￣￣￣￣￣￣￣￣￣￣|　　　　　　　　 　　　` \' 　
／⌒＼／⌒＼／⌒＼／⌒＼|彡~ﾟ　゜~ ~。゜　~ ~　~ ~~　~ ~ `~ ~　~ ~　~ ~~　~゜~ ~。゜　~ ~　~ ~~　~゜~ ~。゜
"#;

// TODO: these two should be two frames of the same animation, instead of different events
static CATCH_FRAME: &str = r#"

　　　　　　　　　　　　　　　　　　 /,
　　　　　　　　　　　　　　 　 　,/  \
　　　　　　　　　　　　　 　 　,/　  ヽ     
　　　　 　　　　　  ∧＿∧　　,／　　　　ヽ,     _    woah!
　　　　　 　　　　（ ´o｀）,／　 　　　　　\_/   ヽ
　　　　　　　 　　（　　つつ@　 　　　　 　　　     ヽ _ ,
　　　 　 　＿＿   ｜ ｜ |　　　 　　　　 　　　         ヽo--
　　　　　　|――|　（_＿）＿）　　　　 　　 　　　　　
￣￣￣￣￣￣￣￣￣￣￣￣|　　　　　　　　 　　　 　  　
／⌒＼／⌒＼／⌒＼／⌒＼|彡~ﾟ　゜~ ~。゜　~ ~　~ ~~　~ ~ `~ ~　~ ~　~ ~~　~゜~ ~。゜　~ ~　~ ~~　~゜~ ~。゜
"#;

static CAUGHT_FRAME: &str = r#"

　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　 　
　　　　　　　　　　　　　 　   
　　　　 　　　　　  ∧＿∧　　
　　　　　 　　　　（ ´∀｀）             mmm...
　　　　　　　 　　 （　　_ つ _つ
　　　 　 　＿＿   ｜ ｜ |　　　 　
　　　　　　|――|　（_＿）＿）　　　　 　　 　　　　　
￣￣￣￣￣￣￣￣￣￣￣￣|　　　　　　　　 　　　 　  　
／⌒＼／⌒＼／⌒＼／⌒＼|彡~ﾟ　゜~ ~。゜　~ ~　~ ~~　~ ~ `~ ~　~ ~　~ ~~　~゜~ ~。゜　~ ~　~ ~~　~゜~ ~。゜
"#;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect, Spacing},
    style::{Color, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::{Line, Span},
    widgets::{
        Block, BorderType, Clear, List, ListItem, Padding, Paragraph, StatefulWidget, Tabs, Widget,
    },
};

use crate::{
    app::{Anim, App, InputMode, MENU_SIZE, Menu},
    inventory::dex::{DexEntries, DexEntry},
    items::{Item, ItemTypes},
};

impl Widget for &mut App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main, toolbar] = Layout::vertical([Constraint::Fill(1), Constraint::Max(3)])
            .spacing(Spacing::Overlap(1))
            .areas(area);
        let [viewport, menu] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(31)])
            .spacing(Spacing::Overlap(1))
            .areas(main);

        let [_, viewport_bottom] =
            Layout::vertical([Constraint::Fill(1), Constraint::Max(8)]).areas(viewport);

        let [_, chatbox] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Max(40)]).areas(viewport_bottom);

        let [messages, input] =
            Layout::vertical([Constraint::Fill(1), Constraint::Max(3)]).areas(chatbox);

        self.render_viewport(viewport, buf);
        self.render_menu(menu, buf);
        self.render_toolbar(toolbar, buf);

        // Render the name prompt if the player has no name; otherwise, render the chat
        if self.player.name == "" {
            self.show_name_prompt(area, buf);
        } else {
            self.render_messages(messages, buf);
            self.render_input(input, buf);
        }
    }
}

impl App {
    fn show_name_prompt(&mut self, area: Rect, buf: &mut Buffer) {
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);

        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 3,
        };
        Clear.render(popup_area, buf);
        let block = Block::bordered()
            .title("What's your name?")
            .border_type(BorderType::Rounded);

        Paragraph::new(self.input.value())
            .block(block)
            .style(Color::Yellow)
            .scroll((0, scroll as u16))
            .render(popup_area, buf);

        let x = self.input.visual_cursor().max(scroll) - scroll + 1;
        self.cursor_position = Some((area.x + x as u16, area.y + 1));
    }

    // move to bottom of impl
    fn render_input(&mut self, area: Rect, buf: &mut Buffer) {
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Color::Yellow.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered());
        input.render(area, buf);

        if self.input_mode == InputMode::Editing {
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            self.cursor_position = Some((area.x + x as u16, area.y + 1));
        }
    }

    fn render_messages(&self, area: Rect, buf: &mut Buffer) {
        let start = self.messages.len().saturating_sub(3);
        let messages = self.messages[start..].iter().map(String::as_str);
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        // multiple renders()'s so specify widget
        Widget::render(messages, area, buf);
    }

    fn render_viewport(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(format!(
                "Fishin' [{:?}, {} to bite, {} in bite, {} in anim]",
                self.player.fishing_state,
                self.player.ticks_until_next_bite,
                self.player.ticks_left_in_current_bite,
                self.player.catch_anim_timer
            ))
            .title_alignment(Alignment::Left)
            .title_top(
                Line::from(format!("${}", self.player.money))
                    .fg(Color::Green)
                    .bold()
                    .right_aligned(),
            )
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Exact);

        let inner = block.inner(area);

        let mut x = 0;
        let mut y = 0;
        let frame = match self.anim {
            Anim::DEFAULT => DEFAULT_FRAME,
            Anim::BITING => BITE_FRAME,
            Anim::CATCHING => {
                x = area.x + 63;
                y = area.y + 9;
                CATCH_FRAME
            }
            Anim::CAUGHT => {
                x = area.x + 31;
                y = area.y + 7;
                CAUGHT_FRAME
            }
        };

        Paragraph::new(frame).block(block).render(area, buf);

        if let Some(fish) = &self.recent_catch {
            let icon = Line::from(fish.icon());
            buf.set_line(x, y, &icon, icon.width() as u16);
        }

        // render toast, if applicable
        if self.toast.timer > 0 {
            let [toast_area, _] =
                Layout::vertical(vec![Constraint::Percentage(15), Constraint::Fill(1)])
                    .areas(inner);
            let spans: Vec<_> = self
                .toast
                .message
                .iter()
                .map(|(p, s)| Span::styled(p, *s))
                .collect();
            Paragraph::new(Line::from(spans))
                .right_aligned()
                .bold()
                .render(toast_area, buf);
        }
    }

    fn render_toolbar(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Exact);

        let inner = block.inner(area).centered_horizontally(Constraint::Fill(1));
        let constraints = (0..MENU_SIZE).map(|_| Constraint::Ratio(1, MENU_SIZE as u32));
        let layout = Layout::horizontal(constraints)
            .flex(Flex::Center)
            .split(inner);

        Line::from("<b> Backpack").centered().render(layout[0], buf);
        Line::from("<p> Fincyclopedia")
            .centered()
            .render(layout[1], buf);
        Line::from("<m> Market").centered().render(layout[2], buf);
        Line::from("<o> Options").centered().render(layout[3], buf);

        block.render(area, buf);
    }

    fn render_menu(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(format!("{:?}", self.menu))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Exact)
            .padding(Padding::horizontal(1));

        match self.menu {
            Menu::Backpack => self.render_backpack(area, buf, block),
            Menu::Fincyclopedia => self.render_dex(area, buf, block),
            Menu::Market => self.render_shop(area, buf, block),
            Menu::Options => Paragraph::new("Options")
                .centered()
                .block(block)
                .render(area, buf),
        }
    }

    fn render_backpack(&mut self, area: Rect, buf: &mut Buffer, block: Block) {
        let items = self.player.backpack.items.clone();

        let inner = block.inner(area);
        let [tabs_area, content_area, controls_area] = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(inner);

        block.render(area, buf);

        Paragraph::new("⮁ move <s> sell ⏎ equip").render(controls_area, buf);

        let [centered_tabs] = Layout::horizontal(vec![Constraint::Length(13)])
            .flex(Flex::Center)
            .areas(tabs_area);
        Tabs::new(vec!["Fish", "Tools"])
            .bold()
            .select(self.menu_tab as usize)
            .render(centered_tabs, buf);

        // filter the items based on whether the player has fish or tools selected
        let mut new_items = Vec::new();
        let mut j = 0;
        for (i, item) in items.into_iter().enumerate() {
            match item {
                // add items to the ui list only if it matches the currently selected tab
                ItemTypes::Fish(_) => {
                    if self.menu_tab == 0 {
                        // add a mapping from the elements new index (in the UI list) to it's
                        // original index (in the player's backpack struct)
                        self.player.backpack_ui_map.insert(j, i);
                        j += 1;
                        new_items.push(item);
                    }
                }
                ItemTypes::Rod(_) => {
                    if self.menu_tab == 1 {
                        self.player.backpack_ui_map.insert(j, i);
                        j += 1;
                        new_items.push(item);
                    }
                }
            }
        }

        // highlight the currently equipped rod
        let list_items = new_items.iter().map(|item| {
            if let ItemTypes::Rod(rod) = item
                && *rod == self.player.equipped_rod
            {
                ListItem::from(rod.equipped_lines())
            } else {
                ListItem::from(item)
            }
        });

        let list = List::new(list_items).highlight_style(Style::new().reversed());
        StatefulWidget::render(list, content_area, buf, &mut self.backpack_state);
    }

    fn render_dex(&mut self, area: Rect, buf: &mut Buffer, block: Block) {
        let mut items = self.player.dex.get_all();

        let inner = block.inner(area);
        let [tabs_area, content_area, controls_area] = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(inner);

        block.render(area, buf);

        Paragraph::new("⮁ navigate").render(controls_area, buf);

        let [centered_tabs] = Layout::horizontal(vec![Constraint::Length(13)])
            .flex(Flex::Center)
            .areas(tabs_area);
        Tabs::new(vec!["Fish", "Tools"])
            .bold()
            .select(self.menu_tab as usize)
            .render(centered_tabs, buf);

        // filter the items based on whether the player has fish or tools selected
        items.retain(|item| match item {
            DexEntries::Fish(_) => self.menu_tab == 0,
            DexEntries::Rod(_) => self.menu_tab == 1,
        });

        let list_items = items
            .into_iter()
            .map(|entry| ListItem::from(entry.get_lines()));

        let list = List::new(list_items).highlight_style(Style::new().reversed());
        StatefulWidget::render(list, content_area, buf, &mut self.dex_state);
    }

    fn render_shop(&mut self, area: Rect, buf: &mut Buffer, block: Block) {
        let inner = block.inner(area);
        let [tabs_area, content_area, controls_area] = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(inner);
        block.render(area, buf);

        Paragraph::new("⮁ navigate, ⏎ buy").render(controls_area, buf);

        let items = self.shop.get_available();

        let [centered_tabs] = Layout::horizontal(vec![Constraint::Length(13)])
            .flex(Flex::Center)
            .areas(tabs_area);
        Tabs::new(vec!["Fish", "Tools"])
            .bold()
            .select(self.menu_tab as usize)
            .render(centered_tabs, buf);

        // filter the items based on whether the player has fish or tools selected
        let mut new_items = Vec::new();
        let mut j = 0;
        for (i, item) in items.into_iter().enumerate() {
            match item {
                // add items to the ui list only if it matches the currently selected tab
                ItemTypes::Fish(_) => {
                    if self.menu_tab == 0 {
                        // add a mapping from the elements new index (in the UI list) to it's
                        // original index (in the shop struct)
                        self.shop.ui_index_map.insert(j, i);
                        j += 1;
                        new_items.push(item);
                    }
                }
                ItemTypes::Rod(_) => {
                    if self.menu_tab == 1 {
                        self.shop.ui_index_map.insert(j, i);
                        j += 1;
                        new_items.push(item);
                    }
                }
            }
        }

        let list_items = new_items.iter().map(|item| ListItem::from(item));

        let list = List::new(list_items).highlight_style(Style::new().reversed());
        StatefulWidget::render(list, content_area, buf, &mut self.shop.state);
    }
}
