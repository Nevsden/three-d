
use three_d::math::*;
use three_d::core::*;
use three_d::window::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let window = Window::new("Triangle", Some((1280, 720))).unwrap();
    let context = window.gl();

    // Camera
    let mut camera = Camera::new_perspective(&context, vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                             degrees(45.0), window.viewport().aspect(), 0.1, 10.0);

    let positions: Vec<f32> = vec![
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0,// bottom left
        0.0,  0.5, 0.0 // top
    ];
    let position_buffer = VertexBuffer::new_with_static_f32(&context, &positions).unwrap();
    let colors: Vec<f32> = vec![
        1.0, 0.0, 0.0,   // bottom right
        0.0, 1.0, 0.0,   // bottom left
        0.0, 0.0, 1.0    // top
    ];
    let color_buffer = VertexBuffer::new_with_static_f32(&context, &colors).unwrap();

    let program = Program::from_source(&context,
                                       include_str!("../assets/shaders/color.vert"),
                                       include_str!("../assets/shaders/color.frag")).unwrap();

    // main loop
    let mut time = 0.0;
    window.render_loop(move |frame_input|
    {
        time += frame_input.elapsed_time as f32;
        camera.set_aspect(frame_input.viewport.aspect());

        Screen::write(&context, &ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0), || {
            program.use_attribute_vec3_float(&position_buffer, "position")?;
            program.use_attribute_vec3_float(&color_buffer, "color")?;

            let world_view_projection = camera.get_projection() * camera.get_view() * Mat4::from_angle_y(radians(time * 0.005));
            program.add_uniform_mat4("worldViewProjectionMatrix", &world_view_projection)?;

            program.draw_arrays(RenderStates::default(), frame_input.viewport, 3);
            Ok(())
        }).unwrap();

        #[cfg(target_arch = "x86_64")]
        if let Some(ref path) = screenshot_path {
            use three_d::io::*;
            let pixels = Screen::read_color(&context, frame_input.viewport).unwrap();
            Saver::save_pixels(path, &pixels, frame_input.viewport.width, frame_input.viewport.height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}