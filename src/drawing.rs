use core::prelude::v1;

use egui::{load::SizedTexture, vec2, Color32, ColorImage, Image, Rect, Response, Ui};
use nalgebra::{Vector2, Vector4};

pub struct Drawing {
    texture: ColorImage
}



impl Drawing {
    pub fn draw(&self, ui: &mut Ui, ctx: &egui::Context) -> Response {
        let img = &self.texture;

        let tex = ctx.load_texture("Image", img.clone(), egui::TextureOptions::default());
        let x : SizedTexture = (&tex).into();
        let img = Image::from_texture(x);
        ui.add(img)
    }


    pub fn draw_update(&mut self, ctx: &egui::Context, img_rect: Rect) {
        if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary) && (i.pointer.delta().length() > 0.1 || i.pointer.press_start_time() == Some(0.0)) && img_rect.contains(i.pointer.interact_pos().unwrap())) {
            let mouse_pos = (ctx.pointer_interact_pos().unwrap() - img_rect.left_top()) / vec2(img_rect.width(), img_rect.height());
            // println!("{}", mouse_pos);
            let pixel_pos = Vector2::<usize>::new((mouse_pos.x * 512.0) as usize, (mouse_pos.y * 512.0) as usize);

            self.add_radius(pixel_pos, 12);
        }
    }


    pub fn new() -> Self {
        Self {
            texture: egui::ColorImage::new([512, 512], Color32::BLACK)
        }
    }


    fn add_box(&mut self, pos: Vector2<usize>, radius: usize) {
        let min_x = ((pos.x as i32) - (radius as i32)).max(0) as usize;
        let min_y = ((pos.y as i32) - (radius as i32)).max(0) as usize;
        let max_x = ((pos.x as i32) + (radius as i32)).min(511) as usize;
        let max_y = ((pos.y as i32) + (radius as i32)).min(511) as usize;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                self.texture.pixels[y * 512 + x] = Color32::RED;
            }
        }
    } 

    fn add_radius(&mut self, pos: Vector2<usize>, radius: usize) {
        let min_x = ((pos.x as i32) - (radius as i32)).max(0) as usize;
        let min_y = ((pos.y as i32) - (radius as i32)).max(0) as usize;
        let max_x = ((pos.x as i32) + (radius as i32)).min(511) as usize;
        let max_y = ((pos.y as i32) + (radius as i32)).min(511) as usize;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let dx = ((x as f32) - (pos.x as f32)).abs();
                let dy = ((y as f32) - (pos.y as f32)).abs();

                if (dx.powf(2.0) + dy.powf(2.0)).sqrt() > (radius as f32) {
                    continue;
                }
                let mut v = col_to_vec4(self.texture.pixels[y * 512 + x]);

                v.x += 0.1;
                v.x = v.x.clamp(0.0, 1.0);
                
                v.y += 0.1;
                v.y = v.y.clamp(0.0, 1.0);
                
                v.z += 0.1;
                v.z = v.z.clamp(0.0, 1.0);

                self.texture.pixels[y * 512 + x] = vec4_to_col(v);
            }
        }
    }
}


fn col_to_vec4(col: Color32) -> Vector4<f32> {
    let col : Vector4<u8> = [col.r(), col.g(), col.b(), col.a()].into();
    let col :  Vector4<f32> = col.map(|x| (x as f32) / 255.0);
    col
}

fn vec4_to_col(vec: Vector4<f32>) -> Color32 {
    vec.iter().for_each(|f| {
        if *f > 1.0 || *f < 0.0 {
            panic!("Illegal Color Supplied");
        }
    });
    let col = vec.map(|x| (x * 255.0) as u8);
    return Color32::from_rgba_unmultiplied(col.x, col.y, col.z, col.w);
}