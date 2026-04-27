mod app;
mod camera;
mod state;
mod vertices;
use winit::event_loop::EventLoop;

pub fn run(config: bool) -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = app::App::new(config);
    event_loop.run_app(&mut app)?;
    Ok(())
}
