use macroquad::prelude::*;

struct Player {
    pos: Vec2,
    bullets: Vec<Vec2>,
    alive: bool,
}

struct Enemy {
    pos: Vec2,
    bullet_timer: f32,
}

struct EnemyBullet {
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
        Self {
            pos: vec2(x, y),
            bullet_timer: 0.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.pos.x -= 2.0;
        self.bullet_timer += dt;
    }

    fn can_shoot(&self) -> bool {
        self.bullet_timer > 2.0
    }

    fn shoot(&mut self) -> EnemyBullet {
        self.bullet_timer = 0.0;
        EnemyBullet {
            pos: vec2(self.pos.x - 10.0, self.pos.y + 10.0),
        }
    }

    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, 30.0, 30.0, DARKGRAY);
    }

    fn hitbox(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, 30.0, 30.0)
    }
}

impl EnemyBullet {
    fn update(&mut self) {
        self.pos.x -= 4.0;
    }

    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, 10.0, 5.0, BLACK);
    }

    fn hitbox(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, 10.0, 5.0)
    }
}

#[macroquad::main("Contra Clone v0.3.0")]
async fn main() {
    let mut player = Player::new();
    let mut enemies: Vec<Enemy> = vec![];
    let mut enemy_bullets: Vec<EnemyBullet> = vec![];
    let mut spawn_timer = 0.0;
    let mut score: u32 = 0;

    loop {
        clear_background(WHITE);
        let dt = get_frame_time();

        if player.alive {
            spawn_timer += dt;
            if spawn_timer > 1.5 {
                spawn_timer = 0.0;
                let y = rand::gen_range(0.0, screen_height() - 30.0);
                enemies.push(Enemy::new(screen_width(), y));
            }
        }

        // --- Update ---
        player.update();
        for enemy in &mut enemies {
            enemy.update(dt);
            if enemy.can_shoot() {
                enemy_bullets.push(enemy.shoot());
            }
        }
        for bullet in &mut enemy_bullets {
            bullet.update();
        }

        // ตรวจชนกระสุนผู้เล่นกับศัตรู
        enemies.retain(|enemy| {
            for bullet in &player.bullets {
                let bullet_rect = Rect::new(bullet.x, bullet.y + 10.0, 10.0, 5.0);
                if bullet_rect.overlaps(&enemy.hitbox()) {
                    score += 10;
                    return false;
                }
            }
            true
        });

        // ตรวจชนผู้เล่นกับศัตรูหรือกระสุนศัตรู
        if player.alive {
            for enemy in &enemies {
                if enemy.hitbox().overlaps(&player.hitbox()) {
                    player.alive = false;
                }
            }
            for bullet in &enemy_bullets {
                if bullet.hitbox().overlaps(&player.hitbox()) {
                    player.alive = false;
                }
            }
        }

        // --- Draw ---
        player.draw();
        for enemy in &enemies {
            enemy.draw();
        }
        for bullet in &enemy_bullets {
            bullet.draw();
        }

        draw_text(&format!("Score: {}", score), 10.0, 30.0, 30.0, BLACK);

        if !player.alive {
            let msg = "GAME OVER";
            let w = measure_text(msg, None, 40, 1.0).width;
            draw_text(
                msg,
                screen_width() / 2.0 - w / 2.0,
                screen_height() / 2.0,
                40.0,
                RED,
            );
            let final_msg = format!("Final Score: {}", score);
            let w2 = measure_text(&final_msg, None, 30, 1.0).width;
            draw_text(
                &final_msg,
                screen_width() / 2.0 - w2 / 2.0,
                screen_height() / 2.0 + 40.0,
                30.0,
                DARKGRAY,
            );
        }

        next_frame().await;
    }
}
