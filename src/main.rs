use dames_italianes::{Board, Position, SquareKind, Team, BOARD_HEIGHT, BOARD_WIDTH};
use macroquad::{prelude::*, text};

mod textures;

use crate::textures::*;

const WINDOW_LENGTH: i32 = 720;

#[macroquad::main(window_conf)]
async fn main() {

    // Tweakables
    let grid_spacing = screen_width() as usize / BOARD_WIDTH;
    let background_white = WHITE;
    let background_black = BLACK;

    // Game state
    let mut board = Board::default();
    let mut current_player = Team::White;
    let mut selected_pos: Option<Position> = None;

    loop {
        //dbg!(&board);
        //dbg!(selected_pos);

        clear_background(background_white);
        draw_black_squares(background_black, grid_spacing);
        dbg!(&selected_pos);
        draw_selected_pos(&selected_pos, 5.0, GREEN, grid_spacing);

        if is_mouse_button_pressed(MouseButton::Right) { selected_pos = None; }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (globl_x, globl_y) = mouse_position();
            let local_x = (globl_x as usize / grid_spacing).min(BOARD_WIDTH - 1).max(0);
            let local_y = (globl_y as usize / grid_spacing).min(BOARD_HEIGHT - 1).max(0);
            dbg!(local_x, local_y);

            let new_pos = Position::new(local_x, local_y);

            if let Some(old_pos) = selected_pos {
                if board.move_is_legal(old_pos, new_pos) {
                    board.make_move(old_pos, new_pos).expect("SAFETY: We just checked validity");
                }

                selected_pos = None;

            } else { // Nothing selected
                if board[new_pos].is_some() {
                    selected_pos = Some(new_pos);
                }
            }
        }

        for (p, ssq) in board.iterate() {
            if let Some(sq) = ssq {
                let texture = match sq.kind {
                    SquareKind::Peça => pawn_texture(sq.team),
                    SquareKind::Dama => pawn_texture(sq.team),
                };
                draw_texture(&texture,
                             (p.col*grid_spacing) as f32,
                             (p.row*grid_spacing) as f32,
                             WHITE);
            }
        }


        next_frame().await
    }

}

fn draw_selected_pos(sel: &Option<Position>, thickness: f32, color: Color, grid_spacing: usize) {
    let f = |k: usize| (k*grid_spacing) as f32;
    if let Some(p) = sel {
        draw_line(f(p.col), f(p.row),
                  f(p.col+1), f(p.row),
                  thickness, color);
        draw_line(f(p.col), f(p.row),
                  f(p.col), f(p.row+1),
                  thickness, color);
        draw_line(f(p.col+1), f(p.row),
                  f(p.col+1), f(p.row),
                  thickness, color);
        draw_line(f(p.col), f(p.row+1),
                  f(p.col), f(p.row+1),
                  thickness, color);
    }
}

fn draw_black_squares(black: Color, grid_spacing: usize) {
    for y in 0..8 {
        for x in 0..8 {
            if (x + y) % 2 == 0 {
                let true_x = x*grid_spacing;
                let true_y = y*grid_spacing;
                draw_rectangle(true_x as f32, true_y as f32,
                               grid_spacing as f32,
                               grid_spacing as f32, black);
            }
        }
    }
}

fn draw_grid(grid_thickness: f32, grid_color: Color, grid_spacing: usize) {
    for y in (0..screen_height() as usize).step_by(grid_spacing) {
        draw_line(0.0, y as f32,
                  screen_width(), y as f32,
                  grid_thickness, grid_color);
    }
    for x in (0..screen_width() as usize).step_by(grid_spacing) {
        draw_line(x as f32, 0.0,
                  x as f32, screen_height(),
                  grid_thickness, grid_color);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dames".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: WINDOW_LENGTH,
        window_height: WINDOW_LENGTH,
        ..Default::default()
    }
}
