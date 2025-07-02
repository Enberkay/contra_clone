use macroquad::prelude::*;

struct Player {
    pos: Vec2,
    bullets: Vec<Vec2>,
    alive: bool,
}

struct Enemy {
    pos: Vec2,
}

impl Player {
    fn new() -> Self {
        Self {
            pos: vec2(100.0, 300.0),
            bullets: vec![],
            alive: true,
        }
    }

    fn update(&mut self) {
        if !self.alive {
            return;
        }

        let speed = 3.0;
        if is_key_down(KeyCode::A) { self.pos.x -= speed; }
        if is_key_down(KeyCode::D) { self.pos.x += speed; }
        if is_key_down(KeyCode::W) { self.pos.y -= speed; }
        if is_key_down(KeyCode::S) { self.pos.y += speed; }

        if is_key_pressed(KeyCode::Space) {
            self.bullets.push(vec2(self.pos.x + 20.0, self.pos.y));
        }

        for bullet in &mut self.bullets {
            bullet.x += 5.0;
        }
        self.bullets.retain(|b| b.x < screen_width());
    }

    fn draw(&self) {
        if self.alive {
            draw_rectangle(self.pos.x, self.pos.y, 30.0, 30.0, BLUE);
        }
        for bullet in &self.bullets {
            draw_rectangle(bullet.x, bullet.y + 10.0, 10.0, 5.0, RED);
        }
    }

    fn hitbox(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, 30.0, 30.0)
    }
}

impl Enemy {
    fn new(x: f32, y: f32) -> Self {
        Self { pos: vec2(x, y) }
    }

    fn update(&mut self) { self.pos.x -= 2.0; }

    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, 30.0, 30.0, DARKGRAY);
    }

    fn hitbox(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, 30.0, 30.0)
    }
}

#[macroquad::main("Contra Clone v0.2.0")]
async fn main() {
    let mut player = Player::new();
    let mut enemies: Vec<Enemy> = vec![];
    let mut spawn_timer = 0.0;

    // ⭐ เพิ่มตัวแปรคะแนน
    let mut score: u32 = 0;

    loop {
        clear_background(WHITE);

        // สุ่มเกิดศัตรูทุก ~1.5 วินาที (ถ้ายังไม่ Game Over)
        if player.alive {
            spawn_timer += get_frame_time();
            if spawn_timer > 1.5 {
                spawn_timer = 0.0;
                let y = rand::gen_range(0.0, screen_height() - 30.0);
                enemies.push(Enemy::new(screen_width(), y));
            }
        }

        // --- Update ---
        player.update();
        for enemy in &mut enemies { enemy.update(); }

        // ตรวจชนกระสุนกับศัตรู → ทำลายศัตรู + เพิ่มคะแนน
        enemies.retain(|enemy| {
            for bullet in &player.bullets {
                let bullet_rect = Rect::new(bullet.x, bullet.y + 10.0, 10.0, 5.0);
                if bullet_rect.overlaps(&enemy.hitbox()) {
                    score += 10;           // +10 แต้มต่อ 1 ศัตรู
                    return false;          // ลบทิ้ง
                }
            }
            true
        });

        // ตรวจชนผู้เล่น
        if player.alive {
            for enemy in &enemies {
                if enemy.hitbox().overlaps(&player.hitbox()) {
                    player.alive = false;
                    break;
                }
            }
        }

        // --- Draw ---
        player.draw();
        for enemy in &enemies { enemy.draw(); }

        // ⭐ แสดงคะแนนตลอดเกม
        draw_text(&format!("Score: {}", score), 10.0, 30.0, 30.0, BLACK);

        // GAME OVER
        if !player.alive {
            let msg = "GAME OVER";
            let w = measure_text(msg, None, 40, 1.0).width;
            draw_text(
                msg,
                screen_width()/2.0 - w/2.0,
                screen_height()/2.0,
                40.0,
                RED,
            );
            // ⭐ คะแนนสุดท้ายใต้ GAME OVER
            let final_msg = format!("Final Score: {}", score);
            let w2 = measure_text(&final_msg, None, 30, 1.0).width;
            draw_text(
                &final_msg,
                screen_width()/2.0 - w2/2.0,
                screen_height()/2.0 + 40.0,
                30.0,
                DARKGRAY,
            );
        }

        next_frame().await
    }
}
