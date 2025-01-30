use egui::{load::SizedTexture, vec2, Color32, ColorImage, Image, Rect, Response, Ui};
use nalgebra::Vector2;

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
        if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary) && img_rect.contains(i.pointer.interact_pos().unwrap())) {
            let mouse_pos = (ctx.pointer_interact_pos().unwrap() - img_rect.left_top()) / vec2(img_rect.width(), img_rect.height());
            // println!("{}", mouse_pos);
            let pixel_pos = Vector2::<usize>::new((mouse_pos.x * 512.0) as usize, (mouse_pos.y * 512.0) as usize);

            self.add_radius(pixel_pos, 12);
        }
    }


    pub fn new() -> Self {
        Self {
            texture: egui::ColorImage::new([512, 512], Color32::GREEN)
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

                self.texture.pixels[y * 512 + x] = Color32::RED;
            }
        }
    }
}