use raytracer_engine::color::Color;
use raytracer_engine::geometry::Vector3;
use raytracer_engine::geometry::Object;
use raytracer_engine::engine::render;
use raytracer_engine::engine::Scene;
use raytracer_engine::sample::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    let mut scene = get_mesh();
    render_scene_sdl2(&mut scene)?;
    Ok(())
}

pub fn render_scene_sdl2(scene: &mut Scene) -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width: u32 = scene.camera.x_resolution.into();
    let height: u32 = scene.camera.y_resolution.into();

    let window = video_subsystem
        .window("rust raytracer", width, height)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(sdl2::pixels::PixelFormatEnum::RGBA8888, width, height)
        .map_err(|e| e.to_string())?;

    render_frame_scene_sdl2(scene, &mut canvas, &mut texture, width, height)?;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            let mut render = false;
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    let to_move = get_object_to_move(scene);
                    to_move.translate(&Vector3 {
                        x: 0f64,
                        y: 0f64,
                        z: -1f64,
                    });
                    render = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    let to_move = get_object_to_move(scene);
                    to_move.translate(&Vector3 {
                        x: 0f64,
                        y: 0f64,
                        z: 1f64,
                    });
                    render = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let to_move = get_object_to_move(scene);
                    to_move.translate(&Vector3 {
                        x: -1f64,
                        y: 0f64,
                        z: 0f64,
                    });
                    render = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let to_move = get_object_to_move(scene);
                    to_move.translate(&Vector3 {
                        x: 1f64,
                        y: 0f64,
                        z: 0f64,
                    });
                    render = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let to_move = get_object_to_move(scene);
                    to_move.translate(&Vector3 {
                        x: 0f64,
                        y: 1f64,
                        z: 0f64,
                    });
                    render = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let to_move = get_object_to_move(scene);
                    to_move.translate(&Vector3 {
                        x: 0f64,
                        y: -1f64,
                        z: 0f64,
                    });
                    render = true;
                }
                _ => {}
            }
            if render {
                render_frame_scene_sdl2(scene, &mut canvas, &mut texture, width, height)?;
            }
        }
    }

    Ok(())
}

fn get_object_to_move<'a>(scene: &'a mut Scene) -> &'a mut Object {
    &mut scene
        .objects
        .iter_mut()
        .filter(|obj| obj.id == 1)
        .nth(0)
        .unwrap()
        .geometry
}

pub fn render_frame_scene_sdl2(
    scene: &Scene,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &mut sdl2::render::Texture,
    width: u32,
    height: u32,
) -> Result<(), String> {
    let screen = render(scene);
    canvas
        .with_texture_canvas(texture, |texture_canvas| {
            texture_canvas.clear();
            for x in 0..scene.camera.x_resolution {
                for y in 0..scene.camera.y_resolution {
                    let sdl2_color: sdl2::pixels::Color = color_to_sdl2(screen[&(x, y)]);
                    texture_canvas.set_draw_color(sdl2_color);
                    texture_canvas
                        .draw_point(sdl2::rect::Point::new(x as i32, y as i32))
                        .unwrap();
                }
            }
        })
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
    canvas.clear();
    canvas.copy(
        &texture,
        None,
        Some(sdl2::rect::Rect::new(0, 0, width, height)),
    )?;
    canvas.present();

    Ok(())
}

fn color_to_sdl2(item: Color) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB(
        ((item.red as f32) * 255.0) as u8,
        ((item.green as f32) * 255.0) as u8,
        ((item.blue as f32) * 255.0) as u8,
    )
}