use glyphon::{Color, FontSystem, Resolution, SwashCache, TextArea, TextAtlas, TextRenderer};
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

mod button;
mod rectangle;
mod text;
mod text_field;

#[repr(C)]
#[derive(Clone, Debug, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    rect: [f32; 4],
    border_color: [f32; 3],
}

pub enum Component {
    Button(button::Button),
    TextField(text_field::TextField),
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

struct InputState {
    clicked: bool,
    mouse_coords: PhysicalPosition<f64>,
}

struct State<'window> {
    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,
    text_renderer: TextRenderer,
    text_atlas: TextAtlas,
    text_cache: SwashCache,
    font_system: FontSystem,
    components: Vec<Component>,
    input_state: InputState,
}

impl<'window> State<'window> {
    async fn new(window: Window) -> State<'window> {
        let size = window.inner_size();
        let mouse_coords = PhysicalPosition { x: 0.0, y: 0.0 };
        let input_state = InputState {
            clicked: false,
            mouse_coords,
        };

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
            .expect("can create adapter");

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

        let button = button::Button::new(
            button::ButtonConfig {
                rect_pos: RectPos {
                    top: 100,
                    left: 100,
                    bottom: 400,
                    right: 500,
                },
                fill_color: [0.5, 0.0, 0.5],
                fill_color_active: [1.0, 0.0, 1.0],
                border_color: [0.0, 0.0, 0.0],
                border_color_active: [1.0, 1.0, 1.0],
                text: "Submit 🚀",
                text_color: Color::rgb(200, 200, 200),
                text_color_active: Color::rgb(255, 255, 255),
                on_click: || {
                    println!("button 1 clicked");
                },
            },
            &mut font_system,
        );

        let button2 = button::Button::new(
            button::ButtonConfig {
                rect_pos: RectPos {
                    top: 600,
                    left: 600,
                    bottom: 700,
                    right: 800,
                },
                fill_color: [0.3, 0.0, 0.3],
                fill_color_active: [0.8, 0.0, 0.8],
                border_color: [0.3, 0.3, 0.3],
                border_color_active: [0.1, 0.1, 0.1],
                text: "Button! 🚀",
                text_color: Color::rgb(200, 200, 200),
                text_color_active: Color::rgb(255, 255, 255),
                on_click: || {},
            },
            &mut font_system,
        );

        let button3 = button::Button::new(
            button::ButtonConfig {
                rect_pos: RectPos {
                    top: 300,
                    left: 900,
                    bottom: 400,
                    right: 1000,
                },
                fill_color: [0.3, 0.0, 0.3],
                fill_color_active: [0.8, 0.0, 0.8],
                border_color: [0.3, 0.3, 0.3],
                border_color_active: [0.1, 0.1, 0.1],
                text: "3!",
                text_color: Color::rgb(200, 200, 200),
                text_color_active: Color::rgb(255, 255, 255),
                on_click: || {},
            },
            &mut font_system,
        );

        let text_field = text_field::TextField::new(
            text_field::TextFieldConfig {
                rect_pos: RectPos {
                    top: 10,
                    left: 10,
                    bottom: 90,
                    right: 200,
                },
                fill_color: [0.9, 0.9, 0.9],
                fill_color_active: [1.0, 1.0, 1.0],
                border_color: [0.3, 0.3, 0.3],
                border_color_active: [0.1, 0.1, 0.1],
                text_color: Color::rgb(10, 10, 10),
            },
            &mut font_system,
        );

        let components = vec![
            Component::Button(button),
            Component::Button(button2),
            Component::Button(button3),
            Component::TextField(text_field),
        ];

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            text_atlas,
            text_cache,
            text_renderer,
            font_system,
            components,
            input_state,
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

