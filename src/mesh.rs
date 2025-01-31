use eframe::glow::{self, Context, HasContext as _};
use egui::ColorImage;
use nalgebra::{Vector2, Vector3, Vector4};



#[derive(Debug)]
pub struct Mesh {
    pub positions: Vec<Vector3<f32>>,
    pub indicies : Vec<u32>,
    uvs: Vec<Vector2<f32>>,
    colors: Vec<Vector4<f32>>,
    pub vertex_array: glow::VertexArray,
    pub position_buffer: glow::Buffer,
    pub color_buffer: glow::Buffer,
    pub index_buffer: glow::Buffer,
    pub uv_buffer: glow::Buffer,
    pub index_buffer_size: u32,
    pub wireframe: bool
}


impl Mesh {
    pub fn new(gl: &glow::Context, positions: Vec<Vector3<f32>>, indicies: Vec<u32>, uvs: Vec<Vector2<f32>>, wireframe: bool) -> Self {
        use glow::HasContext as _;

        unsafe {
            let vert_count = positions.len();

            let mut uvs = uvs.clone();
            
            let mut colors: Vec<Vector4<f32>> = Vec::new();

            for (i, pos) in positions.iter().enumerate() {
                let i = i as f32;

                let rand = rand::random::<f32>().fract();

                if i as i32 % 3 == 0 {
                    let col = Vector3::new(
                        rand::random::<f32>().fract(),
                        rand::random::<f32>().fract(),
                        rand::random::<f32>().fract()
                        // 0.5, 0.5, 0.5
                        // rand, rand, rand
                    );
                    let col = col.push(1.0);
                    colors.push(col);
                } else {
                    colors.push(colors[i as usize - 1]);
                }
            }

            let position_buffer: glow::NativeBuffer = gl.create_buffer().expect("Cannot create position buffer");
            let color_buffer = gl.create_buffer().expect("Cannot create color buffer");
            let uv_buffer = gl.create_buffer().expect("Cannot create uv buffer");
            let index_buffer = gl.create_buffer().expect("Cannot create index buffer");

            let vertex_array = gl.create_vertex_array().expect("Cannot create vertex array");

            let mut x = Self {
                positions: positions.clone(), 
                indicies: indicies.clone(),
                uvs: uvs.clone(),
                colors: colors.clone(),
                vertex_array,
                position_buffer,
                color_buffer,
                index_buffer,
                uv_buffer,
                index_buffer_size: (if wireframe {2} else {1})*indicies.len() as u32,
                wireframe
            };

            x.load_buffers(gl);

            x
        }
    }


    pub fn load_buffers(&mut self, gl: &glow::Context) {
        unsafe {
            self.position_buffer = gl.create_buffer().expect("Cannot create position buffer");
            self.color_buffer = gl.create_buffer().expect("Cannot create color buffer");
            self.uv_buffer = gl.create_buffer().expect("Cannot create uv buffer");
            self.index_buffer = gl.create_buffer().expect("Cannot create index buffer");

            self.vertex_array = gl.create_vertex_array().expect("Cannot create vertex array");

            // gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.index_buffer));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&self.indicies.chunks_exact(3).map(|x| {
                if self.wireframe {
                    [x[0], x[1], x[1], x[2], x[2], x[0]].to_vec()
                } else {
                    [x[0], x[1], x[2]].to_vec()
                }
            } ).flatten().collect::<Vec<u32>>()), glow::STATIC_DRAW);

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.position_buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&self.positions.iter().flat_map(|x| {
                vec![x.x, x.y, x.z, 1.0].into_iter()
            }).collect::<Vec<f32>>()), glow::STATIC_DRAW);
            gl.vertex_attrib_pointer_f32(0, 4, glow::FLOAT, false, 0, 0);  // Position (2 floats per vertex)
            gl.enable_vertex_attrib_array(0);  // Enable position attribute

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.color_buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&self.colors.iter().flat_map(|x| {
                if !self.wireframe {
                    vec![x.x, x.y, x.z, x.w].into_iter()
                } else {
                    vec![1.0, 1.0, 1.0, 1.0].into_iter()
                }
                
            }).collect::<Vec<f32>>()), glow::STATIC_DRAW);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 0, 0);  // Color (4 floats per vertex)
            gl.enable_vertex_attrib_array(1);  // Enable color attribute

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.uv_buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&self.uvs.iter().flat_map(|x|{
                vec![x.x, x.y].into_iter()
            }).collect::<Vec<f32>>()), glow::STATIC_DRAW);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(2);  // Enable uv attribute

            self.index_buffer_size = (if self.wireframe {2} else {1})*self.indicies.len() as u32;
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_buffer(self.position_buffer);
            gl.delete_vertex_array(self.vertex_array);
            gl.delete_buffer(self.color_buffer);
            gl.delete_buffer(self.index_buffer);
            gl.delete_buffer(self.uv_buffer);
        }
    }

}




