extern crate sdl2;
extern crate lerp;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::gfx::primitives::DrawRenderer;

use crate::shapes::Circle;
use crate::shapes::Line;
use crate::vec2::Vec2;

pub fn draw_line(canvas: &mut Canvas<Window>, line: Line, color: Color) {
    let thickness = 4.0;

    // Vector of our original line
    let line_vector = Vec2{x: line.end.x - line.start.x, y: line.end.y - line.start.y}; 

    // Unit vector perpendicular to the original line
    let perpendicular_unit = Vec2{x: line_vector.y, y: -line_vector.x}.normalize().multiply_by_f32(thickness / 2.0);

    let line1 = line.translate(&perpendicular_unit);
    let line2 = line.translate(&-perpendicular_unit);

    let x_vertices = [line1.end.x as i16, line1.start.x as i16, line2.start.x as i16, line2.end.x as i16]; 
    let y_vertices = [line1.end.y as i16, line1.start.y as i16, line2.start.y as i16, line2.end.y as i16]; 
    
    /* Doesn't seem to be needed. Perhaps making it a TOUCH smaller might improve AA, but looks
     * pretty alright to me at the moment.
     *
    let offset_perpendicular = perpendicular_unit.subtract_by_f32(0.0);
    let inner_line1 = line1.translate(&-perpendicular_unit.normalize());
    let inner_line2 = line2.translate(&perpendicular_unit.normalize());

    let inner_x_vertices = [inner_line1.end.x as i16, inner_line1.start.x as i16, inner_line2.start.x as i16, inner_line2.end.x as i16]; 
    let inner_y_vertices = [inner_line1.end.y as i16, inner_line1.start.y as i16, inner_line2.start.y as i16, inner_line2.end.y as i16]; 
    */

    canvas.filled_polygon(&x_vertices, &y_vertices, color);
    canvas.aa_polygon(&x_vertices, &y_vertices, color);
}

pub fn draw_circle(canvas: &mut Canvas<Window>, circle: Circle, color: Color) {
    let r = circle.radius;

    for i in 0..(2 * r as i32) {
        for j in 0..2 * r as i32 {
            let x = j as f32;
            let y = i as f32;

            let delta_x = r - x;
            let delta_y = r - y;
            let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();
            let value = (((r - distance).clamp(0.0, 1.0)) * 255.0) as u8;

            canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, value));

            let offset = Vec2{x: circle.origin.x + x - r, y: circle.origin.y + y - r};
            canvas.draw_point(Point::new(offset.x as i32, offset.y as i32));
        }
    }
}

