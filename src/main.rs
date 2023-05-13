mod ball;
mod table;
mod borders;
mod physics;
mod player;
mod team;

use raylib::prelude::*;
use crate::table::*;
use crate::ball::*;
use crate::borders::*;

use crate::player::*;

const S_WIDTH:i32 = TABLE_WIDTH as i32;
const S_HEIGHT:i32 = TABLE_HEIGHT as i32;
const S_SCALE:i32 = TABLE_SCALE as i32;

const SHEETS:&str = "assets/sheets3.png";
const BG_COLOR:Color = Color::new(64,60,60,1);

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(S_WIDTH*S_SCALE, S_HEIGHT*S_SCALE)
        .title("T-FOOTBALL")
        .build();

    let sheets = rl.load_texture(&thread, SHEETS).unwrap();

    let mut tb = Table::new();
    let mut ball = Ball::new();

    let mut pl = Player::new(7_f32*TABLE_SCALE,TABLE_HEIGHT/2_f32*TABLE_SCALE,TeamType::RedTeam,180_f32);
    let mut pl2 = Player::new((TABLE_WIDTH-9_f32)*TABLE_SCALE,TABLE_HEIGHT/2_f32*TABLE_SCALE,TeamType::BlueTeam,180_f32);

   //CheckCollisionCircleRec(center, radius, rec)

    while !rl.window_should_close() {
        let frame_time = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG_COLOR);
        ball.update(&frame_time);
        tb.draw_ground(&mut d, &sheets);
        ball.draw(&mut d, &sheets);
        draw_borders(&mut d, &sheets);

        pl.draw(&mut d, &sheets);
        pl.update(&frame_time);


        pl2.draw(&mut d, &sheets);
        pl2.update(&frame_time);
    }
}