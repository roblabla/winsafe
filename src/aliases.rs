//! Aliases to Win32 types.

use crate::co;
use crate::handles::HWND;

/// A specialized
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) for Win32
/// operations, which return a [`ERROR`](crate::co::ERROR) on failure.
pub type WinResult<T> = Result<T, co::ERROR>;

/// Type alias to
/// [`DLGPROC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-dlgproc)
/// callback function.
pub type DLGPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
	) -> isize;

/// Type alias to
/// [`HOOKPROC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-hookproc)
/// callback function.
pub type HOOKPROC =
	extern "system" fn(
		code: i32,
		wParam: usize,
		lParam: isize,
	) -> isize;

/// Type alias to
/// [`TIMERPROC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-timerproc)
/// callback function.
pub type TIMERPROC =
	extern "system" fn(
		hWnd: HWND,
		msg: co::WM,
		timerId: usize,
		nSeconds: u32,
	);

/// Type alias to
/// [`SUBCLASSPROC`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-subclassproc)
/// callback function.
pub type SUBCLASSPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
		uIdSubclass: usize,
		dwRefData: usize,
	) -> isize;

/// Type alias to
/// [`WNDENUMPROC`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms633493(v=vs.85))
/// callbak function.
pub type WNDENUMPROC =
	extern "system" fn (
		hwnd: HWND,
		lParam: isize,
	) -> i32;

/// Type alias to
/// [`WNDPROC`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms633573(v=vs.85))
/// callback function.
pub type WNDPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
	) -> isize;
