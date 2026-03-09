use crate::{
    chat::ChatHandler,
    event::{AppEvent, Event, EventHandler, NavigationDirection},
    inventory::shop::Shop,
    items::{Item, ItemTypes, fish::Fish},
    player::{FishingState, Player},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, widgets::ListState};
use tui_input::{Input, backend::crossterm::EventHandler as crosstermEventHandler};

#[derive(Clone, Default, Debug)]
pub enum Menu {
    #[default]
    Backpack,
    Fincyclopedia,
    Market,
    Options,
}

pub const MENU_SIZE: i32 = 4;

impl Menu {
    fn next(&self) -> Self {
        match self {
            Menu::Backpack => Menu::Fincyclopedia,
            Menu::Fincyclopedia => Menu::Market,
            Menu::Market => Menu::Options,
            Menu::Options => Menu::Backpack,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Menu::Backpack => Menu::Options,
            Menu::Fincyclopedia => Menu::Backpack,
            Menu::Market => Menu::Fincyclopedia,
            Menu::Options => Menu::Fincyclopedia,
        }
    }
}

pub enum Anim {
    DEFAULT,
    BITING,
    CATCHING,
    CAUGHT,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Default)]
pub struct Toast {
    pub message: String,
    pub timer: u32,
}

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// Chat handler
    pub chat: ChatHandler,
    /// Currently open menu
    pub menu: Menu,
    /// Currently selected menu tab (if applicable)
    pub menu_tab: u32,
    /// Player data struct
    pub player: Player,
    /// Backpack state for ui
    pub backpack_state: ListState,
    /// Dex state for ui
    pub dex_state: ListState,
    /// Struct used to display messages like notifications
    pub toast: Toast,

    /// Shop struct
    pub shop: Shop,

    pub input: Input,
    // Whether the chatbox is open or not
    pub input_mode: InputMode,
    // most recent n messages
    pub messages: Vec<String>,

    pub cursor_position: Option<(u16, u16)>,
    pub anim: Anim,
    pub recent_catch: Option<Fish>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let events = EventHandler::new();
        let event_tx = events.sender();

        Self {
            running: true,
            events,
            chat: ChatHandler::new(event_tx),
            menu: Menu::default(),
            menu_tab: 0,
            player: Player::default(),
            backpack_state: ListState::default(),
            dex_state: ListState::default(),
            toast: Toast::default(),

            shop: Shop::default(),

            input: Input::new(std::string::String::from("")),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            cursor_position: Option::Some((0, 0)),
            anim: Anim::DEFAULT,
            recent_catch: Option::None,
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    pub async fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next().await? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    self.handle_key_events(key_event)?
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
                AppEvent::ChangeMenu(menu) => self.menu = menu,
                AppEvent::Navigate(dir) => match dir {
                    NavigationDirection::Left => self.menu = self.menu.prev(),
                    NavigationDirection::Right => self.menu = self.menu.next(),
                    _ => {}
                },
                AppEvent::CastRod => self.player.cast_rod(),
                AppEvent::FishBiting => {
                    self.player.bite();
                    self.events.send(AppEvent::SendChat("biting...".to_owned()));
                }
                AppEvent::FishCatching => {
                    // this updates the player state as well as getting the caught fish's icon
                    self.recent_catch = Some(self.player.catch_fish());
                }
                AppEvent::FishCaught => self.player.post_catch(),
                AppEvent::ChangeRod(rod) => self.player.equip(rod),
                AppEvent::ShowToast(msg) => self.toast.start(msg),

                AppEvent::ChangePlayerName(name) => {
                    self.player.name = name.clone();
                    self.chat.update_name(name);
                }
                AppEvent::ChangeInputMode(im) => match im {
                    InputMode::Normal => self.input_mode = im,
                    InputMode::Editing => self.input_mode = im,
                },
                AppEvent::SendChat(msg) => self.chat.send(msg),
                AppEvent::MessageReceived(msg) => self.messages.push(msg),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if self.input_mode == InputMode::Editing {
            match key_event.code {
                KeyCode::Enter => {
                    let msg = self.input.value().to_string();
                    self.input.reset();
                    if self.player.name == "" {
                        self.events.send(AppEvent::ChangePlayerName(msg));
                        self.input_mode = InputMode::Normal;
                    } else {
                        self.messages.push(msg.clone());
                        self.events.send(AppEvent::SendChat(msg));
                    }
                }
                KeyCode::Esc => self
                    .events
                    .send(AppEvent::ChangeInputMode(InputMode::Normal)),
                _ => {
                    self.input
                        .handle_event(&crossterm::event::Event::Key(key_event));
                }
            }
            return Ok(());
        }
        if (self.player.fishing_state == FishingState::Biting)
            && (key_event.code == KeyCode::Char('f'))
        {
            self.events.send(AppEvent::FishCatching);
        }
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Char(' ') => self.events.send(AppEvent::FishBiting),
            KeyCode::Char('t') => self
                .events
                .send(AppEvent::ChangeInputMode(InputMode::Editing)),
            KeyCode::Char('m') => self.events.send(AppEvent::ChangeMenu(Menu::Market)),
            KeyCode::Char('p') => self.events.send(AppEvent::ChangeMenu(Menu::Fincyclopedia)),
            KeyCode::Char('b') => self.events.send(AppEvent::ChangeMenu(Menu::Backpack)),
            KeyCode::Char('o') => self.events.send(AppEvent::ChangeMenu(Menu::Options)),

            // Send any remaining events to the open menu for processing
            _ => self.handle_menu_key_events(key_event)?,
        }
        Ok(())
    }

    fn handle_menu_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match self.menu {
            Menu::Backpack => match key_event.code {
                KeyCode::Up => self.backpack_state.select_previous(),
                KeyCode::Down => self.backpack_state.select_next(),
                KeyCode::Left => self.menu_tab = (self.menu_tab + 1) % 2,
                KeyCode::Right => self.menu_tab = (self.menu_tab + 1) % 2,
                KeyCode::Enter => {
                    if let Some(index) = self.backpack_state.selected() {
                        match &self.player.backpack.items[index] {
                            ItemTypes::Rod(rod) => {
                                if self.player.equipped_rod != *rod {
                                    self.player.equip(rod.clone());
                                }
                            }
                            _ => {}
                        }
                    }
                }
                KeyCode::Char('s') => {
                    if let Some(index) = self.backpack_state.selected() {
                        // grab the translated index from the stored ui index map to use for finding the item in
                        // the backpack
                        if let Some(&new_index) = self.player.backpack_ui_map.get(&index) {
                            self.player.sell(new_index);
                        }
                    }
                }
                _ => {}
            },
            Menu::Fincyclopedia => match key_event.code {
                KeyCode::Up => self.dex_state.select_previous(),
                KeyCode::Down => self.dex_state.select_next(),
                KeyCode::Left => self.menu_tab = (self.menu_tab + 1) % 2,
                KeyCode::Right => self.menu_tab = (self.menu_tab + 1) % 2,

                _ => {}
            },
            Menu::Market => match key_event.code {
                KeyCode::Up => self.shop.state.select_previous(),
                KeyCode::Down => self.shop.state.select_next(),
                KeyCode::Left => self.menu_tab = (self.menu_tab + 1) % 2,
                KeyCode::Right => self.menu_tab = (self.menu_tab + 1) % 2,
                KeyCode::Enter => {
                    if let Some(index) = self.shop.state.selected() {
                        // grab the translated index from the stored ui index map to use for finding the item in
                        // the shop
                        if let Some(&new_index) = self.shop.ui_index_map.get(&index) {
                            self.buy_item(new_index);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    /// Handles purchasing an item from the shop based on it's true index
    fn buy_item(&mut self, index: usize) {
        if let Some(item) = self.shop.sell_item(index, self.player.money) {
            self.events
                .send(AppEvent::ShowToast(format!("Purchased {}", item.name())));
            self.player.add_item(item);
        } else {
            self.events
                .send(AppEvent::ShowToast("Can't afford that!".to_string()));
        }
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        self.player.tick();
        self.toast.tick();

        // Update animation based on the player's state
        self.anim = match self.player.fishing_state {
            FishingState::Idle => Anim::DEFAULT,
            FishingState::Biting => Anim::BITING,
            FishingState::Catching => Anim::CATCHING,
            FishingState::Caught => Anim::CAUGHT,
        };
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl Toast {
    fn start(&mut self, msg: String) {
        self.message = msg;
        self.timer = 150;
    }

    fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
    }
}
