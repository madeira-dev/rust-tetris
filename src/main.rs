use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{ResetColor, Stylize};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute, queue, Result};
use std::io::{stdin, stdout, Stdin, Stdout, Write};

mod board;
mod piece;

fn main() -> Result<()> {
    let mut board;
    let mut piece;
    let mut screen;

    board = board::Board::initialize_board();
    piece = piece::Piece::initialize_piece();
    screen = stdout();

    // entering alternate screen and enabling raw mode
    setup_screen(&mut screen)?;
    board::Board::draw_blank_board(&mut screen, &mut board)?;

    piece::Piece::draw_l_shape(&mut screen, &mut piece)?;
    piece::Piece::draw_s_shape(&mut screen, &mut piece)?;
    piece::Piece::draw_j_shape(&mut screen, &mut piece)?;
    piece::Piece::draw_i_shape(&mut screen, &mut piece)?;
    piece::Piece::draw_z_shape(&mut screen, &mut piece)?;
    piece::Piece::draw_o_shape(&mut screen, &mut piece)?;
    piece::Piece::draw_t_shape(&mut screen, &mut piece)?;
    piece::Piece::clear_piece_squares(&mut screen, &mut piece)?;

    loop {
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
            }) => {
                /* could have placed terminate_screen() and game_interrupt() here
                but then Ok(()) would become unreachable. See if is possible to fix
                this later. For now, leave a break; here */
                break;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: _,
                kind: _,
                state: _,
            }) => piece::Piece::move_right(&mut screen),
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: _,
                kind: _,
                state: _,
            }) => piece::Piece::move_left(&mut screen),
            Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                modifiers: _,
                kind: _,
                state: _,
            }) => piece::Piece::drop_down(&mut screen),
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: _,
                kind: _,
                state: _,
            }) => piece::Piece::hold_piece(&mut screen),
            Event::Key(KeyEvent {
                code: KeyCode::Char('z'),
                modifiers: _,
                kind: _,
                state: _,
            }) => piece::Piece::rotate_piece(&mut screen),
            _ => (),
        }
    }

    // ending game
    terminate_screen(&mut screen)?;
    game_interrupt();

    Ok(())
}

fn setup_screen(terminal: &mut Stdout) -> Result<()> {
    execute!(terminal, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    Ok(())
}

fn terminate_screen(terminal: &mut Stdout) -> Result<()> {
    execute!(terminal, LeaveAlternateScreen)?;
    execute!(terminal, cursor::Show)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn game_interrupt() {
    /* this handles ctrl+c */
    std::process::exit(0);
}
