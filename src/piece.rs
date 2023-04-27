use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor, Stylize};
use crossterm::{cursor, execute, queue, terminal, Result};
use std::io::{Stdout, Write};
use std::vec;

pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<char>>,
}

/* initialize piece matrix */
impl Piece {
    pub fn initialize_piece() -> Piece {
        let width: usize = 3;
        let height: usize = 3;
        let row = vec![' '; width];
        let cells = vec![row.clone(); height];

        let piece = Piece {
            width: width,
            height: height,
            cells: cells,
        };

        return piece;
    }
}

/* pieces draws */
impl Piece {
    // maybe make the draw function return pieces position for further manipulations. i.e. clear piece squares
    pub fn draw_l_shape(
        screen: &mut Stdout,
        piece: &mut Piece,
        starting_x: usize,
        starting_y: usize,
    ) -> Result<()> {
        piece.cells[0][0] = '\u{2588}';
        piece.cells[0][1] = '\u{2588}';
        piece.cells[0][2] = '\u{2588}';
        piece.cells[1][2] = '\u{2588}';

        // probably remove this execute statement
        execute!(
            screen,
            SetForegroundColor(Color::Red),
            MoveTo(starting_x as u16, starting_y as u16),
            Print(piece.cells[0][0]),
            MoveTo(starting_x as u16, (starting_y + 1) as u16),
            Print(piece.cells[0][1]),
            MoveTo(starting_x as u16, (starting_y + 2) as u16),
            Print(piece.cells[0][2]),
            MoveTo((starting_x + 1) as u16, (starting_y + 2) as u16),
            Print(piece.cells[1][2])
        )?;

        Ok(())
    }

    pub fn draw_s_shape(screen: &mut Stdout, piece: &mut Piece) -> Result<()> {
        piece.cells[0][2] = '\u{2588}';
        piece.cells[1][2] = '\u{2588}';
        piece.cells[1][1] = '\u{2588}';
        piece.cells[2][1] = '\u{2588}';

        // probably remove this execute! statement
        execute!(
            screen,
            SetForegroundColor(Color::Green),
            MoveTo(0, 4),
            Print(piece.cells[0][2]),
            MoveTo(1, 4),
            Print(piece.cells[1][2]),
            MoveTo(1, 3),
            Print(piece.cells[1][1]),
            MoveTo(2, 3),
            Print(piece.cells[2][1])
        )?;

        Ok(())
    }

    pub fn draw_j_shape(screen: &mut Stdout, piece: &mut Piece) -> Result<()> {
        piece.cells[2][0] = '\u{2588}';
        piece.cells[2][1] = '\u{2588}';
        piece.cells[2][2] = '\u{2588}';
        piece.cells[1][2] = '\u{2588}';

        // probably remove this execute! statement
        execute!(
            screen,
            SetForegroundColor(Color::Blue),
            MoveTo(6, 0),
            Print(piece.cells[2][0]),
            MoveTo(6, 1),
            Print(piece.cells[2][1]),
            MoveTo(6, 2),
            Print(piece.cells[2][2]),
            MoveTo(5, 2),
            Print(piece.cells[1][2])
        )?;

        Ok(())
    }

    pub fn draw_i_shape(screen: &mut Stdout, piece: &mut Piece) -> Result<()> {
        piece.cells[1][0] = '\u{2588}';
        piece.cells[1][1] = '\u{2588}';
        piece.cells[1][2] = '\u{2588}';

        // probably remove this execute! statement
        execute!(
            screen,
            SetForegroundColor(Color::Yellow),
            MoveTo(5, 4),
            Print(piece.cells[1][0]),
            MoveTo(5, 5),
            Print(piece.cells[1][1]),
            MoveTo(5, 6),
            Print(piece.cells[1][2])
        )?;

        Ok(())
    }

