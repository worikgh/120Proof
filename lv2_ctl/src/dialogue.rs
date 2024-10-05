use std::char;

/// A dialogue to input a value
/// Large chunks of this are from Ratatui example: `user_input.rs`
// use crate::colours::HEADER_BG;
// use crate::colours::NORMAL_ROW_COLOR;
// Using Ratatui get some text input from the user
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};
// use crate::colours::TEXT_COLOR;
// use crate::colours::TEXT_COLOR;

#[derive(Debug)]
pub enum DialogueError {
    // Close the dialogue
    Close,
}

/// Return DialogueValue::Continue for key strokes, and
/// DialogueValue::Final(<entered string>) when Enter pressed
pub enum DialogueValue {
    Continue,
    Final(String),
}

pub type DialogueResult = Result<DialogueValue, DialogueError>;

pub struct Dialogue {
    /// The string to pronpt the user
    prompt: String,

    /// Current value of the user input
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
}

impl Dialogue {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            input: String::new(),
            character_index: 0,
        }
    }
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    /// Bound the cursor position
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }
    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete =
                self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input =
                before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    pub fn handle_key(&mut self, key: &KeyEvent) -> DialogueResult {
        match key.code {
            crossterm::event::KeyCode::Esc => Err(DialogueError::Close),
            KeyCode::Char(c) => {
                self.enter_char(c);
                Ok(DialogueValue::Continue)
            }
            KeyCode::Enter => {
                // Use input as
                Ok(DialogueValue::Final(self.input.clone()))
            }
            KeyCode::Backspace => {
                self.delete_char();
                Ok(DialogueValue::Continue)
            }
            KeyCode::Left => {
                self.move_cursor_left();
                Ok(DialogueValue::Continue)
            }
            KeyCode::Right => {
                self.move_cursor_right();
                Ok(DialogueValue::Continue)
            }
            _ => Ok(DialogueValue::Continue),
        }
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // Header, body, and footer
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [_header_area, rest_area, _footer_area] = vertical.areas(area);

        // Divide middle area (`rest_area`) into three to confine the
        // dialogue to the center of the screen
        let v = Layout::vertical([
            Constraint::Percentage(37),
            Constraint::Percentage(26),
            Constraint::Percentage(37),
        ]);
        let [_, dialogue_area, _] = v.areas(rest_area);

        let block = Block::bordered().title(self.prompt.as_str());

        // let text_area = border_block.inner(dialogue_area);
        let text_area = block.inner(dialogue_area);

        let input = Paragraph::new(self.input.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::bordered().title("Input"));

        //  // border_block.render(dialogue_area, buf);
        // let info_paragraph = Paragraph::new(self.prompt.as_str())
        //    .block(block)
        //    .fg(TEXT_COLOR)
        //    .wrap(Wrap { trim: false });

        // // We can now render the item info
        // info_paragraph.render(text_area, buf);
        input.render(text_area, buf);
    }
    //    fn render_list() {
}
