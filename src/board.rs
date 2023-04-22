use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Attribute, Print, SetAttribute, SetAttributes, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, queue, Result};
use std::io::{Stdout, Write};

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>,
}

impl Board {
    pub fn initialize_board() -> Board {
        let width: usize = 10;
        let height: usize = 20;
        let row = vec![false; width];
        let cells = vec![row.clone(); height];

        let board = Board {
            width: width,
            height: height,
            cells: cells,
        };

        return board;
    }

    pub fn draw_blank_board(screen: &mut Stdout, board: &mut Board) -> Result<()> {
        let (_max_width, max_height) = crossterm::terminal::size()?;

        execute!(screen, cursor::Hide, Clear(ClearType::All))?;

        // left wall
        for i in 0..board.height {
            execute!(
                screen,
                MoveTo(0, max_height - (i as u16)),
                Print("\u{2588}\u{2588}".to_string())
            )?;
        }

        // bottom line
        for i in 0..board.width {
            execute!(
                screen,
                MoveTo((i as u16) * 2, max_height),
                Print("\u{2588}\u{2588}".to_string())
            )?;
        }

        // right wall
        for i in 0..board.height {
            execute!(
                screen,
                MoveTo((board.width as u16) * 2, max_height - (i as u16)),
                Print("\u{2588}\u{2588}".to_string())
            )?;
        }

        Ok(())
    }

    // fn speed_up() {} /* increases the speed the pieces go down */
    // fn update_board() {} /* update with the dropping pieces */
    // fn check_clear() {} /* checks if player cleared a line */
}
