mod BootlegCamera;
mod vector_viewer;
// use glam::{vec2, vec3, Mat4, Vec2, Vec3};

use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: "3D Example".to_owned(),
        window_width: 1200,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

fn rotate_2d_vector(vector: Vec3, angle: f32) -> Vec3 {
    let radians = angle.to_radians();
    let cos_angle = radians.cos();
    let sin_angle = radians.sin();

    vec3(
        vector.x * cos_angle - vector.z * sin_angle,
        vector.y,
        vector.x * sin_angle + vector.z * cos_angle,
    )
}

fn draw_reference_markers() {
    // draw x-axis reference
    draw_line_3d(vec3(0., 0., 0.), vec3(3., 0., 0.), RED);
    // draw y-axis reference
    draw_line_3d(vec3(0., 0., 0.), vec3(0., 3., 0.), GREEN);
    // draw z-axis reference
    draw_line_3d(vec3(0., 0., 0.), vec3(0., 0., 3.), BLUE);
}

fn handle_input(ref mut angle: &mut f32) {
    if is_key_down(KeyCode::Left) {
        **angle -= 0.01;
    }
    if is_key_down(KeyCode::Right) {
        **angle += 0.01;
    }
}

fn construct_view_matrix(position: Vec3, target: Vec3, up: Vec3, angle: f32) -> Mat4 {
    let forward = (target - position).normalize();

    draw_text(&format!("forward view vector"), 500.0, 400.0, 20.0, RED);
    vector_viewer::mini_vector_viewer(500, 0, 500, 500, angle, forward);

    let right = forward.cross(up).normalize();
    vector_viewer::mini_vector_viewer(700, 0, 1000, 100, angle, right);

    let up = right.cross(forward).normalize();

    // print forward and right vectors

    Mat4::from_cols(
        vec4(right.x, up.x, -forward.x, 0.0),
        vec4(right.y, up.y, -forward.y, 0.0),
        vec4(right.z, up.z, -forward.z, 0.0),
        vec4(
            -right.dot(position),
            -up.dot(position),
            forward.dot(position),
            1.0,
        ),
    )
}

fn handle_player_cam_input(ref mut angle: &mut f32) {
    if is_key_down(KeyCode::Q) {
        **angle += 0.01;
    }
    if is_key_down(KeyCode::E) {
        **angle -= 0.01;
    }
}
fn handle_cam_input(ref mut pos: &mut Vec3) {
    if is_key_down(KeyCode::W) {
        pos.z -= 0.005;
    }
    if is_key_down(KeyCode::S) {
        pos.z += 0.005;
    }
    if is_key_down(KeyCode::A) {
        pos.x -= 0.005;
    }
    if is_key_down(KeyCode::D) {
        pos.x += 0.005;
    }
    if is_key_down(KeyCode::Space) {
        pos.y += 0.005;
    }
    if is_key_down(KeyCode::LeftShift) {
        pos.y -= 0.005;
    }
}

fn configure_camera(angle: &f32) {
    let piviot = vec3(0.0, 0.0, 0.0);
    let inital_offset = vec3(-20.0, 10.0, 0.0);

    let rotation = Quat::from_rotation_y(*angle);

    let camera_pos = piviot + rotation * inital_offset;

    let camera = Camera3D {
        aspect: Some(1.0),
        position: camera_pos,
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        viewport: Some((0, 0, 400, 400)),
        ..Default::default()
    };

    set_camera(&camera);
}

fn draw_game_things() {
    // array of cube points
    let cube_points = [
        (1, -1, -5),
        (1, -1, -3),
        (1, 1, -5),
        (1, 1, -3),
        (-1, -1, -5),
        (-1, -1, -3),
        (-1, 1, -5),
        (-1, 1, -3),
    ];

    // draw cubes at the points
    for (x, y, z) in cube_points.iter() {
        draw_cube(
            vec3(*x as f32, *y as f32, *z as f32),
            vec3(0.2, 0.2, 0.2),
            None,
            BLUE,
        );
    }

    // draw_cube(vec3(0., 0., -3.), vec3(0.4, 0.4, 0.4), None, BLACK);
}

fn draw_controlled_camera(pos: Vec3) {
    draw_cube(pos, vec3(0.2, 0.2, 0.2), None, YELLOW);
}

