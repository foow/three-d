
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};
    
    let mut window = Window::new_default("Wireframe").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let scene_center = vec3(0.0, 2.0, 0.0);
    let scene_radius = 6.0;
    let mut renderer = DeferredPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(&gl, scene_center + scene_radius * vec3(0.6, 0.3, 1.0).normalize(), scene_center, vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    Loader::load(&["./examples/assets/models/suzanne.3d"], move |loaded| {
        let mut cpu_mesh = ThreeD::parse(loaded.get("./examples/assets/models/suzanne.3d").unwrap().as_ref().unwrap()).unwrap();
        cpu_mesh.diffuse_intensity = Some(0.2);
        cpu_mesh.specular_intensity = Some(0.4);
        cpu_mesh.specular_power = Some(20.0);
        let model = Mesh::from_cpu_mesh(&gl, &cpu_mesh).unwrap();

        let mut edges = Edges::new(&gl, cpu_mesh.indices.as_ref().unwrap(), &cpu_mesh.positions, 0.007);
        edges.diffuse_intensity = 0.8;
        edges.specular_intensity = 0.2;
        edges.specular_power = 5.0;
        edges.color = vec3(0.7, 0.2, 0.2);

        let mut vertices = Vertices::new(&gl, &cpu_mesh.positions, 0.015);
        vertices.diffuse_intensity = 0.8;
        vertices.specular_intensity = 0.2;
        vertices.specular_power = 5.0;
        vertices.color = vec3(0.9, 0.2, 0.2);

        let plane = Mesh::from_cpu_mesh(&gl, &CPUMesh {
            positions: vec!(-10000.0, 0.0, 10000.0, 10000.0, 0.0, 10000.0, 0.0, 0.0, -10000.0),
            normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
            diffuse_intensity: Some(0.2),
            specular_intensity: Some(0.4),
            specular_power: Some(20.0),
            ..Default::default()
        }).unwrap();

        let mut spot_light0 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &vec3(5.0, 7.0, 5.0),
                                             &vec3(-1.0, -1.0, -1.0), 25.0, 0.1, 0.001, 0.0001).unwrap();
        let mut spot_light1 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &vec3(-5.0, 7.0, 5.0),
                                             &vec3(1.0, -1.0, -1.0), 25.0, 0.1, 0.001, 0.0001).unwrap();
        let mut spot_light2 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &vec3(-5.0, 7.0, -5.0),
                                             &vec3(1.0, -1.0, 1.0), 25.0, 0.1, 0.001, 0.0001).unwrap();
        let mut spot_light3 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &vec3(5.0, 7.0, -5.0),
                                             &vec3(-1.0, -1.0, 1.0), 25.0, 0.1, 0.001, 0.0001).unwrap();

        let render_scene = |camera: &Camera| {
            let transformation = Mat4::from_translation(vec3(0.0, 2.0, 0.0));
            model.render(&transformation, camera);
            edges.render(&transformation, camera);
            vertices.render(&transformation, camera);
        };
        spot_light0.generate_shadow_map(50.0, 512, &render_scene);
        spot_light1.generate_shadow_map(50.0, 512, &render_scene);
        spot_light2.generate_shadow_map(50.0, 512, &render_scene);
        spot_light3.generate_shadow_map(50.0, 512, &render_scene);

        // main loop
        let mut rotating = false;
        window.render_loop(move |frame_input|
            {
                camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

                for event in frame_input.events.iter() {
                    match event {
                        Event::MouseClick { state, button, .. } => {
                            rotating = *button == MouseButton::Left && *state == State::Pressed;
                        },
                        Event::MouseMotion { delta } => {
                            if rotating {
                                camera.rotate(delta.0 as f32, delta.1 as f32);
                            }
                        },
                        Event::MouseWheel { delta } => {
                            camera.zoom(*delta as f32);
                        },
                        _ => {}
                    }
                }

                // Geometry pass
                renderer.geometry_pass(width, height, &|| {
                    let transformation = Mat4::from_translation(vec3(0.0, 2.0, 0.0));
                    state::cull(&gl, state::CullType::Back);
                    model.render(&transformation, &camera);
                    edges.render(&transformation, &camera);
                    vertices.render(&transformation, &camera);
                    plane.render(&Mat4::identity(), &camera);
                }).unwrap();

                // Light pass
                Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.1, 0.1, 0.1, 1.0)), None, &|| {
                    renderer.light_pass(&camera, None, &[], &[&spot_light0, &spot_light1, &spot_light2, &spot_light3], &[]).unwrap();
                }).unwrap();

                if let Some(ref path) = screenshot_path {
                    #[cfg(target_arch = "x86_64")]
                        Screen::save_color(path, &gl, 0, 0, width, height).unwrap();
                    std::process::exit(1);
                }
            }).unwrap();
    });
}