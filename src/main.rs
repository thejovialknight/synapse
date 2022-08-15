extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::BlendMode;

use crate::input::Input;
mod input;

use crate::shapes::Circle;
use crate::shapes::Line;
mod shapes;

use crate::vec2::Vec2;
mod vec2;

use crate::render::draw_circle;
use crate::render::draw_line;
mod render;

use crate::table::Table;
mod table;


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?; 

    let window = video_subsystem
        .window("Synapse", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.set_blend_mode(BlendMode::Blend);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut input = Input{quit: false};

    'running: loop {
        input.set(&mut event_pump); 
        if input.quit { break 'running }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        // Must have alpha set to 254 for some fucking reason. Eventually just make your own damn
        // line algorithm.
        let fg_color = Color::RGBA(200, 200, 200, 254);
        draw_line(&mut canvas, Line{start: Vec2{x: 20.0, y: 20.0}, end: Vec2{x: 90.0, y: 80.0}}, fg_color);
        draw_circle(&mut canvas, Circle{origin: Vec2{x: 20.0, y: 20.0}, radius: 10.0}, fg_color);
        canvas.present();
    }

    Ok(())
}
