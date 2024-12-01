use std::{collections::HashMap, sync::Arc};

use wgpu::core::device;

use crate::texture;


pub struct AssetManager{
    texture_map: HashMap<String, Arc<texture::Texture>>,
}

impl AssetManager{

    pub fn new() -> Self{
        Self{
            texture_map: HashMap::new()
        }
    }

    pub fn add_texture(
        &mut self,
        filepath : &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Arc<texture::Texture>{

        match self.texture_map.get(filepath){
            Some(texture_handle) => {
                Arc::clone(&texture_handle)
            },
            None => {
                let texture = AssetManager::load_texture(filepath, device, queue);
                let texture = Arc::new(texture);
                let texture_handle : Arc<texture::Texture>= Arc::clone(&texture);
                self.texture_map.insert(filepath.to_string(), texture);
                texture_handle
            }
        }
    }

    pub fn get_texture_handle(&self, filepath : &str) -> Option<Arc<texture::Texture>>{
        match self.texture_map.get(filepath){
            Some(handle) => Some(Arc::clone(handle)),
            None => None,
        }
    }

    pub fn load_texture(
        filepath : &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> texture::Texture{
        let diffuse_bytes =
            std::fs::read(filepath).expect(
                &format!("file {} could not be found", filepath)
            );
        texture::Texture::from_bytes(
            &device, &queue, diffuse_bytes.as_slice(), filepath).unwrap()
    }

}
