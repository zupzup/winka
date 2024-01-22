use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("can create an event lopp");
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => elwt.exit(),
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { event, .. },
            ..
        } => match event.key_without_modifiers().as_ref() {
            Key::Character("q") | Key::Named(NamedKey::Escape) => elwt.exit(),
            _ => (),
        },
        _ => (),
    });
}
