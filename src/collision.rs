use macroquad::prelude::*;

pub fn aabb(a: Rect, b: Rect) -> bool {
    // Teste de sobreposição de AABB.
    a.x < b.x + b.w &&
    a.x + a.w > b.x &&
    a.y < b.y + b.h &&
    a.y + a.h > b.y
}
