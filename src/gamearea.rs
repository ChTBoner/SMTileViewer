use sdl2::surface::{Surface, SurfaceRef};
use sdl2::render::{Canvas, Texture, TextureValueError};
use sdl2::rect::{Rect, Point};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::ttf::{Sdl2TtfContext, Font};

pub struct GameArea<'s> {
    canvas : Canvas<Surface<'s>>,
    font : &'s Font<'s, 's>
}

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

impl<'s> GameArea<'s> {
    pub fn new(cv : Canvas<Surface<'s>>, f : &'s Font) -> Result<GameArea<'s>, String> {
        //let surf = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
        Ok(GameArea {
            canvas : cv,
            font : f
        })
    }
    pub fn draw_text(&mut self, x : i32, y : i32, text : &str) -> Result<(), String> {
        let usb2snes_status_surface = self.font.render(text)
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

    /*pub fn into_texture(&mut self) -> Result<Texture, TextureValueError> {
        let tex_creator = self.canvas.texture_creator();
        let plop = tex_creator.create_texture_from_surface(self.canvas.surface()).unwrap();
        return Ok(plop);
    }*/
}