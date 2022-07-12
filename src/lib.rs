#[cfg(any(not(target_arch = "wasm32")))]
use crate::winit::MouseEvent;

#[cfg(any(not(target_arch = "wasm32")))]
pub mod glutin;

#[cfg(any(not(target_arch = "wasm32")))]
pub mod winit;

#[cfg(target_arch = "wasm32")]
pub mod web_sys;

#[cfg(any(not(target_arch = "wasm32")))]
pub trait IDisplayEventListener {
    fn on_resized(&mut self, _width: u32, _height: u32) {}

    fn on_mouse_operated(&mut self, _mouse_event: MouseEvent) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
