use super::actor::Actor;
use super::input_component::InputComponent;
use super::laser::Laser;
use super::move_component::MoveComponent;
use super::ship::Ship;
use super::sprite_component::SpriteComponent;
use super::texture::TextureManager;
use super::vector2::Vector2;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::Arc;
use wgpu;
use winit::event::VirtualKeyCode;
use winit::window::Window;

/// ゲームの状態を管理する構造体
pub struct Game {
    /// アクティブなアクターのマップ
    actors: HashMap<u32, Actor>,
    /// 次のアクターID
    next_actor_id: u32,
    /// ゲームが実行中かどうか
    running: bool,
    /// 前回のフレームからの経過時間
    delta_time: f32,
    pub texture_manager: TextureManager,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    pressed_keys: Vec<VirtualKeyCode>,
}

impl Game {
    /// 新しいゲームインスタンスを作成
    pub fn new(window: &winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
                .unwrap();

        let device = Arc::new(device);
        let queue = Arc::new(queue);
        let mut texture_manager = TextureManager::new(device.clone(), queue.clone());

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        // シェーダーとパイプラインの設定
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sprite Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: None, // 自動レイアウト
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let mut game = Self {
            actors: HashMap::new(),
            next_actor_id: 1,
            running: true,
            delta_time: 0.0,
            device,
            queue,
            surface,
            surface_config,
            render_pipeline,
            texture_manager,
            pressed_keys: Vec::new(),
        };

        // テクスチャを事前にロード
        game.texture_manager.load_texture("Ship.png");
        game.texture_manager.load_texture("Asteroid.png");
        game.texture_manager.load_texture("Laser.png");

        game
    }

    /// 1フレーム分のゲーム更新を実行
    pub fn run(&mut self) {
        self.process_input();
        self.update_game();
        self.generate_output();
    }

    /// 新しいアクターを追加
    pub fn add_actor(&mut self) -> u32 {
        let actor_id = self.next_actor_id;
        self.actors.insert(actor_id, Actor::new());
        self.next_actor_id += 1;
        actor_id
    }

    /// アクターを削除
    pub fn remove_actor(&mut self, actor_id: u32) {
        self.actors.remove(&actor_id);
    }

    /// 全てのアクターを更新
    fn update_game(&mut self) {
        // アクティブな全てのアクターを更新
        for actor in self.actors.values_mut() {
            actor.update(self.delta_time);
        }
    }

    /// 入力処理
    fn process_input(&mut self) {
        println!("Processing input with keys: {:?}", self.pressed_keys);
        for actor in self.actors.values_mut() {
            if let Some(input) = actor.get_component_mut::<InputComponent>() {
                input.process_input(&self.pressed_keys);
            }
        }
    }

    /// 出力生成
    fn generate_output(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            // アクターのスプライトを描画
            for actor in self.actors.values() {
                if let Some(sprite) = actor.get_component::<SpriteComponent>() {
                    sprite.draw(&mut render_pass);
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    /// ゲームを終了
    pub fn shutdown(&mut self) {
        self.running = false;
    }

    /// デルタタイムを設定
    pub fn set_delta_time(&mut self, delta_time: f32) {
        self.delta_time = delta_time;
    }

    /// アクターを取得
    pub fn get_actor(&self, actor_id: u32) -> Option<&Actor> {
        self.actors.get(&actor_id)
    }

    /// アクターを可変で取得
    pub fn get_actor_mut(&mut self, actor_id: u32) -> Option<&mut Actor> {
        self.actors.get_mut(&actor_id)
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn create_laser(&mut self, pos: Vector2, rot: f32) -> u32 {
        let laser_id = self.add_actor();
        if let Some(laser_actor) = self.actors.get_mut(&laser_id) {
            laser_actor.set_position(pos);
            laser_actor.set_rotation(rot);
            laser_actor.set_scale(Vector2::new(1.0, 1.0));
            laser_actor.add_component(Box::new(Laser::new(&mut self.texture_manager)));
        }
        laser_id
    }

    // キー押下状態を更新するメソッドを追加
    pub fn handle_keyboard_input(&mut self, keycode: VirtualKeyCode, pressed: bool) {
        println!("Key event: {:?}, pressed: {}", keycode, pressed);
        if pressed {
            if !self.pressed_keys.contains(&keycode) {
                self.pressed_keys.push(keycode);
                println!("Current pressed keys: {:?}", self.pressed_keys);
            }
        } else {
            self.pressed_keys.retain(|&k| k != keycode);
        }
    }

    pub fn create_ship(&mut self) -> Ship {
        Ship::new(&mut self.texture_manager)
    }

    pub fn setup_player_actor(&mut self, actor_id: u32) {
        if let Some(actor) = self.actors.get_mut(&actor_id) {
            actor.set_position(Vector2::new(100.0, 100.0));
            actor.set_rotation(0.0);
            actor.set_scale(Vector2::one());

            let ship = Ship::new(&mut self.texture_manager);
            let sprite = SpriteComponent::new("Asteroid.png", 100, &mut self.texture_manager);

            let mut move_comp = MoveComponent::new(PI, 300.0);
            let mut input_comp = InputComponent::new(300.0, PI);
            input_comp.set_move_component(&mut move_comp);

            actor.add_component(Box::new(sprite));
            actor.add_component(Box::new(move_comp));
            actor.add_component(Box::new(input_comp));
            actor.add_component(Box::new(ship));
        }
    }
}
