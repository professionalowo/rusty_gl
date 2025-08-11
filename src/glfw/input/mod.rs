use self::action::Action;
use self::keycode::Keycode;
use self::modifier::Modifier;

pub mod action;
pub mod keycode;
pub mod modifier;

#[derive(Debug)]
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
}