fn draw_view_view(
    controller_camera_pos: Vec3,
    angle: &f32,
    view_matrix: Mat4,
    cube_points: &[(i32, i32, i32)],
) {
    // TODO apply the x,y offset correctly from the view matrix.

    let world_offset = vec3(2000.0, 0.0, 2000.0);

    let camera_offset = vec3(-20.0, 10.0, 0.0);
    let cam_pos = world_offset + camera_offset;
    let rotation = Quat::from_rotation_y(*angle);

    let pivot = vec3(2000.0, 0.0, 2000.0);

    let cam_pos = pivot + rotation * camera_offset;

    let camera = Camera3D {
        position: cam_pos,
        aspect: Some(1.0),
        up: vec3(0.0, 1.0, 0.0),
        target: world_offset,
        viewport: Some((500, 250, 400, 400)),
        ..Default::default()
    };

    set_camera(&camera);

    // draw camera cube at origin
    draw_cube(vec3(2000.0, 0.0, 1950.), vec3(2.0, 2.0, 2.), None, YELLOW);

    draw_cube(vec3(2000.0, 0.0, 2000.), vec3(2.0, 2.0, 2.), None, YELLOW);

    for (x, y, z) in cube_points.iter() {
        let point = vec3(*x as f32, *y as f32, *z as f32);
        let transformed_point = view_matrix.transform_point3(point);

        // apply world offset
        let transformed_point = transformed_point + world_offset;

        draw_cube(transformed_point, vec3(0.2, 0.2, 0.2), None, BLACK);
        println!("Transformed Point: {:?}", transformed_point);
    }

    // draw world lines
    for x in -10..=10 {
        let start = pivot + vec3(x as f32, 0.0, -10.0);
        let end = pivot + vec3(x as f32, 0.0, 10.0);
        draw_line_3d(start, end, GRAY);
    }
    for z in -10..=10 {
        let start = pivot + vec3(-10.0, 0.0, z as f32);
        let end = pivot + vec3(10.0, 0.0, z as f32);
        draw_line_3d(start, end, GRAY);
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut angle = 0.0_f32;
    let mut controller_camera_pos = vec3(0.0, 0.0, 3.0);

    let mut controller_camera_angle = 0.0_f32;

    let cube_points = [
        (1, -1, -5),
        (1, -1, -3),
        (1, 1, -5),
        (1, 1, -3),
        (-1, -1, -5),
        (-1, -1, -3),
        (-1, 1, -5),
        (-1, 1, -3),
    ];

    loop {
        clear_background(LIGHTGRAY);

        draw_text(&format!("angle : {:.2}", angle), 20.0, 50.0, 30.0, BLACK);

        draw_text(
            &format!("angle : {:.2}", angle.to_radians()),
            20.0,
            80.0,
            30.0,
            BLACK,
        );

        handle_input(&mut angle);

        handle_cam_input(&mut controller_camera_pos);

        handle_player_cam_input(&mut controller_camera_angle);

        // print controller camera angle
        println!("Controller Camera Angle: {:.2}", controller_camera_angle);
        // print controller camera position
        println!("Controller Camera Position: {:?}", controller_camera_pos);

        // make camera target point in front of the camera rotated by controller_camera_angle
        let pvot = controller_camera_pos;
        let fwd = vec3(0.0, 0.0, -5.0);
        let rotated_fwd = Quat::from_rotation_y(controller_camera_angle);
        let controller_camera_target = pvot + rotated_fwd * fwd;

        let view_matrix = construct_view_matrix(
            controller_camera_pos,
            controller_camera_target,
            vec3(0.0, 1.0, 0.0),
            angle,
        );

        let foo = Mat4::look_at_rh(
            controller_camera_pos,
            controller_camera_target,
            vec3(0.0, 1.0, 0.0),
        );

        let view_matrix_camera_pos = view_matrix
            * vec4(
                controller_camera_pos.x,
                controller_camera_pos.y,
                controller_camera_pos.z,
                1.0,
            );

        configure_camera(&angle);

        draw_grid(20, 1., BLACK, GRAY);

        draw_game_things();

        // draw cube points with foo view matrix
        for (x, y, z) in cube_points.iter() {
            let point = vec3(*x as f32, *y as f32, *z as f32);
            let transformed_point = foo.transform_point3(point);
            draw_cube(transformed_point, vec3(0.2, 0.2, 0.2), None, BLACK);
            println!("Transformed Point: {:?}", transformed_point);
        }

        draw_controlled_camera(controller_camera_pos);

        // draw line from camera pos to camera target
        draw_line_3d(controller_camera_pos, controller_camera_target, BLUE);
        draw_cube(controller_camera_target, vec3(0.2, 0.2, 0.2), None, BLUE);

        // draw a line from the camera to looking forward
        draw_line_3d(
            controller_camera_pos,
            controller_camera_pos + vec3(0.0, 0.0, -5.0),
            RED,
        );

        draw_view_view(controller_camera_pos, &angle, view_matrix, &cube_points);

        draw_reference_markers();

        // vector_viewer::mini_vector_viewer(0, 0, 500, 500, angle);

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        draw_text("test??? : ", 10.0, 90.0, 30.0, BLACK);

        // write the view on screen
        draw_text(
            &format!("View Matrix: {:?}", view_matrix),
            10.0,
            120.0,
            20.0,
            BLACK,
        );

        next_frame().await
    }
}
