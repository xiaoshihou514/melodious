use tuirealm::{
    command::{Cmd, CmdResult},
    tui::layout::Rect,
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent, Props,
    State as TuiRealmState, StateValue,
};

use super::Msg;
use crate::_impl_mock_common;

#[derive(Default)]
struct State {
    content: Vec<char>,
    cursor_pos: usize,
}

impl State {
    fn char_add(&mut self, c: char) {
        self.content.push(c);
    }
    fn backspace(&mut self) {
        if self.cursor_pos > 0 && !self.content.is_empty() {
            self.content.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
        }
    }
    fn incr_cursor_pos(&mut self) {
        if self.cursor_pos < self.content.len() {
            self.cursor_pos += 1;
        }
    }
    fn cursor_pos_at_begin(&mut self) {
        self.cursor_pos = 0;
    }
    fn decr_cursor_pos(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }
}

#[derive(Default)]
pub struct SearchBar {
    props: Props,
    state: State,
}

impl MockComponent for SearchBar {
    _impl_mock_common!();
    fn state(&self) -> TuiRealmState {
        TuiRealmState::Tup2((
            StateValue::String(String::from_iter(self.state.content.iter())),
            StateValue::Usize(self.state.cursor_pos),
        ))
    }

    fn view(&mut self, frame: &mut tuirealm::Frame, area: Rect) {}

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        todo!()
    }
}

impl Component<Msg, NoUserEvent> for SearchBar {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        todo!()
    }
}
