use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{Window, WindowBuilder},
};

struct State<'window> {
    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
}

impl<'window> State<'window> {
    async fn new(window: Window) -> State<'window> {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = unsafe {
            instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap())
        }
        .expect("can create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("can create device");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("can create a new device");

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!();
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!();
    }

    fn update(&mut self) {
        todo!();
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!();
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("can create an event lopp");
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    pollster::block_on(run(event_loop, window));
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let state = State::new(window).await;

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::KeyboardInput { event, .. } => {
                            match event.key_without_modifiers().as_ref() {
                                Key::Character("q") | Key::Named(NamedKey::Escape) => elwt.exit(),
                                _ => (),
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            let output = state
                                .surface
                                .get_current_texture()
                                .expect("can get texture");
                            let view = output
                                .texture
                                .create_view(&wgpu::TextureViewDescriptor::default());
                            let mut encoder = state.device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor { label: None },
                            );
                            {
                                let _render_pass =
                                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                        label: None,
                                        color_attachments: &[Some(
                                            wgpu::RenderPassColorAttachment {
                                                view: &view,
                                                resolve_target: None,
                                                ops: wgpu::Operations {
                                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                                        r: 0.1,
                                                        g: 0.9,
                                                        b: 0.3,
                                                        a: 1.0,
                                                    }),
                                                    store: wgpu::StoreOp::Store,
                                                },
                                            },
                                        )],
                                        depth_stencil_attachment: None,
                                        timestamp_writes: None,
                                        occlusion_query_set: None,
                                    });
                                //     render_pass.set_pipeline(&render_pipeline);
                                //     render_pass.draw(0..3, 0..1);
                            }

                            state.queue.submit(Some(encoder.finish()));
                            output.present();
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            println!("moved {position:?}");
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            println!("mouse button {button:?} pressed {state:?}");
                        }
                        _ => (),
                    };
                }
                Event::AboutToWait => {
                    state.window.request_redraw();
                }
                _ => (),
            }
        })
        .expect("event loop runs");
}
