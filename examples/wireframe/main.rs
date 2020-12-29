
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
    let mut renderer = PhongDeferredPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(&gl, scene_center + scene_radius * vec3(0.6, 0.3, 1.0).normalize(), scene_center, vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    Loader::load(&["./examples/assets/suzanne.obj", "./examples/assets/suzanne.mtl"], move |loaded|
    {
        let (mut meshes, mut materials)  = Obj::parse(loaded, "./examples/assets/suzanne.obj").unwrap();
        let cpu_mesh = meshes.remove(0);
        let mut cpu_material = materials.remove(0);
        cpu_material.diffuse_intensity = Some(0.2);
        cpu_material.specular_intensity = Some(0.4);
        cpu_material.specular_power = Some(20.0);
        let model = renderer.new_mesh(&cpu_mesh, &renderer.new_material(&cpu_material).unwrap()).unwrap();

        let wireframe_material = PhongMaterial {
            name: "wireframe".to_string(),
            diffuse_intensity: 0.8,
            specular_intensity: 0.2,
            specular_power: 5.0,
            color_source: ColorSource::Color(vec4(0.9, 0.2, 0.2, 1.0))
        };
        let edges = renderer.new_instanced_mesh(&edge_transformations(&cpu_mesh), &CPUMesh::cylinder(0.007, 1.0, 10, 1), &wireframe_material).unwrap();
        let vertices = renderer.new_instanced_mesh(&vertex_transformations(&cpu_mesh), &CPUMesh::sphere(0.015), &wireframe_material).unwrap();

        let plane = renderer.new_mesh(
            &CPUMesh {
                positions: vec!(-10000.0, -1.0, 10000.0, 10000.0, -1.0, 10000.0, 0.0, -1.0, -10000.0),
                normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
                ..Default::default()},
            &PhongMaterial {color_source: ColorSource::Color(vec4(1.0, 1.0, 1.0, 1.0)),
                diffuse_intensity: 0.2,
                specular_intensity: 0.4,
                specular_power: 20.0, ..Default::default()}
        ).unwrap();

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
            model.render_geometry(&transformation, camera)?;
            edges.render_geometry(&transformation, camera)?;
            vertices.render_geometry(&transformation, camera)?;
            Ok(())
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
                        Event::Key { state, kind } => {
                            if kind == "R" && *state == State::Pressed
                            {
                                renderer.next_debug_type();
                                println!("{:?}", renderer.debug_type());
                            }
                        }
                    }
                }

                // Geometry pass
                renderer.geometry_pass(width, height, &|| {
                    let transformation = Mat4::from_translation(vec3(0.0, 2.0, 0.0));
                    state::cull(&gl, state::CullType::Back);
                    model.render_geometry(&transformation, &camera)?;
                    edges.render_geometry(&transformation, &camera)?;
                    vertices.render_geometry(&transformation, &camera)?;
                    plane.render_geometry(&Mat4::identity(), &camera)?;
                    Ok(())
                }).unwrap();

                // Light pass
                renderer.render_to_screen(&camera, None, &[], &[&spot_light0, &spot_light1, &spot_light2, &spot_light3], &[], width, height).unwrap();
                
                #[cfg(target_arch = "x86_64")]
                if let Some(ref path) = screenshot_path {
                    let pixels = Screen::read_color(&gl, 0, 0, width, height).unwrap();
                    Saver::save_pixels(path, &pixels, width, height).unwrap();
                    std::process::exit(1);
                }
            }).unwrap();
    });
}

fn vertex_transformations(cpu_mesh: &CPUMesh) -> Vec<Mat4>
{
    let mut iter = cpu_mesh.positions.iter();
    let mut vertex_transformations = Vec::new();
    while let Some(v) = iter.next() {
        vertex_transformations.push(Mat4::from_translation(vec3(*v, *iter.next().unwrap(), *iter.next().unwrap())));
    }
    vertex_transformations
}

fn edge_transformations(cpu_mesh: &CPUMesh) -> Vec<Mat4>
{
    let mut edge_transformations = std::collections::HashMap::new();
    let indices = cpu_mesh.indices.as_ref().unwrap();
    for f in 0..indices.len()/3 {
        let mut fun = |i1, i2| {
            let p1 = vec3(cpu_mesh.positions[i1 * 3], cpu_mesh.positions[i1 * 3 + 1], cpu_mesh.positions[i1 * 3 + 2]);
            let p2 = vec3(cpu_mesh.positions[i2 * 3], cpu_mesh.positions[i2 * 3 + 1], cpu_mesh.positions[i2 * 3 + 2]);
            let scale = Mat4::from_nonuniform_scale((p1-p2).magnitude(), 1.0, 1.0);
            let rotation = rotation_matrix_from_dir_to_dir(vec3(1.0, 0.0, 0.0), (p2-p1).normalize());
            let translation = Mat4::from_translation(p1);
            let key = if i1 < i2 {(i1, i2)} else {(i2, i1)};
            edge_transformations.insert(key, translation * rotation * scale);
        };
        let i1 = indices[3*f] as usize;
        let i2 = indices[3*f+1] as usize;
        let i3 = indices[3*f+2] as usize;
        fun(i1, i2);
        fun(i1, i3);
        fun(i2, i3);
    }
    edge_transformations.drain().map(|(_, v)| v).collect::<Vec<Mat4>>()
}