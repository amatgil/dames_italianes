use dames_italianes::{Board, Position, Team, BOARD_HEIGHT, BOARD_WIDTH};
use macroquad::{prelude::*, text};


const WINDOW_LENGTH: i32 = 720;

//#[macroquad::main(window_conf)]
//async fn main() {
//    // Variables
//    let background_color         = Color::from_rgba(24, 25, 38, 255);
//    let grid_thickness           = 2.5;
//    let grid_color               = Color::from_rgba(138, 173, 244, 255);
//    let text_color               = Color::from_rgba(198, 160, 246, 200);
//
//    // Main loop
//    loop {
//        clear_background(WHITE);
//
//        next_frame().await
//    }
//
//}

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

        if is_mouse_button_pressed(MouseButton::Left) {
            let (globl_x, globl_y) = mouse_position();
            let local_x = (globl_x as usize / grid_spacing).min(BOARD_WIDTH - 1);
            let local_y = (globl_y as usize / grid_spacing).min(BOARD_HEIGHT - 1);

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


        next_frame().await
    }

}

fn draw_black_squares(black: Color, grid_spacing: usize) {
    for y in 0..8 {
        for x in 0..8 {
            if (x + y) % 2 == 0 {
                let true_x = x*grid_spacing;
                let true_y = y*grid_spacing;
                draw_rectangle(true_x as f32, true_y as f32, grid_spacing as f32, grid_spacing as f32, black);
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
