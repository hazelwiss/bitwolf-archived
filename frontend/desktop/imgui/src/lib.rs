mod create_context;

pub use create_context::DrawContext;
pub use winit::event::{KeyboardInput, MouseButton};

use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub mod gui {
    pub use imgui::*;
}

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

    pub fn run<UpdateF, InputF>(mut self, mut update_f: UpdateF, mut input_f: InputF)
    where
        UpdateF: FnMut(&mut DrawContext) + 'static,
        InputF: FnMut(Input) + 'static,
    {
        /* Get the current time of the program. */
        let mut last_frame = Instant::now();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            self.wgpu_imgui_ctx
                .handle_events(&event, &mut last_frame, &mut update_f);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::MouseInput { button, .. } => input_f(Input::MouseButton(button)),
                    WindowEvent::KeyboardInput { input, .. } => input_f(Input::Keyboard(input)),
                    _ => {}
                },
                _ => {}
            }
        });
    }
}

pub mod colour {
    #[derive(Clone, Copy, Default, Debug)]
    pub struct BGRA(u8, u8, u8, u8);

    impl BGRA {
        pub const WHITE: BGRA = BGRA::new(0xFF, 0xFF, 0xFF, 0xFF);
        pub const BLACK: BGRA = BGRA::new(0x00, 0x00, 0x00, 0x00);
        pub const RED: BGRA = BGRA::new(0x00, 0x00, 0xFF, 0xFF);
        pub const BLUE: BGRA = BGRA::new(0xFF, 0x00, 0x00, 0xFF);
        pub const GREEN: BGRA = BGRA::new(0x00, 0xFF, 0x00, 0xFF);

        #[inline(always)]
        pub const fn new(b: u8, g: u8, r: u8, a: u8) -> BGRA {
            BGRA(b, g, r, a)
        }

        #[inline(always)]
        pub fn full(&self) -> u32 {
            ((self.0 as u32) << 24)
                | ((self.1 as u32) << 16)
                | ((self.2 as u32) << 8)
                | (self.3 as u32)
        }

        #[inline(always)]
        pub fn r(&self) -> u8 {
            self.2
        }

        #[inline(always)]
        pub fn g(&self) -> u8 {
            self.1
        }

        #[inline(always)]
        pub fn b(&self) -> u8 {
            self.0
        }
    }
}
