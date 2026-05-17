use std::error::Error;
use std::io;
use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

const WINDOW_WIDTH: f64 = 520.0;
const WINDOW_HEIGHT: f64 = 360.0;
const PALETTES: [(u32, u32, u32); 4] = [
    (0x20, 0x70, 0xc0),
    (0xc0, 0x40, 0x60),
    (0x30, 0xa0, 0x60),
    (0xb0, 0x80, 0x30),
];

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let context = Context::new(event_loop.owned_display_handle())?;
    let mut app = GuiDApp::new(context);

    event_loop.run_app(&mut app)?;
    app.finish()
}

struct GuiDApp {
    context: Context<OwnedDisplayHandle>,
    window: Option<Rc<Window>>,
    surface: Option<Surface<OwnedDisplayHandle, Rc<Window>>>,
    palette_index: usize,
    click_count: u32,
    cursor_position: Option<(u32, u32)>,
    runtime_error: Option<String>,
}

impl GuiDApp {
    fn new(context: Context<OwnedDisplayHandle>) -> Self {
        Self {
            context,
            window: None,
            surface: None,
            palette_index: 0,
            click_count: 0,
            cursor_position: None,
            runtime_error: None,
        }
    }

    fn finish(self) -> Result<(), Box<dyn Error>> {
        if let Some(message) = self.runtime_error {
            return Err(io::Error::other(message).into());
        }

        Ok(())
    }

    fn ensure_window(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attributes = Window::default_attributes()
            .with_title("gui-d: winit + softbuffer Sample")
            .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));

        let window = match event_loop.create_window(attributes) {
            Ok(window) => Rc::new(window),
            Err(error) => {
                self.fail(
                    event_loop,
                    format!("[gui-d] ウィンドウ生成に失敗しました: {error}"),
                );
                return;
            }
        };

        let surface = match Surface::new(&self.context, window.clone()) {
            Ok(surface) => surface,
            Err(error) => {
                self.fail(
                    event_loop,
                    format!("[gui-d] softbuffer の初期化に失敗しました: {error}"),
                );
                return;
            }
        };

        window.request_redraw();
        self.surface = Some(surface);
        self.window = Some(window);
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        let window = self
            .window
            .as_ref()
            .ok_or_else(|| io::Error::other("[gui-d] ウィンドウが初期化されていません"))?;
        let surface = self
            .surface
            .as_mut()
            .ok_or_else(|| io::Error::other("[gui-d] 描画サーフェスが初期化されていません"))?;
        let size = window.inner_size();
        let width = match NonZeroU32::new(size.width) {
            Some(width) => width,
            None => return Ok(()),
        };
        let height = match NonZeroU32::new(size.height) {
            Some(height) => height,
            None => return Ok(()),
        };

        surface.resize(width, height)?;

        let mut buffer = surface.buffer_mut()?;
        render_pixels(
            &mut buffer,
            width.get(),
            height.get(),
            PALETTES[self.palette_index],
            self.click_count,
            self.cursor_position,
        );
        window.pre_present_notify();
        buffer.present()?;
        Ok(())
    }

    fn fail(&mut self, event_loop: &ActiveEventLoop, message: String) {
        self.runtime_error = Some(message);
        event_loop.exit();
    }
}

impl ApplicationHandler for GuiDApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Wait);
        self.ensure_window(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) => {
                window.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position =
                    Some((position.x.max(0.0) as u32, position.y.max(0.0) as u32));
                window.request_redraw();
            }
            WindowEvent::CursorLeft { .. } => {
                self.cursor_position = None;
                window.request_redraw();
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                self.click_count = self.click_count.wrapping_add(1);
                self.palette_index = (self.palette_index + 1) % PALETTES.len();
                window.request_redraw();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match logical_key.as_ref() {
                Key::Named(NamedKey::Escape) => event_loop.exit(),
                Key::Named(NamedKey::Space) => {
                    self.palette_index = (self.palette_index + 1) % PALETTES.len();
                    window.request_redraw();
                }
                _ => {}
            },
            WindowEvent::RedrawRequested => {
                if let Err(error) = self.render() {
                    self.fail(event_loop, format!("[gui-d] 描画に失敗しました: {error}"));
                }
            }
            _ => {}
        }
    }
}

fn render_pixels(
    pixels: &mut [u32],
    width: u32,
    height: u32,
    palette: (u32, u32, u32),
    click_count: u32,
    cursor_position: Option<(u32, u32)>,
) {
    let width_usize = width as usize;
    let band_height = height.min(28);
    let filled_band_width = if width == 0 {
        0
    } else {
        click_count % width.max(1)
    };

    for y in 0..height {
        for x in 0..width {
            let index = y as usize * width_usize + x as usize;
            let red = ((x * 255) / width.max(1) + palette.0) % 256;
            let green = ((y * 255) / height.max(1) + palette.1) % 256;
            let blue = (((x + y) * 255) / (width + height).max(1) + palette.2) % 256;

            pixels[index] = pack_rgb(red, green, blue);

            if y < band_height {
                pixels[index] = if x < filled_band_width {
                    pack_rgb(0xf5, 0xd9, 0x0a)
                } else {
                    pack_rgb(0x18, 0x18, 0x18)
                };
            }
        }
    }

    if let Some((cursor_x, cursor_y)) = cursor_position {
        let clamped_x = cursor_x.min(width.saturating_sub(1));
        let clamped_y = cursor_y.min(height.saturating_sub(1));
        draw_crosshair(pixels, width_usize, height, clamped_x, clamped_y);
    }
}

fn draw_crosshair(pixels: &mut [u32], width: usize, height: u32, x: u32, y: u32) {
    let highlight = pack_rgb(0xff, 0xff, 0xff);
    let accent = pack_rgb(0xff, 0x50, 0x50);

    let row = y as usize * width;
    for draw_x in 0..width {
        let color = if draw_x as u32 == x {
            accent
        } else {
            highlight
        };
        pixels[row + draw_x] = color;
    }

    for draw_y in 0..height as usize {
        let index = draw_y * width + x as usize;
        pixels[index] = accent;
    }
}

fn pack_rgb(red: u32, green: u32, blue: u32) -> u32 {
    blue | (green << 8) | (red << 16)
}
