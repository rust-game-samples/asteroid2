use image::GenericImageView;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub struct TextureManager {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    textures: HashMap<String, Texture>,
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: (u32, u32),
}

impl TextureManager {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            device,
            queue,
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, filename: &str) -> Option<u32> {
        if self.textures.contains_key(filename) {
            return Some(self.textures.len() as u32);
        }

        let path = Path::new("assets").join(filename);
        let img = image::open(path).ok()?;
        let dimensions = img.dimensions();

        let texture = self.device.create_texture_with_data(
            &self.queue,
            &wgpu::TextureDescriptor {
                label: Some(filename),
                size: wgpu::Extent3d {
                    width: dimensions.0,
                    height: dimensions.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            &img.to_rgba8(),
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        self.textures.insert(
            filename.to_string(),
            Texture {
                texture,
                view,
                sampler,
                size: dimensions,
            },
        );

        Some(self.textures.len() as u32)
    }

    pub fn get_texture(&self, filename: &str) -> Option<&Texture> {
        self.textures.get(filename)
    }
}
