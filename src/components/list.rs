use crate::_impl_mock_common;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    props::{Alignment, Borders, Color, Style, TextModifiers},
    tui::{
        layout::{Corner, Rect},
        text::{Span, Spans},
        widgets::{Block, List as NativeList, ListItem, ListState},
    },
    AttrValue, Attribute, MockComponent, Props, State, StateValue,
};

#[derive(Default)]
pub struct List {
    props: Props,
    items: Vec<String>,
    state: ListState,
}

impl MockComponent for List {
    _impl_mock_common!();
    fn state(&self) -> State {
        State::One(StateValue::Usize(self.state.selected().unwrap()))
    }

    fn view(&mut self, frame: &mut tuirealm::Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            let title = self
                .props
                .get_or(
                    Attribute::Title,
                    AttrValue::Title((String::default(), Alignment::Center)),
                )
                .unwrap_title();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            let inactive_style = self
                .props
                .get(Attribute::FocusStyle)
                .map(|x| x.unwrap_style());
            let div = Block::default()
                .borders(borders.sides)
                .border_style(match focus {
                    true => borders.style(),
                    false => inactive_style
                        .unwrap_or_else(|| Style::default().fg(Color::Reset).bg(Color::Reset)),
                })
                .border_type(borders.modifiers)
                .title(title.0)
                .title_alignment(title.1);
            // Make list entries
            let list_items: Vec<ListItem> = self
                .items
                .iter()
                .map(|item| {
                    ListItem::new(Spans::from(Span::styled(
                        item.clone(),
                        Style::default().add_modifier(modifiers).fg(foreground),
                    )))
                })
                .collect();
            let highlighted_color = self
                .props
                .get(Attribute::HighlightedColor)
                .map(|x| x.unwrap_color());
            let modifiers = match focus {
                true => modifiers | TextModifiers::REVERSED,
                false => modifiers,
            };
            // Make list
            let mut list = NativeList::new(list_items)
                .block(div)
                .style(Style::default().fg(foreground).bg(background))
                .start_corner(Corner::TopLeft);
            if let Some(highlighted_color) = highlighted_color {
                list = list.highlight_style(
                    Style::default()
                        .fg(highlighted_color)
                        .add_modifier(modifiers),
                );
            }
            list = list.highlight_symbol(" ");
            frame.render_stateful_widget(list, area, &mut self.state);
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Up) => {
                if self.state.selected().unwrap() == self.items.len() - 1 {
                    self.state.select(Some(0));
                } else {
                    self.state.select(Some(self.state.selected().unwrap()));
                }
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Down) => {
                if self.state.selected().unwrap() == 0 {
                    self.state.select(Some(self.items.len() - 1));
                } else {
                    self.state.select(Some(self.state.selected().unwrap() - 1));
                }
                CmdResult::Changed(self.state())
            }
            Cmd::Scroll(Direction::Down) => {
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                if self.state.selected().unwrap() + step > self.items.len() - 1 {
                    self.state.select(Some(self.items.len() - 1));
                } else {
                    self.state
                        .select(Some(self.state.selected().unwrap() + step))
                }
                CmdResult::Changed(self.state())
            }
            Cmd::Scroll(Direction::Up) => {
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                if self.state.selected().unwrap() < step {
                    self.state.select(Some(0));
                } else {
                    self.state
                        .select(Some(self.state.selected().unwrap() - step))
                }
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::None,
        }
    }
}
