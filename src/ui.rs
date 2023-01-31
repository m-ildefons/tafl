use tui::{
    backend::Backend,
    layout::{Alignment, Layout, Constraint, Rect, Direction},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    text::{Text},
    Frame,
};

use super::{
    game::{Coord},
    piece::{Piece},
    App,
};


const PADDING: u16 = 1;

const CELL_HEIGHT: u16 = 5;
const CELL_WIDTH: u16 = 9;


pub fn draw <B: Backend> (f: &mut Frame<B>, app: &mut App) {
    let main_block =
        Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::Cyan))
        .title(format!("{}", app.title));

    f.render_widget(main_block, f.size());

    let board_size = app.game.board.size;
    let board_height = CELL_HEIGHT * board_size + 6 * PADDING;

    let vert_pad_height = f.size().height.checked_sub(board_height).unwrap_or_default() / 2;

    let main_layout =
        Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .horizontal_margin(0)
        .constraints(vec![
                     Constraint::Min(vert_pad_height),
                     Constraint::Length(board_height),
                     Constraint::Min(vert_pad_height),
        ])
        .split(f.size());

    if ! app.help {
        draw_board(f, main_layout[1], app);
    } else {
        draw_help(f, main_layout[1]);
    }
    draw_footer(f, main_layout[2], app);
}

fn draw_board <B: Backend> (f: &mut Frame<B>, rect: Rect, app: &mut App) {
    let board_size = app.game.board.size;
    let board_width = CELL_WIDTH * board_size + 12 * PADDING;
    let hori_pad_width = rect.width.checked_sub(board_width).unwrap_or_default() / 2;

    let outer_board_layout =
        Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(0)
        .horizontal_margin(1)
        .constraints(vec![
                     Constraint::Min(hori_pad_width),
                     Constraint::Length(board_width),
                     Constraint::Min(hori_pad_width),
        ])
        .split(rect);

    let outer_board_block =
        Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    f.render_widget(outer_board_block, outer_board_layout[1]);

    let middle_board_layout =
        Layout::default()
        .vertical_margin(1)
        .horizontal_margin(2)
        .constraints(vec![Constraint::Percentage(100)])
        .split(outer_board_layout[1]);

    let middle_board_block =
        Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    f.render_widget(middle_board_block, middle_board_layout[0]);

    let inner_board_layout =
        Layout::default()
        .vertical_margin(1)
        .horizontal_margin(2)
        .constraints(vec![Constraint::Percentage(100)])
        .split(middle_board_layout[0]);

    let inner_board_block =
        Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    f.render_widget(inner_board_block, inner_board_layout[0]);

    let board_layout =
        Layout::default()
        .vertical_margin(0)
        .horizontal_margin(1)
        .constraints(vec![Constraint::Percentage(100)])
        .split(inner_board_layout[0]);

    let row_constraints =
        std::iter::repeat(Constraint::Length(CELL_HEIGHT))
        .take(board_size as usize)
        .collect::<Vec<_>>();

    let col_constraints =
        std::iter::repeat(Constraint::Length(CELL_WIDTH))
        .take(board_size as usize)
        .collect::<Vec<_>>();

    let row_rects =
        Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .horizontal_margin(0)
        .constraints(row_constraints.clone())
        .split(board_layout[0]);

    for (r, row_rect) in row_rects.into_iter().enumerate() {
        let col_rects =
            Layout::default()
            .direction(Direction::Horizontal)
            .vertical_margin(0)
            .horizontal_margin(1)
            .constraints(col_constraints.clone())
            .split(row_rect);

        for (c, cell_rect) in col_rects.into_iter().enumerate() {
            draw_cell(f, cell_rect, app, r, c);
        }
    }
}

