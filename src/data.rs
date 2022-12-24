#[derive(Copy, Clone)]
pub enum Usb2SnesError {
    None,
    CantConnect,
    NoDevice,
    CantAttach,
    NoGame
}

pub static RED : Color = Color{a : 0, r : 255, g : 0, b : 0};
pub static GREEN : Color = Color{a : 0, r : 0, g : 255, b : 0};
pub static GREY : Color = Color{a : 0, r : 0x80, g : 0x80, b : 0x80};
pub static LIGHT_GREEN : Color = Color{a : 0, r : 0x80, g : 0xee, b : 0x90}; // #90ee90
pub static DARK_GREEN : Color = Color{a : 0, r : 0, g : 0x80, b : 0};
pub static WHITE : Color = Color{a : 0, r : 255, g : 255, b : 255};
pub static AQUA : Color = Color{a : 0, r : 0, g : 255, b : 255};
pub static PINK : Color = Color{a : 0, r : 255, g : 0xC0, b : 0xCB};
pub static BLACK : Color = Color{a : 0, r : 0, g : 0, b : 0};
pub static YELLOW : Color = Color{a : 0, r : 255, g : 255, b : 0};



pub fn usb2snes_to_string(err : Usb2SnesError) -> String {
    match err {
        Usb2SnesError::None => {String::from("None")},
        Usb2SnesError::CantConnect => {String::from("Can't connect")},
        Usb2SnesError::NoDevice => {String::from("No device")},
        Usb2SnesError::CantAttach => {String::from("Can't attach to device")},
        Usb2SnesError::NoGame => {String::from("No game present")}
    }
}
pub struct SharedData {
    pub usb2snes_ready : bool,
    pub usb2snes_error : Usb2SnesError,
    pub rom_data : Vec<u8>,
    pub map_data : Vec<u8>,
    pub samus_pos : sdl2::rect::Point,
    pub camera : sdl2::rect::Point,
    pub map_id : u8,
    pub radius : sdl2::rect::Point,
    pub width : u16,
    pub clip: u16,
    pub bts : u16,
    pub bts_byte : u8,
    pub door_stuff : u16
}

pub struct GameTileData {
    pub tile_x : i32,
    pub tile_y : i32,
    pub bts : usize,
    pub bts_value : u8,
    pub clip : usize,
    pub clip_value : u16,
    pub door_stuff : u16,
    pub bts_byte : u8
}

impl SharedData {
    pub fn new() -> SharedData {
        SharedData { 
            usb2snes_error: Usb2SnesError::None,
            rom_data: vec![],
            map_data: vec![],
            samus_pos: sdl2::rect::Point::new(0, 0),
            camera: sdl2::rect::Point::new(0, 0),
            map_id: 0,
            radius: sdl2::rect::Point::new(0, 0),
            width: 0,
            clip: 0,
            bts: 0,
            door_stuff : 0,
            usb2snes_ready: false,
            bts_byte : 0
        }
    }
}

use std::{collections::HashMap};
//use crate::gamearea::GameArea;
use sdl2::pixels::Color;

use crate::mycanvas::MyCanvas;
#[macro_use]
use lazy_static::lazy_static;

fn slope00(canvas : &mut MyCanvas, tile_x : i32, tile_y : i32, h_flip : i32, v_flip : i32) {
    canvas.draw_game_box_fill(tile_x + 16 * h_flip, tile_y + 8 + 24 * v_flip, 16, 8, LIGHT_GREEN, GREY);
}

fn slope01(canvas : &mut MyCanvas, tile_x : i32, tile_y : i32, h_flip : i32, v_flip : i32) {
    canvas.draw_game_box_fill(tile_x + 8 + 24 * h_flip, tile_y + 16 * v_flip, 8, 16, LIGHT_GREEN,GREY);
}

fn slope12(canvas : &mut MyCanvas, tile_x : i32, tile_y : i32, h_flip : i32, v_flip : i32) {
    canvas.set_draw_color(LIGHT_GREEN);
    canvas.draw_game_line(tile_x, tile_y + 16 * v_flip, tile_x + 16 * h_flip, tile_y);
    //canvas.set_draw_color(WHITE);
    canvas.draw_game_line(tile_x + 16 * h_flip, tile_y, tile_x + 16 * h_flip, tile_y + 16 * v_flip);
    //canvas.set_draw_color(RED);
    canvas.draw_game_line(tile_x + 16 * h_flip, tile_y + 16 * v_flip, tile_x, tile_y + 16 * v_flip);
    canvas.set_draw_color(BLACK);
}

fn slope13(canvas : &mut MyCanvas, tile_x : i32, tile_y : i32, h_flip : i32, v_flip : i32) {
    canvas.draw_game_box_fill(tile_x + 16 * h_flip, tile_y + 16 * v_flip, 16, 16, LIGHT_GREEN, GREY);
}

fn slope14(canvas : &mut MyCanvas, tile_x : i32, tile_y : i32, h_flip : i32, v_flip : i32) {
    canvas.set_draw_color(LIGHT_GREEN);
    canvas.draw_game_line(tile_x + 8 * h_flip, tile_y + 16 * v_flip, tile_x + 16 * h_flip, tile_y + 8 * v_flip);
    canvas.draw_game_line(tile_x + 16 * h_flip, tile_y + 8 * v_flip, tile_x + 16 * h_flip, tile_y + 16 * v_flip);
    canvas.draw_game_line(tile_x + 16 * h_flip, tile_y + 16 * v_flip, tile_x + 8 * h_flip, tile_y + 16 * v_flip);
    canvas.set_draw_color(BLACK);
}

