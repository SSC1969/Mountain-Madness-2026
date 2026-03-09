use crate::{
    chat::ChatHandler,
    event::{AppEvent, Event, EventHandler, NavigationDirection},
    items::{ItemTypes, fish::Fish},
    player::{FishingState, Player},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, widgets::ListState};
use tui_input::{Input, backend::crossterm::EventHandler as crosstermEventHandler};

#[derive(Clone, Default, Debug)]
pub enum Menu {
    #[default]
    Home,
    Backpack,
    Dex,
    Options,
}

pub const MENU_SIZE: i32 = 4;

impl Menu {
    fn next(&self) -> Self {
        match self {
            Menu::Home => Menu::Backpack,
            Menu::Backpack => Menu::Dex,
            Menu::Dex => Menu::Options,
            Menu::Options => Menu::Home,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Menu::Home => Menu::Options,
            Menu::Backpack => Menu::Home,
            Menu::Dex => Menu::Backpack,
            Menu::Options => Menu::Dex,
        }
    }
}

pub enum Anim {
    DEFAULT,
    BITING,
    CATCHING,
    CAUGHT,
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

    pub input: Input,
    // Whether the chatbox is open or not
    pub input_mode: InputMode,
    // most recent n messages
    pub messages: Vec<String>,

    pub cursor_position: Option<(u16, u16)>,
    pub anim: Anim,
    pub recent_catch: Option<Fish>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
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
                AppEvent::ChangeInputMode(im) => match im {
                    InputMode::Normal => self.input_mode = im,
                    InputMode::Editing => self.input_mode = im,
                },
                AppEvent::ChangePlayerName(name) => {
                    self.player.name = name.clone();
                    self.chat.update_name(name);
                }
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
            KeyCode::Char('h') => self.events.send(AppEvent::ChangeMenu(Menu::Home)),
            KeyCode::Char('d') => self.events.send(AppEvent::ChangeMenu(Menu::Dex)),
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
            Menu::Dex => match key_event.code {
                KeyCode::Up => self.dex_state.select_previous(),
                KeyCode::Down => self.dex_state.select_next(),
                KeyCode::Left => self.menu_tab = (self.menu_tab + 1) % 2,
                KeyCode::Right => self.menu_tab = (self.menu_tab + 1) % 2,
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        self.player.tick();

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
