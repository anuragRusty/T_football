use raylib::prelude::*;
use crate::table::*;
use crate::borders::*;
use crate::physics::check_collision_circle_rect;

const BALL_RADIUS:f32 = 32_f32;
const BALL_DIAMETER:f32 = BALL_RADIUS*2_f32;
const BALL_Y:f32 = 104_f32;
const BALL_SCALE:f32 = TABLE_SCALE/40_f32;
const BALL_SPEED:f32 = 220_f32;
const BALL_FRAMES:[f32;6] = [0_f32,64_f32,128_f32,192_f32,256_f32,320_f32];
const ANIM_DURATION:f32 = 0.05;

const MID_MAP:Vector2 = Vector2::new(TABLE_WIDTH*TABLE_SCALE/2_f32, TABLE_HEIGHT*TABLE_SCALE/2_f32);
pub const CENTRE:Vector2 = Vector2::new(BALL_RADIUS*BALL_SCALE,BALL_RADIUS*BALL_SCALE);

const LEFT_BODERS:[Rectangle;3] = [L_BORDER_THICK1_REC2,L_BORDER_THICK2_REC2,L_BORDER_THIN_REC2];
const RIGHT_BODERS:[Rectangle;3] = [R_BORDER_THICK1_REC2,R_BORDER_THICK2_REC2,R_BORDER_THIN_REC2];

#[derive(Clone, Copy,PartialEq)]
pub enum DIR {
    Left,
    Right,
    Top,
    Down,
    TopLeft,
    TopRight,
    DownLeft,
    DownRight,
}



impl DIR {
   fn get_val(&self) -> [f32;2]{
     match *self{
        DIR::Left => {[-BALL_SPEED,O]}
        DIR::Right => {[BALL_SPEED,O]}
        DIR::Top => {[O,-BALL_SPEED]}
        DIR::Down => {[O,BALL_SPEED]}
        DIR::TopLeft => {[-BALL_SPEED,-BALL_SPEED]}
        DIR::TopRight => {[BALL_SPEED,-BALL_SPEED]}
        DIR::DownLeft => {[-BALL_SPEED,BALL_SPEED]}
        DIR::DownRight => {[BALL_SPEED,BALL_SPEED]}
     }
   }

   fn get_rot(&self) -> f32 {
    match *self {
        DIR::Left => 180.0,
        DIR::Right => 0.0,
        DIR::Top => 270.0,
        DIR::Down => 90.0,
        DIR::TopLeft => 225.0,
        DIR::TopRight => 315.0,
        DIR::DownLeft => 135.0,
        DIR::DownRight => 45.0,
    }
}
}

  pub struct Ball {
      dir:DIR,
      center:Vector2,
      time:f32,
      frames:usize,
      rec:Rectangle,
      rec2:Rectangle,
  }

impl Ball {
   pub fn new() -> Self {
      let dir = DIR::DownLeft;
      let center = Vector2::new(MID_MAP.x,MID_MAP.y);
      let time = O;
      let frames = 0;
      let rec = Rectangle::new(O,BALL_Y,BALL_DIAMETER,BALL_DIAMETER);
      let rec2 = Rectangle::new(MID_MAP.x,MID_MAP.y,BALL_DIAMETER*BALL_SCALE,BALL_DIAMETER*BALL_SCALE);
      Self{dir,center,time,frames,rec,rec2}
    }

   pub fn draw(&mut self,d:&mut RaylibDrawHandle,sheets:&Texture2D){
       d.draw_texture_pro(sheets, self.rec, self.rec2, CENTRE, self.dir.get_rot(), Color::WHITE);
    }

   pub fn go(&mut self,frame_time:&f32){
       let dir_arr = self.dir.get_val();
       self.rec2.x += dir_arr[0] * *frame_time;
       self.rec2.y += dir_arr[1] * *frame_time;
       self.center.x += dir_arr[0] * *frame_time;
       self.center.y += dir_arr[1] * *frame_time;
    } 

    pub fn bounce2(&mut self){
       for (l,r) in LEFT_BODERS.iter().zip(RIGHT_BODERS){
        if check_collision_circle_rect(self.center, BALL_RADIUS*BALL_SCALE, *l){
          self.dir = match self.dir {
            DIR::Left => DIR::Right,
            DIR::DownLeft => DIR::DownRight,
            DIR::TopLeft => DIR::TopRight,
             _ => self.dir
           }
        }else if check_collision_circle_rect(self.center, BALL_RADIUS*BALL_SCALE, r){
          self.dir = match self.dir {
            DIR::Right => DIR::Left,
            DIR::DownRight => DIR::DownLeft,
            DIR::TopRight => DIR::TopLeft,
             _ => self.dir
           }
       } 

       if check_collision_circle_rect(self.center, BALL_RADIUS*BALL_SCALE, TOP_BORDER_REC2){
        self.dir = match self.dir {
          DIR::TopLeft => DIR::DownLeft,
          DIR::TopRight => DIR::DownRight,
          DIR::Top => DIR::Down,
           _ => self.dir
         }
        }else if check_collision_circle_rect(self.center, BALL_RADIUS*BALL_SCALE, BOTTOM_BORDER_REC2){
            self.dir = match self.dir {
              DIR::DownRight => DIR::TopRight,
              DIR::DownLeft => DIR::TopLeft,
              DIR::Down => DIR::Top,
               _ => self.dir
             }
          }
      }

    }

    pub fn update(&mut self,frame_time:&f32){
      self.bounce2();
      self.go(frame_time);
      self.anim(frame_time);
    }

    pub fn anim(&mut self,frame_time:&f32){
      if self.time >= ANIM_DURATION {
        self.time = O;
        self.frames += 1;
      }
      self.time += *frame_time;
      self.frames = self.frames % BALL_FRAMES.len();
      self.rec.x = BALL_FRAMES[self.frames];
    }

    pub fn reset(){
       
    }
}