pub fn generate_tiled_plane(gl: &Context, width: f32, height: f32, tiles_x: usize, tiles_y: usize) -> Mesh {
    let tile_width = width / tiles_x as f32;
    let tile_height = height / tiles_y as f32;

    let mut positions: Vec<Vector3<f32>> = Vec::new();
    let mut uvs: Vec<Vector2<f32>> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for y in 0..tiles_y {
        for x in 0..tiles_x {
            // Compute the position offsets for each tile
            let offset_x = x as f32 * tile_width - width / 2.0;
            let offset_y = y as f32 * tile_height - height / 2.0;

            // Define the positions for the corners of the current tile
            let base_idx = (y * tiles_x + x) * 4;
            positions.push([offset_x, 0.0, offset_y].into());                 // Bottom-left
            positions.push([offset_x + tile_width, 0.0, offset_y].into());    // Bottom-right
            positions.push([offset_x + tile_width, 0.0, offset_y + tile_height].into()); // Top-right
            positions.push([offset_x, 0.0, offset_y + tile_height].into());   // Top-left

            // Define the UV coordinates for the current tile
            let uv_offset_x = x as f32 / tiles_x as f32;
            let uv_offset_y = y as f32 / tiles_y as f32;
            uvs.push([uv_offset_x, uv_offset_y].into());                      // Bottom-left
            uvs.push([uv_offset_x + 1.0 / tiles_x as f32, uv_offset_y].into()); // Bottom-right
            uvs.push([uv_offset_x + 1.0 / tiles_x as f32, uv_offset_y + 1.0 / tiles_y as f32].into()); // Top-right
            uvs.push([uv_offset_x, uv_offset_y + 1.0 / tiles_y as f32].into()); // Top-left

            // Define the indices for the two triangles forming the tile
            indices.push(base_idx as u32);
            indices.push((base_idx + 1) as u32);
            indices.push((base_idx + 2) as u32);
            indices.push((base_idx) as u32);
            indices.push((base_idx + 2) as u32);
            indices.push((base_idx + 3) as u32);
        }
    }

    // Create the mesh
    Mesh::new(&gl,
        positions,
        indices,
        uvs,
        false
    )
}





pub fn generate_tiled_plane_colorimg(gl: &Context, width: f32, height: f32, tiles_x: usize, tiles_y: usize, img: ColorImage) -> Mesh {
    let tile_width = width / tiles_x as f32;
    let tile_height = height / tiles_y as f32;

    let mut positions: Vec<Vector3<f32>> = Vec::new();
    let mut uvs: Vec<Vector2<f32>> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for y in 0..tiles_y {
        for x in 0..tiles_x {
            // Compute the position offsets for each tile
            let offset_x = x as f32 * tile_width - width / 2.0;
            let offset_y = y as f32 * tile_height - height / 2.0;

            let height_0  = img.pixels[y * 512 + x].to_array().iter().map(|x| *x as f32).sum::<f32>() * ((3.0 / 255.0) / 4.0);
            let height_1  = img.pixels[(y + 1) * 512 + x].to_array().iter().map(|x| *x as f32).sum::<f32>() * ((3.0 / 255.0) / 4.0);
            let height_2  = img.pixels[y * 512 + (x + 1)].to_array().iter().map(|x| *x as f32).sum::<f32>() * ((3.0 / 255.0) / 4.0);
            let height_3  = img.pixels[(y+1) * 512 + (x+1)].to_array().iter().map(|x| *x as f32).sum::<f32>() * ((3.0 / 255.0) / 4.0);

            // Define the positions for the corners of the current tile
            let base_idx = (y * tiles_x + x) * 4;
            positions.push([offset_x, height_0, offset_y].into());                 // Bottom-left
            positions.push([offset_x + tile_width, height_2, offset_y].into());    // Bottom-right
            positions.push([offset_x + tile_width, height_1, offset_y + tile_height].into()); // Top-right
            positions.push([offset_x, height_3, offset_y + tile_height].into());   // Top-left

            // Define the UV coordinates for the current tile
            let uv_offset_x = x as f32 / tiles_x as f32;
            let uv_offset_y = y as f32 / tiles_y as f32;
            uvs.push([uv_offset_x, uv_offset_y].into());                      // Bottom-left
            uvs.push([uv_offset_x + 1.0 / tiles_x as f32, uv_offset_y].into()); // Bottom-right
            uvs.push([uv_offset_x + 1.0 / tiles_x as f32, uv_offset_y + 1.0 / tiles_y as f32].into()); // Top-right
            uvs.push([uv_offset_x, uv_offset_y + 1.0 / tiles_y as f32].into()); // Top-left

            // Define the indices for the two triangles forming the tile
            indices.push(base_idx as u32);
            indices.push((base_idx + 1) as u32);
            indices.push((base_idx + 2) as u32);
            indices.push((base_idx) as u32);
            indices.push((base_idx + 2) as u32);
            indices.push((base_idx + 3) as u32);
        }
    }

    // Create the mesh
    Mesh::new(&gl,
        positions,
        indices,
        uvs,
        false
    )
}


