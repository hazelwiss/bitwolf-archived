mod debug_views;
mod gfx;

use crate::{cli::CliArgs, config, emu::EmuState};

use self::gfx::{imgui_ctx::ImguiCtx, window::WindowGfx};
use ::imgui::Ui;
use std::time::Instant;
use util::log::Logger;
#[allow(unused)]
use util::log::{self, info};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct GuiState {
    window: WindowGfx,
    imgui: ImguiCtx,
    last_frame: Instant,
    #[allow(unused)]
    log: Logger,
    debug_views: debug_views::DebugViews,
    emu_state: EmuState,
    config: config::GlobalConfig,
}

impl GuiState {
    fn run<T>(mut self, event_loop: EventLoop<T>) {
        event_loop.run(move |event, _, flow| {
            *flow = ControlFlow::Poll;
            self.imgui
                .platform
                .handle_event(self.imgui.ctx.io_mut(), &self.window.window, &event);
            match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(_) => self.resize_to_inner(),
                    WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                    #[allow(unused)]
                    WindowEvent::MouseInput { state, button, .. } => {}
                    #[allow(unused)]
                    WindowEvent::KeyboardInput { input, .. } => {}
                    _ => {}
                },
                Event::MainEventsCleared => self.window.request_redraw(),
                Event::RedrawEventsCleared => {
                    self.update_dela_time(Instant::now());
                    self.draw()
                }
                Event::LoopDestroyed => *flow = ControlFlow::Exit,
                _ => {}
            }
        });
    }

    fn draw(&mut self) {
        let Self {
            window,
            imgui,
            debug_views,
            ..
        } = self;
        let gfx = &window.gfx;
        let frame = gfx
            .surface
            .get_current_texture()
            .expect("unabel to create frame");

        let mut encoder = gfx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        drop(gfx);
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(window.clear_colour),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            imgui
                .platform
                .prepare_frame(imgui.ctx.io_mut(), &window)
                .expect("Failed to prepare frame");
            let ui = imgui.ctx.frame();
            ui_update(window, debug_views, &ui);
            imgui
                .renderer
                .render(
                    ui.render(),
                    &window.gfx.queue,
                    &window.gfx.device,
                    &mut rpass,
                )
                .expect("Failed to render imgui frame.");
        }
        window.gfx.queue.submit(Some(encoder.finish()));
        frame.present()
    }

    fn update_dela_time(&mut self, instant: Instant) {
        self.imgui
            .ctx
            .io_mut()
            .update_delta_time(instant - self.last_frame);
        self.last_frame = instant;
    }

    fn resize_to_inner(&mut self) {
        let size = self.window.inner_size();
        self.resize(size);
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.window.update_size(size);
        }
    }
}

pub fn run(#[allow(unused)] log: log::Logger, cli_args: CliArgs) {
    let config = if let Some(path) = cli_args.config {
        config::load_config(&path)
    } else {
        config::global_config()
    };
    let emu_state = if let Some(path) = cli_args.rom {
        EmuState::new_with_rom(log.clone(), &config.emu, &path)
    } else {
        EmuState::new(log.clone())
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Bitworlf")
        .build(&event_loop)
        .expect("failure to build winit window");
    let mut window = WindowGfx::new(window, wgpu::Color::BLACK);

    let imgui = ImguiCtx::new(&mut window, None);
    let last_frame = Instant::now();

    let gui_state = GuiState {
        window,
        imgui,
        last_frame,
        log,
        debug_views: Default::default(),
        emu_state,
        config,
    };

    gui_state.run(event_loop);
}

fn ui_update(window: &mut WindowGfx, debug_views: &mut debug_views::DebugViews, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu("file", || {});
        ui.menu("options", || {});
        debug_views.menu(ui);
    });
    debug_views.draw(window, ui);
}
