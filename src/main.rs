use macroquad::prelude::*;

#[allow(dead_code)]
struct Bullet {
    pos: Vec2,
    from_enemy: bool,
}

struct Game {
    player_pos: Vec2,
    velocity: Vec2,
    is_on_ground: bool,

    player_bullets: Vec<Bullet>,
    enemy_pos: Vec<Vec2>,
    enemy_bullets: Vec<Bullet>,
    player_alive: bool,
    score: u32,

    // Sprites
    player_tex: Texture2D,
    enemy_tex: Texture2D,

    // Backgrounds
    sky_tex: Texture2D,
    ground_tex: Texture2D,
    bg_scroll: f32,
}

impl Game {
    async fn new() -> Self {
        let player_tex = load_texture("assets/player.png").await.unwrap();
        let enemy_tex = load_texture("assets/enemy.png").await.unwrap();
        let sky_tex = load_texture("assets/sky.png").await.unwrap();
        let ground_tex = load_texture("assets/ground.png").await.unwrap();

        player_tex.set_filter(FilterMode::Nearest);
        enemy_tex.set_filter(FilterMode::Nearest);
        sky_tex.set_filter(FilterMode::Nearest);
        ground_tex.set_filter(FilterMode::Nearest);

        let ground_y = screen_height() - ground_tex.height();
        let player_y = ground_y - player_tex.height();
        let enemy_y = ground_y - enemy_tex.height();

        Self {
            player_pos: vec2(100.0, player_y),
            velocity: vec2(0.0, 0.0),
            is_on_ground: false,

            player_bullets: vec![],
            enemy_pos: vec![vec2(600.0, enemy_y)],
            enemy_bullets: vec![],
            player_alive: true,
            score: 0,

            player_tex,
            enemy_tex,

            sky_tex,
            ground_tex,
            bg_scroll: 0.0,
        }
    }

    fn update(&mut self) {
        let ground_y = screen_height() - self.ground_tex.height();

        const GRAVITY: f32 = 0.4;
        const JUMP_FORCE: f32 = -8.0;

        self.bg_scroll += 1.0;

        if !self.player_alive {
            return;
        }

        // Move left/right
        let speed = 3.0;
        if is_key_down(KeyCode::A) {
            self.player_pos.x -= speed;
        }
        if is_key_down(KeyCode::D) {
            self.player_pos.x += speed;
        }

        // Jump
        if self.is_on_ground && (is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Space)) {
            self.velocity.y = JUMP_FORCE;
            self.is_on_ground = false;
        }

        // Gravity
        self.velocity.y += GRAVITY;
        self.player_pos.y += self.velocity.y;

        // Ground collision
        if self.player_pos.y >= ground_y - self.player_tex.height() {
            self.player_pos.y = ground_y - self.player_tex.height();
            self.velocity.y = 0.0;
            self.is_on_ground = true;
        }

        // Shoot
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

        // Enemy Y alignment
        for e in &mut self.enemy_pos {
            e.y = ground_y - self.enemy_tex.height();
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
        clear_background(SKYBLUE);

        // Draw sky full screen
        draw_texture_ex(
            &self.sky_tex,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw ground with horizontal tiling
        let ground_y = screen_height() - self.ground_tex.height();
        let mut x = -self.bg_scroll % self.ground_tex.width();
        while x < screen_width() {
            draw_texture(&self.ground_tex, x, ground_y, WHITE);
            x += self.ground_tex.width();
        }

        // Player
        if self.player_alive {
            draw_texture(
                &self.player_tex,
                self.player_pos.x,
                self.player_pos.y,
                WHITE,
            );
        }

        // Enemies
        for e in &self.enemy_pos {
            draw_texture(&self.enemy_tex, e.x, e.y, WHITE);
        }

        // Bullets
        for b in &self.player_bullets {
            draw_rectangle(b.pos.x, b.pos.y, 10.0, 5.0, RED);
        }

        for b in &self.enemy_bullets {
            draw_rectangle(b.pos.x, b.pos.y, 10.0, 5.0, BLACK);
        }

        // Score
        draw_text(&format!("Score: {}", self.score), 10.0, 30.0, 30.0, BLACK);

        // Game over
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

const RESOLUTIONS: &[(u32, u32)] = &[
    (640, 360),
    (800, 600),
    (1024, 768),
    (1280, 720),
    (1920, 1080),
];

#[macroquad::main("Contra Clone v0.6.0")]
async fn main() {
    let mut selected = 0;
    loop {
        clear_background(BLACK);
        draw_text("Select resolution:", 100.0, 80.0, 40.0, WHITE);

        for (i, (w, h)) in RESOLUTIONS.iter().enumerate() {
            let color = if i == selected { YELLOW } else { GRAY };
            draw_text(
                &format!("{}x{}", w, h),
                120.0,
                140.0 + i as f32 * 40.0,
                30.0,
                color,
            );
        }

        if is_key_pressed(KeyCode::Down) {
            selected = (selected + 1) % RESOLUTIONS.len();
        }
        if is_key_pressed(KeyCode::Up) {
            selected = (selected + RESOLUTIONS.len() - 1) % RESOLUTIONS.len();
        }
        if is_key_pressed(KeyCode::Enter) {
            let (w, h) = RESOLUTIONS[selected];
            request_new_screen_size(w as f32, h as f32);
            break;
        }

        next_frame().await;
    }

    let mut game = Game::new().await;
    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