fn draw_cell <B: Backend> (f: &mut Frame<B>, rect: Rect, app: &mut App, row: usize, col: usize) {
    let coord: Coord = (col, row);
    let mid = ((app.game.board.size - 1) / 2) as usize;
    let square = app.game.board.board[col][row];


    let style =
        if app.cursor == coord {
            Style::default().bg(Color::Black).fg(Color::Green)
        } else if app.selected == Some((col,row)) {
            Style::default().bg(Color::Black).fg(Color::Yellow)
        } else if let Some(selected) = app.selected {
            let disabled = (selected.0 != col) && (selected.1 != row);
            if disabled {
                Style::default().bg(Color::Black).fg(Color::Gray)
            } else {
                Style::default().bg(Color::Black).fg(Color::Cyan)
            }
        } else {
            match square.piece {
                Some(Piece::King) => Style::default().bg(Color::Black).fg(Color::Magenta),
                Some(Piece::Muscovite) => Style::default().bg(Color::Black).fg(Color::Gray),
                Some(Piece::Swede) => Style::default().bg(Color::Black).fg(Color::LightRed),
                None => Style::default().bg(Color::Black).fg(Color::Cyan),
            }
        };

    let border_type =
        if app.cursor == coord {
            BorderType::Thick
        } else if app.selected == Some((col,row)) {
            BorderType::Thick
        } else if square.piece != None {
            BorderType::Double
        } else {
            BorderType::Plain
        };

    let cell_block =
        Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .style(style);

    f.render_widget(cell_block, rect);

    let castle_layout =
        Layout::default()
        .vertical_margin(1)
        .horizontal_margin(2)
        .constraints(vec![Constraint::Percentage(100)])
        .split(rect);

    if col == mid {
        if row == mid {
            let castle_block =
                Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double);

            f.render_widget(castle_block, castle_layout[0]);
        }
    }

    let size = (app.game.board.size as usize) - 1;
    let corner =
        (col == 0 && row == 0)
        || (col == size && row == 0)
        || (col == 0 && row == size)
        || (col == size && row == size);
    if corner {
        let corner_block =
            Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

            f.render_widget(corner_block, castle_layout[0]);
    }

    let cell_layout =
        Layout::default()
        .vertical_margin(1)
        .horizontal_margin(1)
        .constraints(vec![Constraint::Percentage(100)])
        .split(castle_layout[0]);

    let p = match square.piece {
        Some(q) => format!("{}", q),
        _ => format!(" "),
    };

    let piece =
        Paragraph::new(p)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center);

    f.render_widget(piece, cell_layout[0]);
}

fn draw_help <B: Backend> (f: &mut Frame<B>, rect: Rect) {
    let help_layout =
        Layout::default()
        .vertical_margin(1)
        .horizontal_margin(1)
        .constraints(vec![
                     Constraint::Min(5),
                     Constraint::Length(rect.height - 5),
        ])
        .split(rect);

    let help_header_text = Text::from(r#"
Tafl Help

Tafl - A strategy board game.
"#);

    let help_header =
        Paragraph::new(help_header_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::TOP))
        .wrap(Wrap {trim: true})
        .alignment(Alignment::Center);

    let help_text = Text::from(r#"
Controls:
  - Navigation                  |  Up, Down, Left, Right
  - Select,Unselect,Move Piece  |  Space
  - Quit                        |  Ctrl+c, q

Rules:

The Muscovites (gray, attackers) start the game. A piece can only move horizontally or vertically
but it can do so any distance unless another piece stands in the way. A piece can also not move
onto an already occupied square.
"#);
    let help =
        Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::BOTTOM))
        .wrap(Wrap {trim: false})
        .alignment(Alignment::Left);

    f.render_widget(help_header, help_layout[0]);
    f.render_widget(help, help_layout[1]);
}

fn draw_footer <B: Backend> (f: &mut Frame<B>, rect: Rect, app: &mut App) {
    let team = match app.game.turn % 2 {
        0 => { "Muscovite" },
        _ => { "Swede" },
    };

    let mut footer_text = Text::from(format!("It's the {}'s turn.\n", team));
    footer_text.extend(Text::from("help: h | quit: q"));

    let footer =
        Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center);

    f.render_widget(footer, rect);
}
