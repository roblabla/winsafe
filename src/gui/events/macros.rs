/// Declares a struct of control events, which is just a proxy to parent events.
macro_rules! ctrl_events_proxy {
	(
		$(#[$doc:meta])*
		$name:ident
	) => {
		$(#[$doc])*
		pub struct $name {
			parent_ptr: NonNull<Base>,
			ctrl_id: u16,
		}

		impl $name {
			pub(crate) fn new(parent_ref: &Base, ctrl_id: u16) -> $name {
				Self {
					parent_ptr: NonNull::from(parent_ref), // convert reference to pointer
					ctrl_id,
				}
			}

			fn parent_user_events(&self) -> &WindowEvents {
				unsafe { self.parent_ptr.as_ref().user_events_ref() }
			}
		}
	}
}

/// Declares a method for a `WM_COMMAND` notification.
macro_rules! cmd_event {
	(
		$name:ident, $cmd:expr,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut() + 'static,
		{
			self.parent_user_events().wm_command($cmd, self.ctrl_id, {
				let mut func = func;
				move || func()
			});
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives a NMHDR
/// parameter, which is not passed because it carries no useful data, and whose
/// callback has no return.
macro_rules! nfy_event {
	(
		$name:ident, $nfy:expr,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut() + 'static,
		{
			self.parent_user_events().add_nfy(self.ctrl_id, $nfy, {
				let mut func = func;
				move |_| { func(); None }
			});
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives a parameter,
/// and whose callback has no return.
macro_rules! nfy_event_p {
	(
		$name:ident, $nfy:expr, $struc:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut(&$struc) + 'static,
		{
			self.parent_user_events().add_nfy(self.ctrl_id, $nfy, {
				let mut func = func;
				move |p| { func(unsafe { p.cast_nmhdr::<$struc>() }); None }
			});
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives a mutable
/// parameter, and whose callback has no return.
macro_rules! nfy_event_mut_p {
	(
		$name:ident, $nfy:expr, $struc:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut(&mut $struc) + 'static,
		{
			self.parent_user_events().add_nfy(self.ctrl_id, $nfy, {
				let mut func = func;
				move |p| { func(unsafe { p.cast_nmhdr_mut::<$struc>() }); None }
			});
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives a parameter,
/// and whose callback returns bool.
macro_rules! nfy_event_p_bool {
	(
		$name:ident, $nfy:expr, $struc:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut(&$struc) -> bool + 'static,
		{
			self.parent_user_events().add_nfy(self.ctrl_id, $nfy, {
				let mut func = func;
				move |p| Some(func(unsafe { p.cast_nmhdr::<$struc>() }) as isize)
			});
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives a mutable
/// parameter, and whose callback returns bool.
macro_rules! nfy_event_mut_p_bool {
	(
		$name:ident, $nfy:expr, $struc:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut(&mut $struc) -> bool + 'static,
		{
			self.parent_user_events().add_nfy(self.ctrl_id, $nfy, {
				let mut func = func;
				move |p| Some(func(unsafe { p.cast_nmhdr_mut::<$struc>() }) as isize)
			});
		}
	};
}
