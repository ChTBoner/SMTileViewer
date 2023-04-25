extern crate sdl2;

use std::sync::{Mutex, Arc};
use data::{*};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Rect, Point};
use sdl2::video::{Window};
use sdl2::surface::Surface;
use sdl2::render::{Canvas};
use sdl2::ttf::{Sdl2TtfContext, Font};
use std::time::{SystemTime, Duration};
use std::env;
mod wsthread;
mod data;
mod mycanvas;
//mod gamearea;


macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut scale : f32 = 1.0;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        match &args[1].parse::<f32>() {
            Ok(v) => { scale = *v},
            Err(err) => {}
        }
    }
    let window = video_subsystem
        .window("SM Tile viewer", (550.0 * scale) as u32, (520.0 * scale) as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    canvas.set_scale(scale, scale)?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    
    
    let mut event_pump = sdl_context.event_pump()?;
    let datas = Arc::new(Mutex::<SharedData>::new(SharedData::new()));
    let datas_clone = Arc::clone(&datas);
    let wsthread = std::thread::spawn(move || {
        wsthread::wsthread(datas_clone)
    });
    let mut game_tile_data : GameTileData = GameTileData { tile_x: 0, tile_y: 0, bts: 0, bts_value: 0, clip: 0, clip_value: 0, door_stuff : 0, bts_byte : 0 };
    let mut status_font = ttf_context.load_font("FreeMonoBold.ttf", 16)?;
    //status_font.set_style(sdl2::ttf::FontStyle::BOLD);
    let mut mycanvas = mycanvas::MyCanvas::new(&mut canvas, &ttf_context, &mut status_font);
    'running: loop {
        let start_loop_time = SystemTime::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let font_game = ttf_context.load_font("FreeMonoBold.ttf", 8)?;
        /*let mut c = surf.into_canvas().unwrap();
        let mut game_area = gamearea::GameArea::new(c, &font_game)?; */
        mycanvas.clear();
        let samus : sdl2::rect::Point;
        let width : u16;
        let camera : sdl2::rect::Point;
        let draw_game : bool;
        let radius : sdl2::rect::Point;
        let usb2snes_status : String;
        let start_get_data_time = SystemTime::now();
        let map_infos : Vec<u8>;
        {
            let mutex = datas.lock().unwrap();
            draw_game = (*mutex).usb2snes_ready;
            usb2snes_status = if (*mutex).usb2snes_ready {
                String::from("Usb2Snes connection : Ready")
            } else {
                let err = (*mutex).usb2snes_error.clone();
                let error_str = data::usb2snes_to_string(err);
                format!("Usb2Snes connection : Not ready - {}", error_str)
            };
            samus = (*mutex).samus_pos;
            width = (*mutex).width;
            camera = (*mutex).camera;
            radius = (*mutex).radius;
            map_infos = (*mutex).map_data.clone();
            game_tile_data.door_stuff = (*mutex).door_stuff.clone();
        }
        let data_time_duration = start_get_data_time.elapsed().unwrap();
        mycanvas.draw_text(0, 0, usb2snes_status.as_str())?;
        //println!("{:?}", data_time_duration);
        mycanvas.draw_text(0, 10, format!("Time to get datas : {} ms", data_time_duration.as_millis()).as_str())?;
        if draw_game {
            let top_left = Point::new(256 - radius.x, 224 - radius.y);
            let bottom_right = Point::new(256 + radius.x, 224 + radius.y);
            mycanvas.draw_text(0, 20, format!("Samus coord : {},{} Camera : {} , {} - Width : {}", samus.x, samus.y, camera.x, camera.y, width).as_str())?;
            if map_infos.len() != 0 {
            for y in 0..28 {
                for x in 0..32 {
                    let tile_x = x * 16 - (camera.x & 0x000F);
                    let tile_y = y * 16 - (camera.y & 0x000F);
                    let a : i32 = ((camera.x + x * 16) & 0xFFFF) / 16 + ((((camera.y + y * 16) & 0xFFF) / 16) * (width as i32) & 0xFFFF);
                    let bts : usize = (0x6402 as usize + a as usize) % 0x10000;
                    let bts_value = map_infos[bts];
                    let clip : usize = (0x0002 + a as usize * 2) % 0x10000;
                    let clip_value : u16 = map_infos[clip + 1] as u16  * 256 + map_infos[clip] as u16;
                    let outline_index : u8 = (clip_value >> 12) as u8;
                    game_tile_data.tile_x = tile_x;
                    game_tile_data.tile_y = tile_y;
                    game_tile_data.bts = bts;
                    game_tile_data.clip = clip;
                    game_tile_data.bts_value = bts_value;
                    if OUTLINES.contains_key(&outline_index) {
                        OUTLINES[&outline_index](&mut mycanvas, &game_tile_data);
                    } else {
                        mycanvas.draw_game_box_fill(tile_x, tile_y, 16, 16, DARK_GREEN, GREY)?;
                        mycanvas.draw_game_text(tile_x + 3, tile_y - 1, format!("{:02x}", outline_index).as_str(), DARK_GREEN)?;
                    }
                }
            }
            }
            mycanvas.draw_game_box_fill(256 - radius.x, 224 - radius.y, radius.x * 2, radius.y * 2, AQUA, AQUA)?;
        }
        mycanvas.draw_box_cord_fill(0, 50, 18, 600, BLACK, BLACK)?;
        mycanvas.draw_box_cord_fill(0, 40, 580, 28, BLACK, BLACK)?;
        mycanvas.draw_box_cord_fill(518, 50, 18, 600, BLACK, BLACK)?;
        mycanvas.draw_box_cord_fill(0, 495, 580, 28, BLACK, BLACK)?;
        /*let game_surface = game_area.into_surface();
        let tex_creator = mycanvas.texture_creator();
        let game_texture = tex_creator.create_texture_from_surface(game_surface).unwrap();
        mycanvas.copy(&game_texture, None, Some(rect!(0, 50, 32 * 16, 28 * 16)))?;*/
        mycanvas.present();
        /*let d_60fps = Duration::new(0, 1_000_000_000u32 / 60);
        let end_draw_time = start_loop_time.elapsed().unwrap();
        if end_draw_time < d_60fps {
            std::thread::sleep(d_60fps - end_draw_time);
        }*/
    }

    Ok(())
}
