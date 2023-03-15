use smithay::{
    delegate_xdg_decoration, delegate_xdg_shell,
    desktop::{Space, Window},
    input::{pointer::GrabStartData as PointerGrabStartData, Seat},
    reexports::{
        wayland_protocols::xdg::{
            decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode, shell::server::xdg_toplevel,
        },
        wayland_server::{
            protocol::{wl_seat, wl_surface::WlSurface},
            Resource,
        },
    },
    utils::{Serial, Size},
    wayland::{
        compositor::with_states,
        shell::xdg::{
            decoration::XdgDecorationHandler, PopupSurface, PositionerState, ToplevelSurface,
            XdgShellHandler, XdgShellState, XdgToplevelSurfaceData,
        },
    },
};

use crate::{utils::bsp_layout, PoopLand};

impl XdgShellHandler for PoopLand {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let window = Window::new(surface);
        self.space.map_element(window, (0, 0), false);
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        // TODO: Popup handling using PopupManager
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {
        // TODO popup grabs
    }
}

// Disable decorations
impl XdgDecorationHandler for PoopLand {
    fn new_decoration(&mut self, toplevel: ToplevelSurface) {
        toplevel.with_pending_state(|state| {
            // Advertise server side decoration
            state.decoration_mode = Some(Mode::ServerSide);
        });
        toplevel.send_configure();
    }
    fn request_mode(&mut self, _toplevel: ToplevelSurface, _mode: Mode) { /* ... */
    }
    fn unset_mode(&mut self, _toplevel: ToplevelSurface) { /* ... */
    }
}

// Xdg Decoration
delegate_xdg_decoration!(PoopLand);
// Xdg Shell
delegate_xdg_shell!(PoopLand);

fn check_grab(
    seat: &Seat<PoopLand>,
    surface: &WlSurface,
    serial: Serial,
) -> Option<PointerGrabStartData<PoopLand>> {
    let pointer = seat.get_pointer()?;

    // Check that this surface has a click grab.
    if !pointer.has_grab(serial) {
        return None;
    }

    let start_data = pointer.grab_start_data()?;

    let (focus, _) = start_data.focus.as_ref()?;
    // If the focus was for a different surface, ignore the request.
    if !focus.id().same_client_as(&surface.id()) {
        return None;
    }

    Some(start_data)
}

/// Should be called on `WlSurface::commit`
pub fn handle_commit(space: &mut Space<Window>, surface: &WlSurface) -> Option<()> {
    let window = space
        .elements()
        .find(|w| w.toplevel().wl_surface() == surface)
        .cloned()?;

    let initial_configure_sent = with_states(surface, |states| {
        states
            .data_map
            .get::<XdgToplevelSurfaceData>()
            .unwrap()
            .lock()
            .unwrap()
            .initial_configure_sent
    });

    if !initial_configure_sent {
        let layout = bsp_layout(space);
        let windows: Vec<_> = space.elements().cloned().collect();
        for (i, window) in windows.iter().enumerate() {
            space.map_element(window.clone(), layout[i].loc, false);
            let xdg_toplevel = window.toplevel();
            xdg_toplevel.with_pending_state(|state| {
                state.size = Some(layout[i].size);
            });
            xdg_toplevel.send_configure();
        }
    }

    Some(())
}
