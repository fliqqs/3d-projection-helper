mod BootlegCamera;
mod vector_viewer;

use macroquad::prelude::*;

const LOOK_SPEED: f32 = 0.1;

fn conf() -> Conf {
    Conf {
        window_title: "3D Example".to_owned(),
        window_width: 1000,
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
}

fn draw_controlled_camera(pos: Vec3) {
    draw_cube(pos, vec3(0.2, 0.2, 0.2), None, YELLOW);
}

fn compute_frustum_corners(
    fov: f32,
    aspect: f32,
    znear: f32,
    zfar: f32,
) -> (Vec3, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3) {
    let tan_half_fov = (fov * 0.5).tan();

    // Near plane half extents
    let near_height = 2.0 * tan_half_fov * znear;
    let near_width = near_height * aspect;

    // Far plane half extents
    let far_height = 2.0 * tan_half_fov * zfar;
    let far_width = far_height * aspect;

    // Define corners in camera/view space
    let near_top_left = vec3(-near_width / 2.0, near_height / 2.0, -znear);
    let near_top_right = vec3(near_width / 2.0, near_height / 2.0, -znear);
    let near_bottom_left = vec3(-near_width / 2.0, -near_height / 2.0, -znear);
    let near_bottom_right = vec3(near_width / 2.0, -near_height / 2.0, -znear);

    let far_top_left = vec3(-far_width / 2.0, far_height / 2.0, -zfar);
    let far_top_right = vec3(far_width / 2.0, far_height / 2.0, -zfar);
    let far_bottom_left = vec3(-far_width / 2.0, -far_height / 2.0, -zfar);
    let far_bottom_right = vec3(far_width / 2.0, -far_height / 2.0, -zfar);

    (
        near_top_left,
        near_top_right,
        near_bottom_left,
        near_bottom_right,
        far_top_left,
        far_top_right,
        far_bottom_left,
        far_bottom_right,
    )
}

fn build_projection_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    Mat4::perspective_rh(fov, aspect_ratio, near, far)
}

fn world_to_screen_single_point(
    world_pos: Vec3,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    screen_w: f32,
    screen_h: f32,
) -> Option<Vec2> {
    let clip_space_pos =
        projection_matrix * view_matrix * vec4(world_pos.x, world_pos.y, world_pos.z, 1.0);

    // If w <= 0, the point is behind the camera
    if clip_space_pos.w <= 0.0 {
        return None;
    }

    let ndc_x = clip_space_pos.x / clip_space_pos.w;
    let ndc_y = clip_space_pos.y / clip_space_pos.w;
    let ndc_z = clip_space_pos.z / clip_space_pos.w;

    // Reject points outside the clip volume
    if ndc_x < -1.0 || ndc_x > 1.0 || ndc_y < -1.0 || ndc_y > 1.0 || ndc_z < -1.0 || ndc_z > 1.0 {
        return None;
    }

    // Convert NDC → screen space
    let screen_x = (ndc_x + 1.0) * 0.5 * screen_w;
    let screen_y = (1.0 - (ndc_y + 1.0) * 0.5) * screen_h;

    Some(vec2(screen_x, screen_y))
}

fn draw_view_frustum(fov: f32, aspect: f32, znear: f32, zfar: f32, view_matrix: Mat4) {
    let frustum_corners = compute_frustum_corners(fov, aspect, znear, zfar);
    let inv_view = view_matrix.inverse();

    // draw lines around near and far planes
    let near_top_left = inv_view.transform_point3(frustum_corners.0);
    let near_top_right = inv_view.transform_point3(frustum_corners.1);
    let near_bottom_left = inv_view.transform_point3(frustum_corners.2);
    let near_bottom_right = inv_view.transform_point3(frustum_corners.3);
    let far_top_left = inv_view.transform_point3(frustum_corners.4);
    let far_top_right = inv_view.transform_point3(frustum_corners.5);
    let far_bottom_left = inv_view.transform_point3(frustum_corners.6);
    let far_bottom_right = inv_view.transform_point3(frustum_corners.7);

    // Near plane
    draw_line_3d(near_top_left, near_top_right, RED);
    draw_line_3d(near_top_right, near_bottom_right, RED);
    draw_line_3d(near_bottom_right, near_bottom_left, RED);
    draw_line_3d(near_bottom_left, near_top_left, RED);

    // Far plane
    draw_line_3d(far_top_left, far_top_right, BLUE);
    draw_line_3d(far_top_right, far_bottom_right, BLUE);
    draw_line_3d(far_bottom_right, far_bottom_left, BLUE);
    draw_line_3d(far_bottom_left, far_top_left, BLUE);

    // Connect near and far planes
    draw_line_3d(near_top_left, far_top_left, GREEN);
    draw_line_3d(near_top_right, far_top_right, GREEN);
    draw_line_3d(near_bottom_right, far_bottom_right, GREEN);
    draw_line_3d(near_bottom_left, far_bottom_left, GREEN);
}