    fn handle_click(&mut self) {
        self.components
            .iter_mut()
            .for_each(|component| match component {
                Component::Button(button) => {
                    if button.is_hovered(self.input_state.mouse_coords) {
                        button.click();
                    }
                }
                Component::TextField(text_field) => {
                    if text_field.is_hovered(self.input_state.mouse_coords) {
                        text_field.set_active();
                    } else {
                        text_field.set_inactive();
                    }
                }
            });
    }

    fn input(&mut self, event: &WindowEvent, elwt: &EventLoopWindowTarget<()>) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.input_state.mouse_coords = position.to_owned();
                true
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => {
                    if button == &winit::event::MouseButton::Left && !self.input_state.clicked {
                        self.input_state.clicked = true;
                        self.handle_click();
                    }
                    log::info!(
                        "{button:?} mouse button pressed at {:?}, clicked: {}",
                        self.input_state.mouse_coords,
                        self.input_state.clicked
                    );
                    true
                }
                ElementState::Released => {
                    if button == &winit::event::MouseButton::Left && self.input_state.clicked {
                        self.input_state.clicked = false;
                    }
                    log::info!(
                        "{button:?} mouse button released at {:?}, size: {:?}, clicked: {}",
                        self.input_state.mouse_coords,
                        self.size,
                        self.input_state.clicked
                    );
                    true
                }
            },
            WindowEvent::KeyboardInput { event, .. } => {
                if let Key::Named(NamedKey::Escape) = event.key_without_modifiers() {
                    elwt.exit()
                }

                self.components
                    .iter_mut()
                    .filter_map(|component| match component {
                        Component::TextField(text_field) => {
                            if text_field.is_active() {
                                Some(text_field)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .for_each(|text_field| {
                        if event.state == ElementState::Pressed {
                            match event.key_without_modifiers().as_ref() {
                                Key::Named(NamedKey::Backspace) => {
                                    text_field.remove_character(&mut self.font_system);
                                }
                                Key::Named(NamedKey::Enter) => (),
                                _ => {
                                    if let Some(ref txt) = event.text {
                                        text_field.add_text(&mut self.font_system, txt.as_str());
                                    }
                                }
                            }
                        }
                    });
                true
            }
            _ => false,
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let mut text_areas: Vec<TextArea> = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let mut num_vertices = 0;
        let mut num_indices = 0;
        self.components
            .iter_mut()
            .for_each(|component| match component {
                Component::Button(button) => {
                    let button_active = button.is_hovered(self.input_state.mouse_coords);
                    let button_vertices = button.rectangle().vertices(button_active, self.size);

                    vertices.extend_from_slice(&button_vertices);
                    indices.extend_from_slice(&button.rectangle().indices(num_vertices));

                    num_vertices += button_vertices.len() as u16;
                    num_indices += button.rectangle().num_indices();

                    text_areas.push(
                        button
                            .text()
                            .text_area(button_active && self.input_state.clicked),
                    );
                }
                Component::TextField(text_field) => {
                    let text_field_active = text_field.is_active();
                    let text_field_vertices = text_field
                        .rectangle_mut()
                        .vertices(text_field_active, self.size);
                    let cursor_vertices = text_field.get_cursor().vertices(false, self.size);

                    vertices.extend_from_slice(&text_field_vertices);
                    indices.extend_from_slice(&text_field.rectangle().indices(num_vertices));

                    num_vertices += text_field_vertices.len() as u16;
                    num_indices += text_field.rectangle().num_indices();

                    if text_field_active {
                        vertices.extend_from_slice(&cursor_vertices);
                        indices.extend_from_slice(&text_field.get_cursor().indices(num_vertices));
                        num_vertices += cursor_vertices.len() as u16;
                        num_indices += text_field.get_cursor().num_indices();
                    }

                    text_areas.push(
                        text_field
                            .text()
                            .text_area(text_field_active && self.input_state.clicked),
                    );
                }
            });

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&indices),
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
                text_areas,
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
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
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
            render_pass.draw_indexed(0..(num_indices), 0, 0..1);

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
                        match state.render() {
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
