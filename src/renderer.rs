use crate::color::Color;
use crate::engine::render;
use crate::engine::Scene;
use crate::geometry::Object;
use crate::geometry::Vector3;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[allow(unused)]
pub fn render_scene_console(scene: &Scene) -> Result<(), String> {
    let screen = render(scene);
    for y in 0..scene.camera.y_resolution {
        for x in 0..scene.camera.x_resolution {
            let ansi_color: ansi_term::Color = screen[&(x, y)].into();
            print!("{}", ansi_color.paint("█"));
        }
        println!();
    }
    Ok(())
}

impl From<Color> for ansi_term::Color {
    fn from(item: Color) -> Self {
        ansi_term::Color::RGB(
            (item.red * 255f64).round() as u8,
            (item.green * 255f64).round() as u8,
            (item.blue * 255f64).round() as u8,
        )
    }
}

#[allow(unused)]
pub fn render_scene_file(scene: &Scene) -> Result<(), String> {
    let screen = render(scene);
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(
        scene.camera.x_resolution as u32,
        scene.camera.y_resolution as u32,
    );

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = screen[&(x as u16, y as u16)].into();
    }
    imgbuf.save("output.png").unwrap();

    Ok(())
}

impl From<Color> for image::Rgb<u8> {
    fn from(item: Color) -> Self {
        image::Rgb([
            ((item.red as f32) * 255.0) as u8,
            ((item.green as f32) * 255.0) as u8,
            ((item.blue as f32) * 255.0) as u8,
        ])
    }
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
                    let sdl2_color: sdl2::pixels::Color = screen[&(x, y)].into();
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

impl From<Color> for sdl2::pixels::Color {
    fn from(item: Color) -> Self {
        sdl2::pixels::Color::RGB(
            ((item.red as f32) * 255.0) as u8,
            ((item.green as f32) * 255.0) as u8,
            ((item.blue as f32) * 255.0) as u8,
        )
    }
}
