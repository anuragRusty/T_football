use raylib::prelude::*;
use crate::table::*;

const PLAYER_BLUE_VEC2:Vector2 = Vector2::new(12_f32,72_f32);
const PLAYER_RED_VEC2:Vector2 = Vector2::new(12_f32,88_f32);
const PLAYER_FRAMES:[f32;9] = [12_f32,48_f32,84_f32,120_f32,156_f32,192_f32,228_f32,264_f32,300_f32];
const PLAYER_HEIGHT:f32 = 16_f32;
const PLAYER_WIDTH:f32 = 24_f32;
const ANIM_DURATION:f32 = 0.04;
const PLAYER_SCALE:f32 = TABLE_SCALE/5_f32;

const BLUE_PLAYER:Rectangle = Rectangle::new(PLAYER_BLUE_VEC2.x,PLAYER_BLUE_VEC2.y,PLAYER_WIDTH,PLAYER_HEIGHT);
const RED_PLAYER:Rectangle = Rectangle::new(PLAYER_RED_VEC2.x,PLAYER_RED_VEC2.y,PLAYER_WIDTH,PLAYER_HEIGHT);
const CENTRE:Vector2 = Vector2::new(PLAYER_WIDTH*PLAYER_SCALE/2_f32,PLAYER_HEIGHT*PLAYER_SCALE/2_f32);

#[derive(PartialEq,Clone,Debug,Copy)]
pub enum TeamType {
   RedTeam,
   BlueTeam,
}

#[derive(PartialEq,Clone,Debug,Copy)]
pub struct Player {
    rec:Rectangle,
    rec2:Rectangle,
    deg:f32,
    frames:usize,
    time:f32,
    team:TeamType,
}

impl Player{
    pub fn new(x:f32,y:f32,team:TeamType,deg:f32) -> Self {
      let rec = if team == TeamType::BlueTeam{BLUE_PLAYER} else {RED_PLAYER};
      let rec2 = Rectangle::new(x,y,PLAYER_WIDTH*PLAYER_SCALE,PLAYER_HEIGHT*PLAYER_SCALE);
      let frames = 0;
      let time:f32 = O;
      let team = TeamType::BlueTeam;
      Self{rec,rec2,deg,frames,time,team}
    }

    pub fn update(&mut self,frame_time:&f32){
       self.animate(frame_time);
    }

    pub fn animate(&mut self,frame_time:&f32){
      if self.time > ANIM_DURATION {
         self.time = O;
        // self.frames += 1;
      }
      self.time += *frame_time;
      self.frames = self.frames % PLAYER_FRAMES.len();
      self.rec.x = PLAYER_FRAMES[self.frames];        
    }

    pub fn draw(&mut self,d:&mut RaylibDrawHandle,sheets:&Texture2D){
        d.draw_texture_pro(sheets, self.rec, self.rec2, CENTRE,self.deg, Color::WHITE);
    }

}