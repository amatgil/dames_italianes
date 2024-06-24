use std::path::{Path, PathBuf};

use macroquad::prelude::*;

use crate::Team;


pub fn pawn_texture(team: Team) -> Texture2D {
    let mut img = match team {
        Team::White => {
            Image::from_file_with_format(
                include_bytes!("../assets/white_pawn_texture.png"),
                Some(ImageFormat::Png)).expect("Failed to load pawn sprite at compile time")
        },
        Team::Black => {
            Image::from_file_with_format(
                include_bytes!("../assets/black_pawn_texture.png"),
                Some(ImageFormat::Png)).expect("Failed to load pawn sprite at compile time")
        }
    };
    Texture2D::from_image(&img)
}


pub fn queen_texture(team: Team) -> Texture2D {
    let mut img = match team {
        Team::White => {
            Image::from_file_with_format(
                include_bytes!("../assets/white_queen_texture.png"),
                Some(ImageFormat::Png)).expect("Failed to load pawn sprite at compile time")
        },
        Team::Black => {
            Image::from_file_with_format(
                include_bytes!("../assets/black_queen_texture.png"),
                Some(ImageFormat::Png)).expect("Failed to load pawn sprite at compile time")
        }
    };


    Texture2D::from_image(&img)
}