fn world_space_to_view(
    view_matrix: Mat4,
    cube_points: &[(i32, i32, i32)],
    projection_matrix: Mat4,
) {
    let first_person_pannel_offset = vec2(100.0, 200.0);

    for (x, y, z) in cube_points.iter() {
        let world_pos = vec3(*x as f32, *y as f32, *z as f32);
        let screen_pos = world_to_screen_single_point(
            world_pos,
            view_matrix,
            projection_matrix,
            200.0, // width of panel
            200.0, // height of panel
        );

        if let Some(screen_pos) = screen_pos {
            draw_circle(
                screen_pos.x + first_person_pannel_offset.x,
                screen_pos.y + first_person_pannel_offset.y,
                5.0,
                RED,
            );
        }
    }
}

fn draw_view_view(
    controller_camera_pos: Vec3,
    angle: &f32,
    view_matrix: Mat4,
    cube_points: &[(i32, i32, i32)],
) {
    let world_offset = vec3(2000.0, 0.0, 2000.0);
    let camera_offset = vec3(0.0, 10.0, 20.0);

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

fn draw_panel_labels() {
    draw_text("World Space View", 60.0, 400.0, 30.0, BLACK);
    draw_text("First Person View", 60.0, 100.0, 30.0, BLACK);
    draw_text("View Matrix World", 600.0, 100.0, 30.0, BLACK);
    draw_text("Look at vector", 600.0, 550.0, 30.0, BLACK);
}

#[macroquad::main(conf)]
async fn main() {
    let mut angle = 0.0_f32;
    let mut controller_camera_pos = vec3(0.0, 0.0, 3.0);

    let mut controller_camera_angle = 0.0_f32;

    // handle mouse stuff
    let mut yaw: f32 = 1.0;
    let mut last_mouse_position: Vec2 = mouse_position().into();
    let mut grabbed = false;
    set_cursor_grab(grabbed);
    show_mouse(true);

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
        let delta = get_frame_time();
        clear_background(LIGHTGRAY);

        // if is_key_pressed(KeyCode::Tab) {
        //     grabbed = !grabbed;
        //     set_cursor_grab(grabbed);
        //     show_mouse(!grabbed);
        // }

        if grabbed {}

        // get mouse movements
        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;

        handle_input(&mut angle);
        handle_cam_input(&mut controller_camera_pos);
        handle_player_cam_input(&mut controller_camera_angle);

        // make camera target point in front of the camera rotated by controller_camera_angle
        let pvot = controller_camera_pos;
        let fwd = vec3(0.0, 0.0, -1.0);
        let rotated_fwd = Quat::from_rotation_y(controller_camera_angle);

        let controller_camera_target = pvot + rotated_fwd * fwd;

        let view_matrix = construct_view_matrix(
            controller_camera_pos,
            controller_camera_target,
            vec3(0.0, 1.0, 0.0),
            angle,
        );

        let projection_matrix = build_projection_matrix(
            std::f32::consts::FRAC_PI_3,
            1.0,   // aspect ratio
            0.1,   // near plane
            100.0, // far plane
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

        draw_view_frustum(
            std::f32::consts::FRAC_PI_3,
            1.0,   // aspect ratio
            0.1,   // near plane
            100.0, // far plane
            view_matrix,
        );

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

        set_default_camera();

        draw_panel_labels();

        draw_text(
            "WASD - Camera Movement | QE - Rotate Camera | <- -> rotate world view",
            10.0,
            20.0,
            30.0,
            BLACK,
        );

        world_space_to_view(view_matrix, &cube_points, projection_matrix);

        next_frame().await
    }
}
