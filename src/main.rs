use actor_game::actor::game::Game;
use actor_game::actor::{
    Actor, Vector2, Ship, InputComponent, MoveComponent,
};
use std::time::{Duration, Instant};
use winit::{
    event::{Event, WindowEvent, ElementState, KeyboardInput},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder as WinitWindowBuilder,
};
use wgpu;
use pollster;
use std::f32::consts::PI;

async fn init() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    adapter.request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap()
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WinitWindowBuilder::new()
        .with_title("Actor Game")
        .build(&event_loop)
        .unwrap();

    let mut game = Game::new(&window);
    let actor_id = game.add_actor();
    game.setup_player_actor(actor_id);

    let mut last_update_time = Instant::now();
    let target_frame_duration = Duration::from_secs_f32(1.0 / 60.0);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(keycode),
                        state,
                        ..
                    },
                    ..
                },
                ..
            } => {
                let pressed = state == ElementState::Pressed;
                game.handle_keyboard_input(keycode, pressed);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                game.shutdown();
            }
            Event::MainEventsCleared => {
                let current_time = Instant::now();
                let delta_time = current_time.duration_since(last_update_time).as_secs_f32();
                last_update_time = current_time;

                game.set_delta_time(delta_time);
                game.run();

                window.request_redraw();
            }
            _ => (),
        }
    });
}
