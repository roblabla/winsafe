use std::any::Any;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::events::{ButtonEvents, WindowEvents};
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, calc_text_bound_box_check, multiply_dpi, ui_font};
use crate::gui::traits::{baseref_from_parent, Child, Parent};
use crate::handles::HWND;
use crate::msg::{bm, wm};
use crate::structs::POINT;

/// Native
/// [radio button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#radio-buttons)
/// control, actually a variation of the ordinary
/// [`Button`](crate::gui::Button): just a button with a specific style.
///
/// Implements [`Child`](crate::gui::Child) trait.
///
/// You cannot directly instantiate this object, you must use
/// [`RadioGroup`](crate::gui::RadioGroup).
pub struct RadioButton(Obj);

struct Obj { // actual fields of RadioButton
	base: NativeControlBase,
	opts_id: OptsId<RadioButtonOpts>,
	events: ButtonEvents,
}

impl Child for RadioButton {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl RadioButton {
	pub(crate) fn new(parent: &dyn Parent, opts: RadioButtonOpts) -> RadioButton {
		let parent_ref = baseref_from_parent(parent);
		let opts = RadioButtonOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		Self(
			Obj {
				base: NativeControlBase::new(parent_ref),
				opts_id: OptsId::Wnd(opts),
				events: ButtonEvents::new(parent_ref, ctrl_id),
			},
		)
	}

	pub(crate) fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> RadioButton {
		let parent_ref = baseref_from_parent(parent);

		Self(
			Obj {
				base: NativeControlBase::new(parent_ref),
				opts_id: OptsId::Dlg(ctrl_id),
				events: ButtonEvents::new(parent_ref, ctrl_id),
			},
		)
	}

	pub(crate) fn create(&self) -> WinResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				if opts.baseline_text_align { pos.y += 3; }
				multiply_dpi(Some(&mut pos), None)?;

				let bound_box = calc_text_bound_box_check(&opts.text)?;

				let our_hwnd = self.0.base.create_window( // may panic
					"BUTTON", Some(&opts.text), pos, bound_box,
					opts.ctrl_id,
					opts.ex_window_style,
					opts.window_style | opts.button_style.into(),
				)?;

				our_hwnd.SendMessage(wm::SetFont{ hfont: ui_font(), redraw: true });
			},
			OptsId::Dlg(ctrl_id) => {
				self.0.base.create_dlg(*ctrl_id)?; // may panic
			},
		}

		self.hwnd().SendMessage(bm::SetDontClick { dont_click: true });
		Ok(())
	}

	hwnd_ctrlid_on_onsubclass!(ButtonEvents);

	/// Tells if this radio button is currently checked by sending a
	/// [`BM_GETCHECK`](crate::msg::bm::GetCheck) message.
	pub fn is_checked(&self) -> bool {
		self.hwnd().SendMessage(bm::GetCheck {}) == co::BST::CHECKED
	}

	/// Sets the current check state by sending a
	/// [`BM_SETCHECK`](crate::msg::bm::SetCheck) message.
	pub fn set_check(&self, checked: bool) {
		self.hwnd().SendMessage(bm::SetCheck {
			state: if checked { co::BST::CHECKED } else { co::BST::UNCHECKED },
		});
	}

	/// Fires the click event for the radio button by posting a
	/// [`BM_CLICK`](crate::msg::bm::Click) message. The event is asynchronous,
	/// the method returns immediately.
	pub fn trigger_click(&self) -> WinResult<()> {
		self.hwnd().PostMessage(bm::Click {})
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`RadioButton`](crate::gui::RadioButton) programatically
/// with [`RadioButton::new`](crate::gui::RadioButton::new).
pub struct RadioButtonOpts {
	/// Text of the control to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Will adjust `position.cy` so that, if the control is placed side-by-side
	/// with an [`Edit`](crate::gui::Edit) control, their texts will be aligned.
	///
	/// Defaults to false.
	pub baseline_text_align: bool,
	/// Radio button styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::AUTORADIOBUTTON`.
	pub button_style: co::BS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE`.
	///
	/// The first RadioButton of a group should also have `WS::TABSTOP | WS::GROUP`.
	/// If this object being passed to a [`RadioGroup`](crate::gui::RadioGroup),
	/// this will be automatically set.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub ex_window_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
}

impl Default for RadioButtonOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: POINT::new(0, 0),
			baseline_text_align: false,
			button_style: co::BS::AUTORADIOBUTTON,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			ex_window_style: co::WS_EX::LEFT,
			ctrl_id: 0,
		}
	}
}

impl RadioButtonOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}

	pub(crate) fn manual_clone(&self) -> RadioButtonOpts { // avoids a public clone method
		Self {
			text: self.text.clone(),
			position: self.position,
			baseline_text_align: self.baseline_text_align,
			button_style: self.button_style,
			window_style: self.window_style,
			ex_window_style: self.ex_window_style,
			ctrl_id: self.ctrl_id,
		}
	}
}
