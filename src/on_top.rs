//! 窗口保持在最前 (#127): the "…" menu floats this window above other apps'
//! windows, like a sticky note, until it is turned off or the window closes
//! (it isn't remembered).
use gpui::Window;

/// Whether this platform keeps a window on top: macOS and Windows; Linux
/// (X11 and Wayland differ, and Wayland leaves it to the compositor) hides the item.
pub const SUPPORTED: bool = cfg!(any(target_os = "macos", target_os = "windows"));

/// Float `window` above other apps' windows, or put it back among them.
pub fn set(window: &Window, on: bool) {
    let handle = crate::printing::parent_of(window);
    if handle == 0 { return }
    #[cfg(target_os = "macos")]
    unsafe {
        use objc::runtime::Object;
        use objc::{msg_send, sel, sel_impl};
        let win: *mut Object = msg_send![handle as *mut Object, window];
        if win.is_null() { return }
        // NSFloatingWindowLevel, or back to NSNormalWindowLevel
        let _: () = msg_send![win, setLevel: if on { 3isize } else { 0isize }];
    }
    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetWindowPos};
        if let Err(e) = SetWindowPos(HWND(handle as *mut std::ffi::c_void), Some(if on { HWND_TOPMOST } else { HWND_NOTOPMOST }), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE) {
            log::warn!("on top: {e}");
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let _ = on;
}
