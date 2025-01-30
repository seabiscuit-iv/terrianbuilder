
use std::{default, ops::RangeInclusive, ptr::null, sync::{Arc, Mutex}};

use drawing::Drawing;
use mesh::{generate_tiled_plane, Mesh};
use tobj;

use camera::Camera;
use eframe::{egui, egui_glow, glow::{self, HasContext, RIGHT}};
use egui::{load::SizedTexture, vec2, Align, Color32, ColorImage, Frame, Image, Layout, Margin, Rect, Style, TextureHandle, TextureOptions, ViewportBuilder};
use nalgebra::{Vector2, Vector3};

mod shader;
use shader::ShaderProgram;

mod mesh;
mod drawing;


mod camera;


fn main() -> eframe::Result{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1060.0, 700.0]).with_position([100.0, 60.0]),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        depth_buffer: 16,
        ..Default::default()
    };
    eframe::run_native(
        "TerrainBuilder",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}


// Main App UI

struct App {
    drawing: Drawing,
    mesh: Arc<Mutex<Mesh>>,
    camera: Arc<Mutex<Camera>>,
    shader_program: Arc<Mutex<ShaderProgram>>,
    value: f32,
    angle: (f32, f32, f32),
    speed: f32
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Top Panel")
            .frame(egui::Frame { inner_margin: 
                Margin { 
                    left: (10.0), right: (10.0), top: (8.0), bottom: (8.0) 
                }, 
                ..egui::Frame::default()
            })
            .show(ctx, |ui| {

            });

        let mut img_rect : Rect = Rect::NOTHING;

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut rect = ui.max_rect();
            rect.set_height(rect.height());
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rect).layout(Layout::left_to_right(Align::Center)), |ui| {
                let w = ui.available_width()/2.0;
                let mut h : f32 = -1.0;
                
                egui::Frame::none().show(ui, |ui| {
                    ui.set_max_width(w);
                    
                    let mut rect = ui.max_rect();
                    rect.set_height(rect.height()/2.0);

                    
                    h = ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rect), |ui| {
                        img_rect = self.drawing.draw(ui, ctx).rect;
                    }).response.rect.height();
                });
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let mut rect = ui.max_rect();
                    rect.set_height(h);

                    ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rect), |ui| { 
                        self.custom_painting(ui);
                    });
                });
            });

            // ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            //     egui::Frame::canvas(ui.style()).show(ui, |ui| {
            //         self.custom_painting(ui);
            //     });
            //     // egui::Frame::canvas(ui.style()).show(ui, |ui| {
            //         // img_rect = self.drawing.draw(ui, ctx).rect;
            //     // });
            // }); 
            
            ui.label(format!("Verts: {}", self.mesh.lock().unwrap().positions.len()));
            ui.label(format!("Tris: {}", self.mesh.lock().unwrap().indicies.len()/3));

            ui.add_space(12.0);

            ui.collapsing("Visual Properties", |ui| {
                if ui.toggle_value(&mut self.mesh.lock().unwrap().wireframe, "Wireframe").clicked() {    
                    self.mesh.lock().unwrap().load_buffers(&_frame.gl().unwrap());
                }
            });

            ui.add_space(12.0);

            ui.collapsing("Camera Controls", |ui| {
                ui.label("Position");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut self.camera.lock().unwrap().pos.x));
                    ui.add(egui::DragValue::new(&mut self.camera.lock().unwrap().pos.y));
                    ui.add(egui::DragValue::new(&mut self.camera.lock().unwrap().pos.z));
                });
                ui.label("Rotation");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut self.angle.0));
                    ui.add(egui::DragValue::new(&mut self.angle.1));
                    ui.add(egui::DragValue::new(&mut self.angle.2));
                });
                ui.label("Speed");
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut self.speed, RangeInclusive::new(0.0, 20.0)));
                });
            });
        });

        // update logic
        let rot = nalgebra::Rotation3::from_euler_angles(
            self.angle.0.to_radians(), 
            self.angle.1.to_radians(), 
            self.angle.2.to_radians()
        );

        // MOVEMENT HANDLER 
        {
            if ctx.input(|i| i.key_down(egui::Key::W)) {
                let mut cam = self.camera.lock().unwrap();
                let look = cam.look;
                cam.pos += look * 0.01 * self.speed;
            }
            if ctx.input(|i| i.key_down(egui::Key::S)) {
                let mut cam = self.camera.lock().unwrap();
                let look = cam.look;
                cam.pos += look * -0.01 * self.speed;
            }
    
            if ctx.input(|i| i.key_down(egui::Key::A)) {
                let mut cam = self.camera.lock().unwrap();
                let right = cam.right;
                cam.pos += right * -0.01 * self.speed;
            }
    
            if ctx.input(|i| i.key_down(egui::Key::D)) {
                let mut cam = self.camera.lock().unwrap();
                let right = cam.right;
                cam.pos += right * 0.01 * self.speed;
            }
    
            if ctx.input(|i| i.key_down(egui::Key::Q)) {
                let mut cam = self.camera.lock().unwrap();
                let up = cam.get_up_vec() ;
                cam.pos += up * -0.01 * self.speed;
            }
            
            if ctx.input(|i| i.key_down(egui::Key::E)) {
                let mut cam = self.camera.lock().unwrap();
                let up = cam.get_up_vec() ;
                cam.pos += up * 0.01 * self.speed;
            }
    
        }


        //DRAWING LOGIC
        self.drawing.draw_update(ctx, img_rect);


        let look = rot * Vector3::new(0.0, 0.0, -1.0);
        let right = rot * Vector3::new(1.0, 0.0, 0.0);
        self.camera.lock().unwrap().right = right;
        self.camera.lock().unwrap().look = look;
        
        ctx.request_repaint();
    }
}


impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");

        let mesh = generate_tiled_plane(gl, 20.0, 20.0, 200, 200);

        let shader_program = ShaderProgram::new(gl, "src/main.vert.glsl", "src/main.frag.glsl");
        
        let camera = Camera::default();
        
        Self { 
            drawing: Drawing::new(),
            mesh: Arc::new(Mutex::new(mesh)), 
            shader_program: Arc::new(Mutex::new(shader_program)),
            camera: Arc::new(Mutex::new(camera)),
            value: 0.0,
            angle: (-20.0, 0.0, 0.0),
            speed: 10.0
        }
    }

    fn custom_painting(&mut self, ui : &mut egui::Ui) {
        let (w, h) = (ui.available_width(), ui.available_height() - 5.0);

        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(w, h) , egui::Sense::drag());

        self.camera.lock().unwrap().aspect_ratio = w / h;

        let shader_program = self.shader_program.clone();
        let mesh = self.mesh.clone();
        let camera = self.camera.clone();

        self.angle.0 += response.drag_motion().y * -0.1;
        self.angle.1 += response.drag_motion().x * -0.1;

        let value = self.value;

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                shader_program.lock().unwrap().paint(painter.gl(), &mesh.lock().unwrap(), &camera.lock().unwrap());
            })),
        };
        ui.painter().add(callback);
    }
}