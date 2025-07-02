use macroquad::prelude::*;


#[allow(dead_code)]
struct Bullet {
    pos: Vec2,
    from_enemy: bool,
}

struct Game {
    player_pos: Vec2,
    player_bullets: Vec<Bullet>,
    enemy_pos: Vec<Vec2>,
    enemy_bullets: Vec<Bullet>,
    player_alive: bool,
    score: u32,

    // Sprites
    player_tex: Texture2D,
    enemy_tex: Texture2D,
}

impl Game {
    async fn new() -> Self {
        let player_tex = load_texture("assets/player.png").await.unwrap();
        let enemy_tex = load_texture("assets/enemy.png").await.unwrap();
        player_tex.set_filter(FilterMode::Nearest);
        enemy_tex.set_filter(FilterMode::Nearest);

        Self {
            player_pos: vec2(100.0, 300.0),
            player_bullets: vec![],
            enemy_pos: vec![vec2(600.0, 200.0)],
            enemy_bullets: vec![],
            player_alive: true,
            score: 0,

            player_tex,
            enemy_tex,
        }
    }

    fn update(&mut self) {
        if !self.player_alive {
            return;
        }

        let speed = 3.0;
        if is_key_down(KeyCode::A) {
            self.player_pos.x -= speed;
        }
        if is_key_down(KeyCode::D) {
            self.player_pos.x += speed;
        }
        if is_key_down(KeyCode::W) {
            self.player_pos.y -= speed;
        }
        if is_key_down(KeyCode::S) {
            self.player_pos.y += speed;
        }

        if is_key_pressed(KeyCode::Space) {
            self.player_bullets.push(Bullet {
                pos: self.player_pos + vec2(20.0, 8.0),
                from_enemy: false,
            });
        }

        for bullet in &mut self.player_bullets {
            bullet.pos.x += 5.0;
        }
        self.player_bullets.retain(|b| b.pos.x < screen_width());

        // Enemy movement
        for e in &mut self.enemy_pos {
            e.x -= 1.0;
        }

        // Enemy shooting
        for e in &self.enemy_pos {
            if rand::gen_range(0.0, 1.0) < 0.01 {
                self.enemy_bullets.push(Bullet {
                    pos: *e + vec2(-5.0, 8.0),
                    from_enemy: true,
                });
            }
        }

        for bullet in &mut self.enemy_bullets {
            bullet.pos.x -= 4.0;
        }

        // Bullet hits enemy
        self.enemy_pos.retain(|enemy| {
            for b in &self.player_bullets {
                if b.pos.distance(*enemy) < 16.0 {
                    self.score += 10;
                    return false;
                }
            }
            true
        });

        // Bullet hits player
        for b in &self.enemy_bullets {
            if b.pos.distance(self.player_pos) < 16.0 {
                self.player_alive = false;
            }
        }
    }

    fn draw(&self) {
        clear_background(WHITE);

        if self.player_alive {
            draw_texture(&self.player_tex, self.player_pos.x, self.player_pos.y, WHITE);
        }

        for e in &self.enemy_pos {
            draw_texture(&self.enemy_tex, e.x, e.y, WHITE);
        }

        // Bullets: still use rectangles
        for b in &self.player_bullets {
            draw_rectangle(b.pos.x, b.pos.y, 10.0, 5.0, RED);
        }

        for b in &self.enemy_bullets {
            draw_rectangle(b.pos.x, b.pos.y, 10.0, 5.0, BLACK);
        }

        draw_text(&format!("Score: {}", self.score), 10.0, 30.0, 30.0, BLACK);

        if !self.player_alive {
            draw_text(
                "GAME OVER",
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0,
                40.0,
                RED,
            );
        }
    }
}

#[macroquad::main("Contra Clone v0.4.1")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