fn slope15(canvas : &mut MyCanvas, tile_x : i32, tile_y : i32, h_flip : i32, v_flip : i32) {
    canvas.set_draw_color(LIGHT_GREEN);
    canvas.draw_game_line(tile_x + 8 * h_flip, tile_y, tile_x + 16 * h_flip, tile_y);
    canvas.draw_game_line(tile_x + 16 * h_flip, tile_y, tile_x + 16 * h_flip, tile_y + 16 * v_flip);
    canvas.draw_game_line(tile_x, tile_y + 8 * v_flip, tile_x + 7 * h_flip, tile_y);
    canvas.draw_game_line(tile_x, tile_y + 8 * v_flip, tile_x, tile_y + 16 * v_flip);
    canvas.draw_game_line(tile_x, tile_y + 16 * v_flip, tile_x + 16 * h_flip, tile_y + 16 * v_flip);
    canvas.set_draw_color(BLACK);
}

lazy_static! {
    static ref SLOPES : HashMap<u8, fn(&mut MyCanvas, i32, i32, i32, i32)> = {
        let mut s : HashMap<u8, fn(&mut MyCanvas, i32, i32, i32, i32)> = HashMap::new();
        s.insert(0, slope00);
        s.insert(1, slope01);
        s.insert(0x12, slope12);
        s.insert(0x13, slope13);
        s.insert(0x14, slope14);
        s.insert(0x15, slope15);
        s
    };
    pub static ref OUTLINES : HashMap::<u8, fn(&mut MyCanvas, &GameTileData)> = {
        let mut hash = HashMap::<u8, fn(&mut MyCanvas, &GameTileData)>::new();
        hash.insert(0, outline00);
        hash.insert(1, outline01);
        hash.insert(2, outline02);
        hash.insert(3, outline03);
        hash.insert(4, outline04);
        hash.insert(5, outline05);
        hash.insert(6, outline06);
        hash.insert(7, outline07);
        hash.insert(8, outline08);
        hash.insert(9, outline09);
        hash.insert(0xA, outline0A);
        hash.insert(0xB, outline0B);
        hash.insert(0xC, outline0C);
        hash.insert(0xD, outline0D);
        hash.insert(0xE, outline0E);
        hash.insert(0xF, outline0F);
        return hash;
    };
}

fn outline00(_canvas : &mut MyCanvas, _game_tile_data : &GameTileData) {

}

// Slopes
fn outline01(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    let mut v_flip = 0;
    let mut h_flip = 0;
    let mut new_tile_x = game_tile_data.tile_x;
    let mut new_tile_y = game_tile_data.tile_y;
    if game_tile_data.bts_value & 0x40 != 0 {
        new_tile_x += 16;
        h_flip = -1
    } else {
        h_flip = 1
    }
    if game_tile_data.bts_value & 0x80 != 0 {
        new_tile_y += 16;
        v_flip = -1
    } else {
        v_flip = 1
    }
    let slope_id = game_tile_data.bts_value & 0x1F;
    if slope_id == 0x13 || slope_id == 0x00 || slope_id == 0x01 || slope_id == 0x07 {
        if v_flip == 1 {
            v_flip = 0
        }
        if h_flip == 1 {
            h_flip = 0
        }
    }
    if SLOPES.contains_key(&slope_id) {
        SLOPES[&slope_id](canvas, new_tile_x, new_tile_y, h_flip, v_flip)
    } else {
        canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, LIGHT_GREEN);
        canvas.draw_game_text(game_tile_data.tile_x + 1, game_tile_data.tile_y + 1, format!("{:02x}", slope_id).as_str(), LIGHT_GREEN);
    }
}

// X-RAY Air
fn outline02(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, RED);
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "X", RED);
}

// Threadmill
fn outline03(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box_fill(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, Color::RGB(170, 0, 0), Color::RGB(85, 0, 0));
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "T", Color::RGB(128, 128, 128));
}

// Shootable air
fn outline04(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, Color::RGB(0, 170, 0));
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "A", Color::RGB(128, 128, 128));
}

// Horizontal extension, this is supposed to find the right tile of extension but this is more memory request~~
fn outline05(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, Color::RGB(170, 0, 170));
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "H", Color::RGB(128, 128, 128));
}

// Denied X-RAY
fn outline06(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box_fill(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, Color::RGB(180, 180, 180), Color::RGB(85, 85, 85));
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "F", Color::RGB(128, 128, 128));
}

// Bombable air? niée?
fn outline07(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, Color::RGB(170, 0, 170));
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "H", Color::RGB(128, 128, 128));
}

// Solid
fn outline08(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box_fill(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, WHITE, GREY).unwrap();
}

// Doors
fn outline09(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box_fill(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, RED, PINK);
    canvas.draw_game_text(game_tile_data.tile_x + 1, game_tile_data.tile_y, format!("{:02X}", game_tile_data.bts_value).as_str(), RED);
}

// Spike
fn outline0A(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, GREY);
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "S", YELLOW);
}

// Crumble
fn outline0B(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, GREY);
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "C", WHITE);
}

// Shot Block
fn outline0C(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, Color::RGB(0xB5, 0x1D, 0xC3,));
    // We need to read more memory to get the type :(
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "S", Color::RGB(0xF4, 0xD8, 0xF6));
}

// Vertical Extension
fn outline0D(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, GREY);
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "V", GREY);
}

// Grapple block
fn outline0E(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, GREY);
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "G", WHITE);
}

fn outline0F(canvas : &mut MyCanvas, game_tile_data : &GameTileData) {
    canvas.draw_game_box(game_tile_data.tile_x, game_tile_data.tile_y, 16, 16, GREY);
    canvas.draw_game_text(game_tile_data.tile_x + 4, game_tile_data.tile_y - 1, "B", WHITE);
}



