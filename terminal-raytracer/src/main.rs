use raytracer_engine::color::Color;
use raytracer_engine::engine::render;
use raytracer_engine::engine::Scene;
use raytracer_engine::sample::*;

fn main() -> Result<(), String> {
    let mut scene = get_simple_mesh();
    scene.camera.x_resolution /= 10;
    scene.camera.y_resolution /= 10;
    render_scene_console(&scene)?;

    Ok(())
}

pub fn render_scene_console(scene: &Scene) -> Result<(), String> {
    let screen = render(scene);
    for y in 0..scene.camera.y_resolution {
        for x in 0..scene.camera.x_resolution {
            let ansi_color: ansi_term::Color = color_to_ansi(screen[&(x, y)]);
            print!("{}", ansi_color.paint("█"));
        }
        println!();
    }
    Ok(())
}

fn color_to_ansi(item: Color) -> ansi_term::Color {
    ansi_term::Color::RGB(
        (item.red * 255f64).round() as u8,
        (item.green * 255f64).round() as u8,
        (item.blue * 255f64).round() as u8,
    )
}