    pub fn draw_z_shape(screen: &mut Stdout, piece: &mut Piece) -> Result<()> {
        piece.cells[0][1] = '\u{2588}';
        piece.cells[1][1] = '\u{2588}';
        piece.cells[1][2] = '\u{2588}';
        piece.cells[2][2] = '\u{2588}';

        // probably remove this execute! statement
        execute!(
            screen,
            SetForegroundColor(Color::Cyan),
            MoveTo(0, 9),
            Print(piece.cells[0][1]),
            MoveTo(1, 9),
            Print(piece.cells[1][1]),
            MoveTo(1, 10),
            Print(piece.cells[1][2]),
            MoveTo(2, 10),
            Print(piece.cells[2][2])
        )?;

        Ok(())
    }

    pub fn draw_o_shape(screen: &mut Stdout, piece: &mut Piece) -> Result<()> {
        piece.cells[0][0] = '\u{2588}';
        piece.cells[0][1] = '\u{2588}';
        piece.cells[1][0] = '\u{2588}';
        piece.cells[1][1] = '\u{2588}';

        // probably remove this execute! statement
        execute!(
            screen,
            SetForegroundColor(Color::Cyan),
            MoveTo(8, 8),
            Print(piece.cells[0][0]),
            MoveTo(8, 9),
            Print(piece.cells[0][1]),
            MoveTo(9, 8),
            Print(piece.cells[1][0]),
            MoveTo(9, 9),
            Print(piece.cells[1][1])
        )?;

        Ok(())
    }

    pub fn draw_t_shape(screen: &mut Stdout, piece: &mut Piece) -> Result<()> {
        piece.cells[1][1] = '\u{2588}';
        piece.cells[1][2] = '\u{2588}';
        piece.cells[0][2] = '\u{2588}';
        piece.cells[2][2] = '\u{2588}';

        // probably remove this execute! statement
        execute!(
            screen,
            SetForegroundColor(Color::Magenta),
            MoveTo(13, 13),
            Print(piece.cells[1][1]),
            MoveTo(13, 14),
            Print(piece.cells[1][2]),
            MoveTo(12, 14),
            Print(piece.cells[0][2]),
            MoveTo(14, 14),
            Print(piece.cells[2][2])
        )?;

        Ok(())
    }

    pub fn clear_piece_squares(
        _screen: &mut Stdout,
        _piece: &mut Piece,
        starting_x: usize,
        starting_y: usize,
    ) -> Result<()> {
        for i in starting_y..(starting_y + 3) {
            for j in starting_x..(starting_x + 3) {
                execute!(_screen, MoveTo(i as u16, j as u16), Print(' '))?;
            }
        }

        Ok(())
    }
}

/* piece movements */
impl Piece {
    pub fn rotate_piece(
        screen: &mut Stdout,
        piece: &mut Piece,
        starting_x: usize,
        starting_y: usize,
    ) -> Result<()> {
        Piece::clear_piece_squares(screen, piece, starting_x, starting_y)?;
        // swaping columns with rows
        for i in starting_y..(starting_y + 3) {
            for j in starting_x..(starting_x + 3) {
                piece.cells[i][j] = piece.cells[j][i];
            }
        }
        screen.flush().unwrap();

        Ok(())
    }

    pub fn move_right(
        screen: &mut Stdout,
        piece: &mut Piece,
        starting_x: &mut usize,
        starting_y: usize,
    ) -> Result<()> {
        Piece::clear_piece_squares(screen, piece, *starting_x, starting_y)?;
        *starting_x += 1;
        Piece::draw_l_shape(screen, piece, *starting_x, starting_y)?;
        screen.flush().unwrap();
        Ok(())
    }

    pub fn move_left(
        screen: &mut Stdout,
        piece: &mut Piece,
        starting_x: usize,
        starting_y: usize,
    ) -> Result<()> {
        Piece::clear_piece_squares(screen, piece, starting_x, starting_y)?;
        screen.flush().unwrap();
        Ok(())
    }

    pub fn drop_down(screen: &mut Stdout) {
        write!(screen, "space bar pressed ").unwrap();
        screen.flush().unwrap();
    }

    pub fn hold_piece(screen: &mut Stdout) {
        write!(screen, "c key pressed ").unwrap();
        screen.flush().unwrap();
    }
}
