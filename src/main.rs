use ggez::{
    self,
    graphics as g,
    event as e,
    nalgebra::{
        Point2 as p2,
        Vector2 as v2,
    },
    input::keyboard::{
        self as k,
        KeyCode as kc,
    },
    Context,
    GameResult,
};
use rand::{
    thread_rng as trng,
    Rng,
};

const PADDING: f32 = 40.0;
const MIDDLE_LINE_WIDTH: f32 = 2.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;

const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED: f32 = 600.0;

fn move_racket(pos: &mut p2<f32>, keycode: kc, dir: f32, ctx: &mut Context) {
    let dt = ggez::timer::delta(ctx).as_secs_f32();
    let screen_h = g::drawable_size(ctx).1;
    if k::is_key_pressed(ctx, keycode) {
        pos.y += PLAYER_SPEED * dt * dir;
    }
    clamp(&mut pos.y, RACKET_HEIGHT_HALF, screen_h - RACKET_HEIGHT_HALF);
}

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn randomize_vec(vec: &mut v2<f32>, x: f32, y: f32) {
    let mut rng = trng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

struct MainState {
    player_1_pos: p2<f32>,
    player_2_pos: p2<f32>,
    ball_pos: p2<f32>,
    ball_vel: v2<f32>,
    player_1_score: i32,
    player_2_score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = g::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        let mut ball_vel = v2::new(0.0, 0.0);
        randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_pos : p2::new(RACKET_WIDTH_HALF + PADDING, screen_h_half),
            player_2_pos : p2::new(screen_w - RACKET_WIDTH_HALF - PADDING, screen_h_half),
            ball_pos : p2::new(screen_w_half, screen_h_half),
            ball_vel,
            player_1_score : 0,
            player_2_score : 0,
        }
    }
}

impl e::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        move_racket(&mut self.player_1_pos, kc::W, -1.0, ctx);
        move_racket(&mut self.player_1_pos, kc::S, 1.0, ctx);
        move_racket(&mut self.player_2_pos, kc::I, -1.0, ctx);
        move_racket(&mut self.player_2_pos, kc::K, 1.0, ctx);

        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.ball_pos += self.ball_vel * dt;

        let (screen_w, screen_h) = g::drawable_size(ctx);

        if self.ball_pos.x < 0.0 {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1;
        } else if self.ball_pos.x > screen_w {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1;
        }

        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - BALL_SIZE_HALF;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }

        if self.ball_pos.x - BALL_SIZE_HALF < self.player_1_pos.x + RACKET_WIDTH_HALF
        && self.ball_pos.x + BALL_SIZE_HALF > self.player_1_pos.x - RACKET_WIDTH_HALF
        && self.ball_pos.y - BALL_SIZE_HALF < self.player_1_pos.y + RACKET_HEIGHT_HALF
        && self.ball_pos.y + BALL_SIZE_HALF > self.player_1_pos.y - RACKET_HEIGHT_HALF {
            self.ball_vel.x = self.ball_vel.x.abs();
        }

        if self.ball_pos.x - BALL_SIZE_HALF < self.player_2_pos.x + RACKET_WIDTH_HALF
        && self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - RACKET_WIDTH_HALF
        && self.ball_pos.y - BALL_SIZE_HALF < self.player_2_pos.y + RACKET_HEIGHT_HALF
        && self.ball_pos.y + BALL_SIZE_HALF > self.player_2_pos.y - RACKET_HEIGHT_HALF {
            self.ball_vel.x = -self.ball_vel.x.abs();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        g::clear(ctx, g::BLACK);

        let screen_w_half = g::drawable_size(ctx).0 * 0.5;

        let racket_rect = g::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = g::Mesh::new_rectangle(ctx, g::DrawMode::fill(), racket_rect, g::WHITE)?;

        let ball_rect = g::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = g::Mesh::new_rectangle(ctx, g::DrawMode::fill(), ball_rect, g::WHITE)?;

        let screen_h = g::drawable_size(ctx).1;
        let middle_rect = g::Rect::new(-MIDDLE_LINE_WIDTH*0.5, 0.0, MIDDLE_LINE_WIDTH, screen_h);
        let middle_mesh = g::Mesh::new_rectangle(ctx, g::DrawMode::fill(), middle_rect, g::WHITE)?;

        let mut draw_param = g::DrawParam::default();

        draw_param.dest = self.player_1_pos.into();
        g::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.player_2_pos.into();
        g::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.ball_pos.into();
        g::draw(ctx, &ball_mesh, draw_param)?;

        let screen_middle_x = g::drawable_size(ctx).0 * 0.5;
        draw_param.dest = [screen_middle_x, 0.0].into();
        g::draw(ctx, &middle_mesh, draw_param)?;

        let score_text = g::Text::new(format!("{}          {}", self.player_1_score, self.player_2_score));
        draw_param.dest = p2::new(screen_w_half - (score_text.dimensions(ctx).0 as f32 * 0.5), 16.0).into();
        g::draw(ctx, &score_text, draw_param)?;

        g::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult{
    let cb = ggez::ContextBuilder::new("Pong","Marius");
    let (ctx, event_loop) = &mut cb.build()?;
    g::set_window_title(ctx, "Pong");

    let mut state = MainState::new(ctx);
    e::run(ctx, event_loop, &mut state)?;
    Ok(())
}
