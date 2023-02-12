mod compositor;
mod xdg_shell;

use crate::PoopLand;

//
// Wl Seat
//

use smithay::input::{SeatHandler, SeatState};
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::wayland::data_device::{
    ClientDndGrabHandler, DataDeviceHandler, ServerDndGrabHandler,
};
use smithay::{delegate_data_device, delegate_output, delegate_seat};

impl SeatHandler for PoopLand {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<PoopLand> {
        &mut self.seat_state
    }

    fn cursor_image(
        &mut self,
        _seat: &smithay::input::Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }
    fn focus_changed(&mut self, _seat: &smithay::input::Seat<Self>, _focused: Option<&WlSurface>) {}
}

delegate_seat!(PoopLand);

//
// Wl Data Device
//

impl DataDeviceHandler for PoopLand {
    fn data_device_state(&self) -> &smithay::wayland::data_device::DataDeviceState {
        &self.data_device_state
    }
}

impl ClientDndGrabHandler for PoopLand {}
impl ServerDndGrabHandler for PoopLand {}

delegate_data_device!(PoopLand);

//
// Wl Output & Xdg Output
//

delegate_output!(PoopLand);
