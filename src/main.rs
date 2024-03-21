use glyphon::{Color, FontSystem, Resolution, SwashCache, TextAtlas, TextRenderer};
use rectangle::*;
use std::time::SystemTime;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{Window, WindowBuilder},
};

mod rectangle;
mod text;

#[repr(C)]
#[derive(Clone, Debug, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    rect: [f32; 4],
    border_color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 10]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

struct State<'window> {
    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    mouse_coords: PhysicalPosition<f64>,
    render_pipeline: wgpu::RenderPipeline,
    use_color: bool,
    clicked: bool,
    text_renderer: TextRenderer,
    text_atlas: TextAtlas,
    text_cache: SwashCache,
    font_system: FontSystem,
    rectangle: Rectangle,
    text: text::Text,
}

impl<'window> State<'window> {
    async fn new(window: Window) -> State<'window> {
        let size = window.inner_size();
        let mouse_coords = PhysicalPosition { x: 0.0, y: 0.0 };

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

        let swapchain_format = wgpu::TextureFormat::Bgra8UnormSrgb;

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &config);

        let mut font_system = FontSystem::new();
        let text_cache = SwashCache::new();
        let mut text_atlas = TextAtlas::new(&device, &queue, swapchain_format);
        let text_renderer = TextRenderer::new(
            &mut text_atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            depth_stencil: None,
            multiview: None,
        });

        let rectangle = Rectangle::new(
            RectPos {
                top: 100,
                left: 100,
                bottom: 400,
                right: 500,
            },
            [0.5, 0.0, 0.5],
            [1.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 1.0],
        );

        let text = text::Text::new(
            &mut font_system,
            rectangle.position().to_owned(),
            "Submit 🚀",
            Color::rgb(255, 255, 255),
        );

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            mouse_coords,
            render_pipeline,
            use_color: true,
            clicked: false,
            text_atlas,
            text_cache,
            text_renderer,
            font_system,
            rectangle,
            text,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent, elwt: &EventLoopWindowTarget<()>) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_coords = position.to_owned();
                true
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => {
                    if button == &winit::event::MouseButton::Left {
                        self.clicked = true;
                    }
                    log::info!(
                        "{button:?} mouse button pressed at {:?}, clicked: {}",
                        self.mouse_coords,
                        self.clicked
                    );
                    true
                }
                ElementState::Released => {
                    if button == &winit::event::MouseButton::Left {
                        self.clicked = false;
                    }
                    log::info!(
                        "{button:?} mouse button released at {:?}, size: {:?}, clicked: {}",
                        self.mouse_coords,
                        self.size,
                        self.clicked
                    );
                    true
                }
            },
            WindowEvent::KeyboardInput { event, .. } => {
                match event.key_without_modifiers().as_ref() {
                    Key::Character("q") | Key::Named(NamedKey::Escape) => elwt.exit(),
                    Key::Named(NamedKey::Space) => {
                        self.use_color = event.state == ElementState::Released
                    }
                    _ => (),
                }
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {}

    fn render(&mut self, color: wgpu::Color) -> Result<(), wgpu::SurfaceError> {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.rectangle.vertices(
                    self.mouse_coords,
                    self.clicked,
                    self.size,
                )),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.rectangle.indices()),
                usage: wgpu::BufferUsages::INDEX,
            });

        self.text_renderer
            .prepare(
                &self.device,
                &self.queue,
                &mut self.font_system,
                &mut self.text_atlas,
                Resolution {
                    width: self.size.width,
                    height: self.size.height,
                },
                [self.text.text_area()],
                &mut self.text_cache,
            )
            .unwrap();

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.rectangle.num_indices(), 0, 0..1);
            self.text_renderer
                .render(&self.text_atlas, &mut render_pass)
                .unwrap();
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();
        self.text_atlas.trim();

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("can create an event lopp");
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    pollster::block_on(run(event_loop, window));
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut state = State::new(window).await;

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut then = SystemTime::now();
    let mut now = SystemTime::now();
    let mut fps = 0;
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent { window_id, event }
                if window_id == state.window().id() && !state.input(&event, elwt) =>
            {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(physical_size) => {
                        state.resize(physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        let blue = state.mouse_coords.x / state.size.width as f64;
                        let red = state.mouse_coords.y / state.size.height as f64;
                        let color = wgpu::Color {
                            r: red,
                            g: red * blue,
                            b: blue,
                            a: 1.0,
                        };
                        state.update();
                        match state.render(color) {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => log::error!("render error: {e:?}"),
                        }

                        fps += 1;
                        if now.duration_since(then).unwrap().as_millis() > 1000 {
                            state
                                .window()
                                .set_title(&format!("wgpu-text: 'simple' example, FPS: {}", fps));
                            fps = 0;
                            then = now;
                        }
                        now = SystemTime::now();
                    }
                    _ => (),
                };
            }
            Event::AboutToWait => {
                state.window.request_redraw();
            }
            _ => (),
        })
        .expect("event loop runs");
}
