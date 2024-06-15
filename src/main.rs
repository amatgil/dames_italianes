use macroquad::{prelude::*, text};



#[macroquad::main(window_conf)]
async fn main() {
    // Variables
    let background_color         = Color::from_rgba(24, 25, 38, 255);
    let grid_thickness           = 2.5;
    let grid_color               = Color::from_rgba(138, 173, 244, 255);
    let grid_spacing             = 30;
    let text_color               = Color::from_rgba(198, 160, 246, 200);

    // Main loop
    loop {
        draw_grid(grid_thickness, grid_color, grid_spacing);
        draw_controls(text_color, grid_spacing);

        next_frame().await
    }
}

fn draw_controls(text_color: Color, grid_spacing: usize) {
    // let grid_spacing = grid_spacing as f32;
    // let tps = (time_between_ticks + 1.0) / (1.0/get_fps() as f32 + time_between_ticks);
    // let is_p = if paused { "On" } else { "Off" };

    // draw_rectangle(0.0, 0.0,
    //                 grid_spacing*10.0, grid_spacing*7.0,
    //                 Color::from_rgba(0, 0, 0, 200));
    // draw_text("U: Increase Speed",                 10.0, grid_spacing*0.8 + 0.0*grid_spacing, grid_spacing, text_color);
    // draw_text("D: Decrease Speed",                 10.0, grid_spacing*0.8 + 1.0*grid_spacing, grid_spacing, text_color);
    // draw_text("R: Reset",                          10.0, grid_spacing*0.8 + 2.0*grid_spacing, grid_spacing, text_color);
    // draw_text(&format!("Space: Pause ({is_p})"),   10.0, grid_spacing*0.8 + 3.0*grid_spacing, grid_spacing, text_color);
    // draw_text(&format!("Speed: {tps:.2} tps",),    10.0, grid_spacing*0.8 + 5.0*grid_spacing, grid_spacing, text_color);
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
        window_title: "Game of Life".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: 1080,
        window_height: 720,
        ..Default::default()
    }
}
