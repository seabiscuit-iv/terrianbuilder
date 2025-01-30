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
    
            self.texture.pixels[pixel_pos.y * 512 + pixel_pos.x] = Color32::RED;
        }
    }


    pub fn new() -> Self {
        Self {
            texture: egui::ColorImage::new([512, 512], Color32::GREEN)
        }
    }


    fn add_radius(&mut self, pos: Vector2<usize>, radius: usize) {
        let minx = 

        self.texture.pixels[pos.y * 512 + pos.x] = Color32::RED
    } 
}