use raylib::prelude::*;
use crate::table::*;

pub const BORDER_THIN:f32 = 1_f32;
pub const BORDER_THICK:f32 = 2_f32;

pub const THICK_LR_SEGMENT:f32 = 10_f32;
pub const THIN_LR_SEGMENT:f32 = 16_f32;

pub const BOTTOM_DRAW_Y:f32 = B_Y-BORDER_THICK;
pub const RIGHT_THICK_DRAW_X:f32 = TABLE_WIDTH - BORDER_THICK;
pub const LR_THIN_Y:f32 = THICK_LR_SEGMENT+B_Y;
pub const RIGHT_THIN_DRAW_X:f32 = TABLE_WIDTH-BORDER_THIN;

pub const TB_BORDER_REC:Rectangle = Rectangle{x:O,y:B_Y,width:TABLE_WIDTH,height: BORDER_THICK};
pub const TOP_BORDER_REC2:Rectangle = Rectangle{x:O,y:O,width:TABLE_WIDTH*TABLE_SCALE,height: BORDER_THICK*TABLE_SCALE};
pub const BOTTOM_BORDER_REC2:Rectangle = Rectangle{x:O,y:BOTTOM_DRAW_Y*TABLE_SCALE,width:TABLE_WIDTH*TABLE_SCALE,height: BORDER_THICK*TABLE_SCALE};

pub const LR_BORDER_THICK1_REC:Rectangle = Rectangle{x:O,y:B_Y,width:BORDER_THICK,height: THICK_LR_SEGMENT};
pub const L_BORDER_THICK1_REC2:Rectangle = Rectangle{x:O,y:O,width:BORDER_THICK*TABLE_SCALE,height: THICK_LR_SEGMENT*TABLE_SCALE};
pub const L_BORDER_THICK2_REC2:Rectangle = Rectangle{x:O,y:(THICK_LR_SEGMENT+THIN_LR_SEGMENT)*TABLE_SCALE,width:BORDER_THICK*TABLE_SCALE,height: THICK_LR_SEGMENT*TABLE_SCALE};
pub const R_BORDER_THICK1_REC2:Rectangle = Rectangle{x:RIGHT_THICK_DRAW_X*TABLE_SCALE,y:O,width:BORDER_THICK*TABLE_SCALE,height: THICK_LR_SEGMENT*TABLE_SCALE};
pub const R_BORDER_THICK2_REC2:Rectangle = Rectangle{x:RIGHT_THICK_DRAW_X*TABLE_SCALE,y:(THICK_LR_SEGMENT+THIN_LR_SEGMENT)*TABLE_SCALE,width:BORDER_THICK*TABLE_SCALE,height: THICK_LR_SEGMENT*TABLE_SCALE};

pub const LR_BORDER_THIN_REC:Rectangle = Rectangle{x:O,y:LR_THIN_Y,width:BORDER_THIN,height: THIN_LR_SEGMENT};
pub const L_BORDER_THIN_REC2:Rectangle = Rectangle{x:O,y:THICK_LR_SEGMENT*TABLE_SCALE,width:BORDER_THIN*TABLE_SCALE,height: THIN_LR_SEGMENT*TABLE_SCALE};
pub const R_BORDER_THIN_REC2:Rectangle = Rectangle{x:RIGHT_THIN_DRAW_X*TABLE_SCALE,y:THICK_LR_SEGMENT*TABLE_SCALE,width:BORDER_THIN*TABLE_SCALE,height: THIN_LR_SEGMENT*TABLE_SCALE};

pub fn draw_borders(d:&mut RaylibDrawHandle,sheets:&Texture2D){
   d.draw_texture_pro(sheets, TB_BORDER_REC, TOP_BORDER_REC2, Vector2::default(), O, Color::WHITE);
   d.draw_texture_pro(sheets, TB_BORDER_REC, BOTTOM_BORDER_REC2, Vector2::default(), O, Color::WHITE);
   d.draw_texture_pro(sheets,LR_BORDER_THICK1_REC,  L_BORDER_THICK1_REC2, Vector2::default(), O, Color::WHITE);
   d.draw_texture_pro(sheets,LR_BORDER_THICK1_REC,  L_BORDER_THICK2_REC2, Vector2::default(), O, Color::WHITE);
   d.draw_texture_pro(sheets,LR_BORDER_THICK1_REC,  R_BORDER_THICK1_REC2, Vector2::default(), O, Color::WHITE);
   d.draw_texture_pro(sheets,LR_BORDER_THICK1_REC,  R_BORDER_THICK2_REC2, Vector2::default(), O, Color::WHITE);
   d.draw_texture_pro(sheets,LR_BORDER_THIN_REC,  L_BORDER_THIN_REC2, Vector2::default(), O, Color::WHITE);
   d.draw_texture_pro(sheets,LR_BORDER_THIN_REC,  R_BORDER_THIN_REC2, Vector2::default(), O, Color::WHITE);
}