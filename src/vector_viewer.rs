use macroquad::prelude::*;
// use glam::{Vec3, Quat, Mat4};

// const for vector view height and width
const VECTOR_VIEW_WIDTH: i32 = 200;

pub(crate) fn mini_vector_viewer(x: i32, y: i32, x_offset: i32, z_offset: i32, y_angle: f32, vec_to_draw: Vec3) {
    // let forward = vec3(0.9622505, -0.19245009, -0.19245009).normalize();

    let forward = vec_to_draw.normalize();

    let mut camera_pos = vec3(-5. + x_offset as f32, 2. as f32, 0. + z_offset as f32);

    let camera = Camera3D {
        position: camera_pos,
        aspect: Some(1.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(x_offset as f32, 0.0, z_offset as f32),
        viewport: Some((x, y, VECTOR_VIEW_WIDTH, VECTOR_VIEW_WIDTH)),
        ..Default::default()
    };
    set_camera(&camera);


    let pivot = vec3(x_offset as f32, 0.0, z_offset as f32);
    let rotation = Quat::from_rotation_y(y_angle);

    draw_line_3d(pivot, pivot + rotation * vec3(2.0, 0.0, 0.0), RED);
    draw_cube(
        pivot + rotation * vec3(2.0, 0.0, 0.0),
        vec3(0.2, 0.2, 0.2),
        None,
        RED,
    );
    draw_line_3d(pivot, pivot + rotation * vec3(0.0, 2.0, 0.0), GREEN);
    draw_cube(
        pivot + rotation * vec3(0.0, 2.0, 0.0),
        vec3(0.2, 0.2, 0.2),
        None,
        GREEN,
    );
    draw_line_3d(pivot, pivot + rotation * vec3(0.0, 0.0, 2.0), BLUE);
    draw_cube(
        pivot + rotation * vec3(0.0, 0.0, 2.0),
        vec3(0.2, 0.2, 0.2),
        None,
        BLUE,
    );

    // draw forward vector and a cube at the end of it
    let forward_end = pivot + rotation * forward * 2.0;
    draw_line_3d(pivot, forward_end, YELLOW);
    draw_cube(forward_end, vec3(0.2, 0.2, 0.2), None, YELLOW);

    // draw cube at pivot
    draw_cube(pivot, vec3(0.2, 0.2, 0.2), None, RED);

    // draw world grid
    for x in -10..=10 {
        let start = pivot + rotation * vec3(x as f32, 0.0, -10.0);
        let end = pivot + rotation * vec3(x as f32, 0.0, 10.0);
        draw_line_3d(start, end, GRAY);
    }
    for z in -10..=10 {
        let start = pivot + rotation * vec3(-10.0, 0.0, z as f32);
        let end = pivot + rotation * vec3(10.0, 0.0, z as f32);
        draw_line_3d(start, end, GRAY);
    }
}

// // draw a cube to represent origin (4, 0, 4)
// draw_cube(vec3(4., 0., 4.), vec3(0.2, 0.2, 0.2), None, BLACK);
//
// // draw green cube at (6, 0, 4)
// draw_cube(vec3(6., 0., 4.), vec3(0.2, 0.2, 0.2), None, GREEN);
//
// // draw purple at rotatet (3.65, 0, 5.9)
// draw_cube(vec3(3.65, 0., 5.9), vec3(0.2, 0.2, 0.2), None, PURPLE);
//
// let pivot = vec3(4., 0., 4.);
// let rotation = Quat::from_rotation_y(angle);
//
// draw_line_3d(pivot, pivot + rotation * vec3(2.0, 0.0, 0.0), RED);
// draw_line_3d(pivot, pivot + rotation * vec3(0.0, 2.0, 0.0), GREEN);
// draw_line_3d(pivot, pivot + rotation * vec3(0.0, 0.0, 2.0), BLUE);
//
// // draw a cube +4 on the z axis
// let cube_offset = vec3(0., 0., 4.);
// let rotated_cube_pos = pivot + rotation * cube_offset;
//
// draw_cube(rotated_cube_pos, vec3(0.2, 0.2, 0.2), None, YELLOW);
