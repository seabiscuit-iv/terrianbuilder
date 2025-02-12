use core::{panic, prelude::v1};

use egui::{load::SizedTexture, pos2, vec2, Color32, ColorImage, Image, Rect, Response, Ui};
use nalgebra::{Vector2, Vector3, Vector4, U32};

pub struct Drawing {
    pub texture: ColorImage
}



impl Drawing {
    pub fn draw(&self, ui: &mut Ui, ctx: &egui::Context) -> Response {
        let img = &self.texture;

        let tex = ctx.load_texture("Image", img.clone(), egui::TextureOptions::default());
        let x : SizedTexture = (&tex).into();
        let img = Image::from_texture(x);
        ui.add(img)
    }

    pub fn get_image(&self) -> ColorImage {
        return self.texture.clone();
    }


    pub fn draw_update(&mut self, ctx: &egui::Context, img_rect: Rect) {
        if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary) && (i.pointer.delta().length() > 0.1 || i.pointer.press_start_time() == Some(0.0)) && img_rect.contains(i.pointer.interact_pos().unwrap())) {
            let mouse_pos = (ctx.pointer_interact_pos().unwrap() - img_rect.left_top()) / vec2(img_rect.width(), img_rect.height());
            // println!("{}", mouse_pos);
            let pixel_pos = Vector2::<usize>::new((mouse_pos.x * 512.0) as usize, (mouse_pos.y * 512.0) as usize);

            self.add_radius(pixel_pos, 12);
        }
    }

    pub fn draw_update_color(&mut self, ctx: &egui::Context, img_rect: Rect, color: Color32) {
        if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary) && (i.pointer.delta().length() > 0.1 || i.pointer.press_start_time() == Some(0.0)) && img_rect.contains(i.pointer.interact_pos().unwrap())) {
            let mouse_pos = (ctx.pointer_interact_pos().unwrap() - img_rect.left_top()) / vec2(img_rect.width(), img_rect.height());
            // println!("{}", mouse_pos);
            let pixel_pos = Vector2::<usize>::new((mouse_pos.x * 512.0) as usize, (mouse_pos.y * 512.0) as usize);

            self.add_radius_color(pixel_pos, 12, color);
        }
    }


    pub fn new() -> Self {
        Self {
            texture: egui::ColorImage::new([512, 512], Color32::BLACK)
            // texture: colorimage_from_image("ur mom")
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
        self.add_radius_color(pos, radius, Color32::from_rgb(26, 26, 26));
    }

    fn add_radius_color(&mut self, pos: Vector2<usize>, radius: usize, color: Color32) {
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

                let (dx, dy, dz) = (color.r() as f32 / 255.0, color.g() as f32 / 255.0, color.b() as f32 / 255.0);

                // println!("{}, {}, {}", dx, dy, dz);

                v.x += dx;
                v.x = v.x.clamp(0.0, 1.0);
                
                v.y += dy;
                v.y = v.y.clamp(0.0, 1.0);
                
                v.z += dz;
                v.z = v.z.clamp(0.0, 1.0);

                self.texture.pixels[y * 512 + x] = vec4_to_col(v);
            }
        }
    }
}


pub fn col_to_vec4(col: Color32) -> Vector4<f32> {
    let col : Vector4<u8> = [col.r(), col.g(), col.b(), col.a()].into();
    let col :  Vector4<f32> = col.map(|x| (x as f32) / 255.0);
    col
}

pub fn vec4_to_col(vec: Vector4<f32>) -> Color32 {
    vec.iter().for_each(|f| {
        if *f > 1.0 || *f < 0.0 {
            panic!("Illegal Color Supplied");
        }
    });
    let col = vec.map(|x| (x * 255.0) as u8);
    return Color32::from_rgba_unmultiplied(col.x, col.y, col.z, col.w);
}




pub fn bicubic_downsize(img: ColorImage, target_size: usize) -> ColorImage{
    if img.size[0] != img.size[1] {
        panic!("Supplied image is not square");
    }
    // if img.size[0] < target_size {
    //     panic!("Attempting to upscale image, illegal");
    // }

    let og_size = img.size[0];
    let scale = (og_size as f32) / (target_size as f32);

    let mut new_image = ColorImage::new([target_size, target_size], Color32::BLACK);

    for y in 0..target_size {
        for x in 0..target_size {
            let x = x as f32;
            let y = y as f32;

            let src_x = x * scale;
            let src_y = y * scale;

            let x0 = src_x.floor() - 1.0;
            let y0 = src_y.floor() - 1.0;

            let mut result = Vector3::new(0.0, 0.0, 0.0);

            for i in 0..4 {
                for j in 0..4 {
                    let px = (x0 + j as f32).clamp(0.0, og_size as f32 -1.0);
                    let py = (y0 + i as f32).clamp(0.0, og_size as f32 -1.0);
                    let pixel = img[(px as usize, py as usize)];

                    let wx = cubic_weight((x0 + (j as f32) - src_x).abs());
                    let wy = cubic_weight((y0 + (i as f32) - src_y).abs());

                    result += Vector3::new(pixel.r() as f32, pixel.g() as f32, pixel.b() as f32) * wx * wy;
                }
            }

            let col = Color32::from_rgb(result.x.clamp(0.0, 255.0) as u8, result.y.clamp(0.0, 255.0) as u8, result.z.clamp(0.0, 255.0) as u8);

            new_image[(x as usize, y as usize)] = col;
        }
    };

    new_image
}



fn cubic_weight(t: f32) -> f32{
    let a = -0.5;

    if t < 1.0 {
        (a + 2.0) * t.powf(3.0) - (a + 3.0) * t.powf(2.0) + 1.0
    } else if t < 2.0 {
        (a) * t.powf(3.0) - 5.0 * (a) * t.powf(2.0) + 8.0 * a * t - 4.0 * a
    } else {
        0.0
    }
}



pub fn colorimage_from_image(path: &str) -> ColorImage {
    let img = image::open(path).unwrap().into_rgba8();

    let (width, height) = img.dimensions();

    let pixels = img.into_raw();

    let color_image = ColorImage::from_rgba_unmultiplied([width as _, height as _], &pixels);

    let width = width.min(height);

    let img = color_image.region(&Rect {
        min: egui::Pos2 { x: 0.0, y: 0.0 },
        max: egui::Pos2 { x: width as f32, y: width as f32 },
    }, None);

    bicubic_downsize(img, 512)
}


pub fn colorimage_to_bw(img: &ColorImage) -> ColorImage {
    let dim = img.size;

    println!("Calling BW");



    let pixels: Vec<u8> = img.as_raw()
        .chunks_exact(4)
        .map(|x| {
            let sum = ((x[0] as u32 + x[1] as u32 + x[2] as u32) / 3) as u8;
            [sum, sum, sum, 255] // Expand to RGB grayscale
        })
        .flatten()
        .collect();


    println!("{}, {}", pixels.len(), img.as_raw().len());
    assert!(pixels.len() == img.as_raw().len());

    println!("Return BW");


    let x = ColorImage::from_rgba_unmultiplied(dim, &pixels);

    println!("Return BW 2");

    x
}