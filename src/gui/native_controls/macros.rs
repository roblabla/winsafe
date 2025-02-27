/// Implements methods common to controls.
macro_rules! hwnd_on_onsubclass {
	($evstruc:ident) => {
		pub(crate) fn base_ref(&self) -> &NativeControlBase {
			&self.0.base
		}

		/// Returns the underlying handle for this control.
		///
		/// **Note:** the handle is initially null, receiving an actual value only
		/// after the control is physically created, what usually happens right
		/// before [`WM_CREATE`](crate::gui::events::WindowEvents::wm_create) or
		/// [`WM_INITDIALOG`](crate::gui::events::WindowEvents::wm_init_dialog)
		/// events.
		pub fn hwnd(&self) -> HWND {
			*self.0.base.hwnd_ref()
		}

		/// Exposes the control events.
		///
		/// These event methods are just proxies to the
		/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent
		/// window, who is the real responsible for the child event handling.
		///
		/// # Panics
		///
		/// Panics if the control or the parent window are already created. Events
		/// must be set before control and parent window creation.
		pub fn on(&self) -> &$evstruc {
			if !self.0.base.hwnd_ref().is_null() {
				panic!("Cannot add events after the control is created.");
			} else if !self.0.base.parent_ref().hwnd_ref().is_null() {
				panic!("Cannot add events after the parent window is created.");
			}
			&self.0.events
		}

		/// Exposes the subclass events. If at least one event exists, the control
		/// will be
		/// [subclassed](https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
		///
		/// **Note:** Subclassing may impact performance, use with care.
		///
		/// # Panics
		///
		/// Panics if the control or the parent window are already created. Events
		/// must be set before control and parent window creation.
		pub fn on_subclass(&self) -> &WindowEvents {
			self.0.base.on_subclass()
		}
	};
}

/// Implements methods common to controls, plus `ctrl_id`.
macro_rules! hwnd_ctrlid_on_onsubclass {
	($evstruc: ident) => {
		hwnd_on_onsubclass!($evstruc);

		/// Returns the control ID.
		pub fn ctrl_id(&self) -> u16 {
			match &self.0.opts_id {
				OptsId::Wnd(opts) => opts.ctrl_id,
				OptsId::Dlg(ctrl_id) => *ctrl_id,
			}
		}
	};
}
