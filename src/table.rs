use raylib::prelude::*;

pub const O:f32 = 0_f32;
pub const TABLE_WIDTH:f32 = 71_f32;
pub const TABLE_HEIGHT:f32 = 36_f32;
pub const TABLE_SCALE:f32 = 16_f32;
pub const B_Y:f32 = TABLE_HEIGHT;

pub struct Table {
       rec:Rectangle,
       rec2:Rectangle,
}

impl Table{
   pub fn new() -> Self{
    let rec = Rectangle::new(O,O,TABLE_WIDTH,TABLE_HEIGHT);
    let rec2 = Rectangle::new(O,O,TABLE_WIDTH*TABLE_SCALE,TABLE_HEIGHT*TABLE_SCALE);
    Self{rec,rec2}
    }

   pub fn draw_ground(&self,d:&mut RaylibDrawHandle,sheets:&Texture2D){
     d.draw_texture_pro(sheets, self.rec, self.rec2, Vector2::default(), O, Color::WHITE)
    }
}