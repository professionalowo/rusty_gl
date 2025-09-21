use self::{action::Action, keycode::Keycode, modifier::Modifier};

pub mod action;
pub mod keycode;
pub mod modifier;

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub keycode: Keycode,
    pub action: Action,
    pub modifier: Modifier,
}

impl KeyEvent {
    pub fn new(keycode: Keycode, action: Action, modifier: Modifier) -> Self {
        Self {
            keycode,
            action,
            modifier,
        }
    }

    pub fn is_press(&self) -> bool {
        self.action == Action::Press
    }
}
