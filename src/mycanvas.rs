use std::ops::RangeBounds;

use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::{Sdl2TtfContext, Font};
use sdl2::pixels::{Color};
use sdl2::rect::{Rect, Point};

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);
pub struct MyCanvas<'a> {
    canvas : &'a mut Canvas<Window>,
    ttf_context : &'a Sdl2TtfContext,
    default_font : &'a Font<'a, 'a>,
    game_font : Font<'a, 'a>
}
const GAME_X_OFFSET : i32 = 20;
const GAME_Y_OFFSET : i32 = 60;

impl<'a> MyCanvas<'a> {
    pub fn new(c : &'a mut Canvas<Window>, ttf : &'a Sdl2TtfContext, font : &'a Font) -> MyCanvas<'a> {
        MyCanvas {
            canvas : c,
            ttf_context : ttf,
            default_font : font,
            game_font  : ttf.load_font("FreeMonoBold.ttf", 11).unwrap()
        }
    }
    pub fn clear(&mut self) {
        self.canvas.clear();
    }
    pub fn present(&mut self) {
        self.canvas.present();
    }
    pub fn draw_text(&mut self, x : i32, y : i32, text : &str) -> Result<(), String> {
        let usb2snes_status_surface = self.default_font.render(text)
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string())?;
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
        .create_texture_from_surface(&usb2snes_status_surface)
        .map_err(|e| e.to_string())?;
        let dest_rect : Rect = rect!(
            x, y,
            usb2snes_status_surface.rect().width(),
            usb2snes_status_surface.rect().height());
        self.canvas.copy(&texture, None, Some(dest_rect))?;
        Ok(())
    }
    pub fn draw_game_text(&mut self, x : i32, y : i32, text : &str, color : Color) -> Result<(), String> {
        let usb2snes_status_surface = self.game_font.render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
        .create_texture_from_surface(&usb2snes_status_surface)
        .map_err(|e| e.to_string())?;
        let dest_rect : Rect = rect!(
            x + GAME_X_OFFSET, y + GAME_Y_OFFSET,
            usb2snes_status_surface.rect().width(),
            usb2snes_status_surface.rect().height());
        self.canvas.copy(&texture, None, Some(dest_rect))?;
        Ok(())
    }
    pub fn draw_box(&mut self, top_left : Point, bottom_right : Point, color : Color) -> Result<(), String> {
        let rect = rect!(top_left.x, top_left.y, bottom_right.x - top_left.x, bottom_right.y - top_left.y);
        let old_color = self.canvas.draw_color();
        self.canvas.set_draw_color(color);
        self.canvas.draw_rect(rect)?;
        self.canvas.set_draw_color(old_color);
        Ok(())
    }
    pub fn draw_box_cord(&mut self, x : i32, y : i32, w : i32, h : i32, color : Color) ->Result<(), String> {
        let rect = rect!(x, y, w, h);
        let old_color = self.canvas.draw_color();
        self.canvas.set_draw_color(color);
        self.canvas.draw_rect(rect)?;
        self.canvas.set_draw_color(old_color);
        Ok(())
    }
    pub fn draw_box_cord_fill(&mut self, x : i32, y : i32, w : i32, h : i32, color : Color, color_fill : Color) ->Result<(), String> {
        let rect = rect!(x, y, w, h);
        let old_color = self.canvas.draw_color();
        self.canvas.set_draw_color(color_fill);
        self.canvas.fill_rect(rect)?;
        self.canvas.set_draw_color(color);
        self.canvas.draw_rect(rect)?;
        self.canvas.set_draw_color(old_color);
        Ok(())
    }

    pub fn set_draw_color(&mut self, color : Color) {
        self.canvas.set_draw_color(color);
    }
    pub fn draw_game_line(&mut self, x : i32, y : i32, x2 : i32, y2 : i32) ->Result<(), String> {
        self.canvas.draw_line(Point::new(x + GAME_X_OFFSET, y + GAME_Y_OFFSET), Point::new(x2 + GAME_X_OFFSET, y2 + GAME_Y_OFFSET))
    }

    pub fn draw_game_box(&mut self, x : i32, y : i32, w : i32, h : i32, color : Color) ->Result<(), String> {
        self.draw_box_cord(x + GAME_X_OFFSET, y + GAME_Y_OFFSET, w, h, color)
    }
    pub fn draw_game_box_fill(&mut self, x : i32, y : i32, w : i32, h : i32, color : Color, color_fill : Color) -> Result<(), String> {
        self.draw_box_cord_fill(x + GAME_X_OFFSET, y + GAME_Y_OFFSET, w, h, color, color_fill)
    }

    /*pub fn copy(&mut self, texture : &Texture<'_>, src : Option<Rect>, dst : Option<Rect>) -> Result<(), String> {
        self.canvas.copy(texture, src, dst)?;
        Ok(())
    }
    pub fn texture_creator(&mut self) -> TextureCreator<WindowContext> {
        return self.canvas.texture_creator()
    }*/
}