use raylib::prelude::*;

pub fn limit_range(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn check_collision_circle_rect(circle_center: Vector2, circle_radius: f32, rect: Rectangle) -> bool {
    let closest_x = limit_range(circle_center.x, rect.x, rect.x + rect.width);
    let closest_y = limit_range(circle_center.y, rect.y, rect.y + rect.height);
    let distance_x = closest_x - circle_center.x;
    let distance_y = closest_y - circle_center.y;
    let distance_squared = distance_x * distance_x + distance_y * distance_y;
    return distance_squared < (circle_radius * circle_radius);
}