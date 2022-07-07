mod create_context;

pub extern crate imgui as gui;
pub extern crate winit as win_api;

pub use create_context::DrawContext;
pub use create_context::WGPUContext;
pub use winit::event::{KeyboardInput, MouseButton};

use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

#[derive(Debug)]
pub enum Input {
    Keyboard(KeyboardInput),
    MouseButton(MouseButton),
}

pub struct Context {
    wgpu_imgui_ctx: create_context::WGPUImguiContext,
    event_loop: EventLoop<()>,
}

impl Context {
    pub fn spawn_with_window() -> Self {
        let event_loop = EventLoop::new();
        let wgpu_imgui_ctx =
            create_context::WGPUImguiContext::create_imgui_window(&event_loop, wgpu::Color::BLACK);
        Self {
            event_loop,
            wgpu_imgui_ctx,
        }
    }

    pub fn wgpu_ctx(&mut self) -> &mut WGPUContext {
        &mut self.wgpu_imgui_ctx.wgpu_ctx
    }

    pub fn run<UpdateF, InputF, RunCtx>(
        mut self,
        mut ctx: RunCtx,
        mut update_f: UpdateF,
        mut input_f: InputF,
    ) where
        RunCtx: 'static,
        UpdateF: FnMut(&mut RunCtx, &mut DrawContext) + 'static,
        InputF: FnMut(&mut RunCtx, Input) + 'static,
    {
        /* Get the current time of the program. */
        let mut last_frame = Instant::now();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            self.wgpu_imgui_ctx
                .handle_events(&event, &mut last_frame, &mut ctx, &mut update_f);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::MouseInput { button, .. } => {
                        input_f(&mut ctx, Input::MouseButton(button))
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        input_f(&mut ctx, Input::Keyboard(input))
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}
