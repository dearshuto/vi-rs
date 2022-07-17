use glutin::event::{ElementState, Event, MouseButton};
use glutin::{ContextWrapper, PossiblyCurrent};
use std::collections::HashMap;
use std::{thread::sleep, time::Duration};

use glutin::dpi::PhysicalSize;
use glutin::event::Event::{MainEventsCleared, RedrawRequested, WindowEvent};
use glutin::event::WindowEvent::Resized;
use glutin::window::WindowId;
use glutin::{
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

use crate::{IDisplay, IDisplayEventListener, IInstance};

pub enum MouseEvent {
    Pressed(f64, f64, MouseButton),
    Released(f64, f64, MouseButton),
    Moved(f64, f64),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Id {
    id: WindowId,
}

pub struct Instance {
    event_loop: EventLoop<()>,
    display_map: HashMap<Id, Display>,
}

impl Instance {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            display_map: HashMap::new(),
        }
    }

    pub fn get_event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    pub fn create_display(&mut self) -> Id {
        self.create_display_with_size(1280, 960)
    }

    pub fn create_display_with_size(&mut self, width: u32, height: u32) -> Id {
        let window_builder = WindowBuilder::new().with_inner_size(PhysicalSize::new(width, height));
        let window = unsafe {
            glutin::ContextBuilder::new()
                // .with_shared_lists(other)
                // .with_depth_buffer(native_options.depth_buffer)
                // .with_multisampling(native_options.multisampling)
                // .with_srgb(true)
                // .with_stencil_buffer(native_options.stencil_buffer)
                // .with_vsync(native_options.vsync)
                .build_windowed(window_builder, &self.event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };

        let id = Id {
            id: window.window().id(),
        };
        let display = Display {
            window,
            is_close_requested: false,
            is_redraw_requested: false,
            width,
            height,

            // マウスイベント
            mouse_event: Vec::new(),
            current_mouse_position: (0.0, 0.0),
        };
        self.display_map.insert(id, display);
        id
    }

    pub fn try_update(&mut self) -> bool {
        self.try_update_direct_event_callback(|_| {})
    }

    pub fn try_update_direct_event_callback<TFunc: FnMut(&Event<()>)>(
        &mut self,
        mut func: TFunc,
    ) -> bool {
        for display in self.display_map.values_mut() {
            display.is_redraw_requested = false;
            display.mouse_event.clear();
        }

        self.event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            func(&event);

            match event {
                RedrawRequested(window_id) => {
                    if let Some(display) = self.display_map.get_mut(&Id { id: window_id }) {
                        display.window.swap_buffers().unwrap();
                        display.is_redraw_requested = true;
                    }
                }
                MainEventsCleared => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent {
                    ref event,
                    window_id,
                } => match event {
                    Resized(size) => {
                        if let Some(display) = self.display_map.get_mut(&Id { id: window_id }) {
                            display.width = size.width;
                            display.height = size.height;
                        }
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        self.display_map.remove(&Id { id: window_id });
                    }
                    winit::event::WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    } => {
                        if let Some(display) = self.display_map.get_mut(&Id { id: window_id }) {
                            display.mouse_event.push(MouseEvent::Pressed(
                                display.current_mouse_position.0,
                                display.current_mouse_position.1,
                                MouseButton::Left,
                            ))
                        }
                    }
                    winit::event::WindowEvent::MouseInput {
                        state: ElementState::Released,
                        button: MouseButton::Left,
                        ..
                    } => {
                        if let Some(display) = self.display_map.get_mut(&Id { id: window_id }) {
                            display.mouse_event.push(MouseEvent::Released(
                                display.current_mouse_position.0,
                                display.current_mouse_position.1,
                                MouseButton::Left,
                            ))
                        }
                    }
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        if let Some(display) = self.display_map.get_mut(&Id { id: window_id }) {
                            if !(display.current_mouse_position.0 == position.x
                                && display.current_mouse_position.1 == position.y)
                            {
                                display
                                    .mouse_event
                                    .push(MouseEvent::Moved(position.x, position.y));
                                display.current_mouse_position = (position.x, position.y);
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        });

        for display in self.display_map.values_mut() {
            display.window.window().request_redraw();
        }

        sleep(Duration::from_millis(16));

        !self.display_map.is_empty()
    }

    pub fn try_get_display(&self, id: Id) -> Option<&Display> {
        self.display_map.get(&id)
    }
}

pub struct Display {
    pub window: ContextWrapper<PossiblyCurrent, Window>,
    is_close_requested: bool,
    is_redraw_requested: bool,
    width: u32,
    height: u32,

    // マウス操作
    mouse_event: Vec<MouseEvent>,
    current_mouse_position: (f64, f64),
}

impl Display {
    pub fn should_close(&self) -> bool {
        self.is_close_requested
    }

    pub fn listen<TListener: IDisplayEventListener>(&self, listener: &mut TListener) {
        listener.on_resized(self.width, self.height);
    }

    pub fn is_redraw_requested(&self) -> bool {
        self.is_redraw_requested
    }

    pub fn get_mouse_events(&self) -> &[MouseEvent] {
        &self.mouse_event
    }
}

struct DummyListener;
impl IDisplayEventListener for DummyListener {
    fn on_resized(&mut self, _width: u32, _height: u32) {}
}

impl IInstance for Instance {
    type DisplayId = self::Id;
    type Display = self::Display;

    fn new() -> Self {
        Self::new()
    }

    fn create_display(&mut self) -> Self::DisplayId {
        self.create_display()
    }

    fn try_get_display(&self, id: &Self::DisplayId) -> Option<&Self::Display> {
        self.try_get_display(id.clone())
    }

    fn try_update(&mut self) -> bool {
        self.try_update()
    }
}

impl IDisplay for Display {
    fn is_redraw_requested(&self) -> bool {
        self.is_redraw_requested
    }

    fn listen<TListener: IDisplayEventListener>(&self, _listener: &mut TListener) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
