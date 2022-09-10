use super::gfx::{self, PRESENT_MODE};
use imgui::{Context, DrawData, FontSource, Ui};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window as WinitWindow,
};

pub struct Builder {
    pub gfx_builder: gfx::Builder,
    pub clear_colour: wgpu::Color,
}

pub struct Window {
    winit_win: WinitWindow,
    pub gfx: gfx::GfxContext,
    pub clear_colour: wgpu::Color,
}

impl Window {
    fn frame(&mut self, draw_data: &DrawData) {
        self.gfx.render_pass(self.clear_colour, draw_data).present();
    }

    fn resize_to_inner(&mut self) {
        let size = self.winit_win.inner_size();
        self.gfx.resize(size);
    }
}

impl Builder {
    pub fn run(self, mut render_frame: impl FnMut(&mut Window, &Ui, &mut ControlFlow) + 'static) {
        let event_loop = EventLoop::new();
        let window = WinitWindow::new(&event_loop).expect("Unable to create window using winit");

        let (mut gfx, mut imgui) = self.gfx_builder.build(&window);

        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);

        let mut last_ts = Instant::now();

        let mut window = Window {
            winit_win: window,
            gfx,
            clear_colour: self.clear_colour,
        };

        let mut last_frame = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::MainEventsCleared => window.winit_win.request_redraw(),
                Event::RedrawEventsCleared => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                Event::RedrawRequested(_) => {
                    platform
                        .prepare_frame(imgui.io_mut(), &window.winit_win)
                        .expect("Failed to prepare frame");
                    let ui = imgui.frame();
                    render_frame(&mut window, &ui, control_flow);
                    platform.prepare_render(&ui, &window.winit_win);
                    let draw_data = ui.render();
                    window.frame(draw_data);
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => window.resize_to_inner(),
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                Event::LoopDestroyed => {
                    panic!("loop destroyed!")
                }
                _ => {}
            }
            platform.handle_event(imgui.io_mut(), &window.winit_win, &event);
        });
    }
}
