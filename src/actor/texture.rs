use image::GenericImageView;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub struct TextureManager {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    textures: HashMap<String, Arc<Texture>>,
    bind_group_layout: wgpu::BindGroupLayout,
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
    pub size: (u32, u32),
}

impl TextureManager {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        // バインドグループレイアウトを作成
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        Self {
            device,
            queue,
            textures: HashMap::new(),
            bind_group_layout,
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

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(filename),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        self.textures.insert(
            filename.to_string(),
            Arc::new(Texture {
                texture,
                view,
                sampler,
                bind_group,
                size: dimensions,
            }),
        );

        Some(self.textures.len() as u32)
    }

    pub fn get_texture(&self, filename: &str) -> Option<Arc<Texture>> {
        self.textures.get(filename).cloned()
    }
}
