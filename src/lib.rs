use crate::winit::MouseEvent;

pub mod glutin;
pub mod winit;

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
