use crate::state::State;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler, event::*, event_loop::ActiveEventLoop, keyboard::PhysicalKey,
    window::Window,
};

pub struct App {
    state: Option<State>,
    config_zad_1: bool,
}

impl App {
    pub fn new(config: bool) -> Self {
        Self {
            state: None,
            config_zad_1: config,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Project 1")
            .with_inner_size(winit::dpi::LogicalSize::new(640, 480));
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.state = Some(pollster::block_on(State::new(window, self.config_zad_1)).unwrap());
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            WindowEvent::CursorMoved { position, .. } => {
                state.handle_mouse_moved(position.x, position.y)
            }
            _ => {}
        }
    }
}
