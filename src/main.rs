use macroquad::prelude::*;

struct Player {
    pos: Vec2,
    bullets: Vec<Vec2>,
}

impl Player {
    fn new() -> Self {
        Self {
            pos: vec2(100.0, 300.0),
            bullets: vec![],
        }
    }

    fn update(&mut self) {
        let speed = 3.0;
        if is_key_down(KeyCode::A) {
            self.pos.x -= speed;
        }
        if is_key_down(KeyCode::D) {
            self.pos.x += speed;
        }
        if is_key_down(KeyCode::W) {
            self.pos.y -= speed;
        }
        if is_key_down(KeyCode::S) {
            self.pos.y += speed;
        }

        // ยิงกระสุน
        if is_key_pressed(KeyCode::Space) {
            self.bullets.push(vec2(self.pos.x + 20.0, self.pos.y));
        }

        // อัปเดตกระสุน
        for bullet in &mut self.bullets {
            bullet.x += 5.0;
        }

        // ลบกระสุนที่ออกนอกจอ
        self.bullets.retain(|b| b.x < screen_width());
    }

    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, 30.0, 30.0, BLUE);

        for bullet in &self.bullets {
            draw_rectangle(bullet.x, bullet.y + 10.0, 10.0, 5.0, RED);
        }
    }
}

#[macroquad::main("Contra Clone")]
async fn main() {
    let mut player = Player::new();

    loop {
        clear_background(WHITE);

        player.update();
        player.draw();

        next_frame().await
    }
}
