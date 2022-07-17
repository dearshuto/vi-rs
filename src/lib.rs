#[cfg(any(not(target_arch = "wasm32")))]
pub mod glutin;

#[cfg(any(not(target_arch = "wasm32")))]
pub mod winit;

#[cfg(target_arch = "wasm32")]
pub mod web_sys;

#[cfg(any(not(target_arch = "wasm32")))]
pub trait IDisplayEventListener {
    fn on_resized(&mut self, _width: u32, _height: u32) {}
}

pub trait IInstance {
    type DisplayId: Eq + PartialEq + Clone + Copy;
    type Display: IDisplay;

    fn new() -> Self;

    fn create_display(&mut self) -> Self::DisplayId;

    fn try_get_display(&self, id: &Self::DisplayId) -> Option<&Self::Display>;

    fn try_update(&mut self) -> bool;
}

pub trait IDisplay {
    fn is_redraw_requested(&self) -> bool;

    fn listen<TListener: IDisplayEventListener>(&self, listener: &mut TListener);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
