use super::*;
#[derive(Clone, PartialEq, Default)]
pub struct WindowClassStyle {
    globa: bool,           //CS_GLOBALCLASS
    no_close_button: bool, //CS_NOCLOSE
    ver_draw: bool,        //CS_VREDRAW
    her_draw: bool,        //CS_HREDRAW
    dbl_clk_msg: bool,     //CS_DBLCLKS
    parentDC: bool,          //CS_PARENTDC
    save_bits: bool,         //CS_SAVEBITS
    byte_ailgn_client: bool, //CS_BYTEALIGNCLIENT
    byte_ailgn_window: bool, //CS_BYTEALIGNWINDOW
    drop_shadrow: bool,      //CS_DROPSHADOW
    DCtype: DCtype,
}
impl From<WNDCLASS_STYLES> for WindowClassStyle {
    fn from(ms_style: WNDCLASS_STYLES) -> Self {
    	Self {
			globa: ms_style.contains(CS_GLOBALCLASS),
			no_close_button:ms_style.contains(CS_NOCLOSE),
			ver_draw: ms_style.contains(CS_VREDRAW),
			her_draw: ms_style.contains(CS_HREDRAW),
			dbl_clk_msg: ms_style.contains(CS_DBLCLKS),
			parentDC: ms_style.contains(CS_PARENTDC),
			save_bits: ms_style.contains(CS_SAVEBITS),
			byte_ailgn_client: ms_style.contains(CS_BYTEALIGNCLIENT),
			byte_ailgn_window: ms_style.contains(CS_BYTEALIGNWINDOW),
			drop_shadrow: ms_style.contains(CS_DROPSHADOW),
			DCtype: ms_style.into(),
		}
    }
}
impl Into<WNDCLASS_STYLES> for WindowClassStyle {
    fn into(self) -> WNDCLASS_STYLES {
        let mut ms_style = WNDCLASS_STYLES(0u32);
		if self.globa {ms_style |= CS_GLOBALCLASS;};
		if self.no_close_button {ms_style |= CS_NOCLOSE;};
		if self.ver_draw {ms_style |= CS_VREDRAW;};
		if self.her_draw {ms_style |= CS_HREDRAW;};
		if self.dbl_clk_msg {ms_style |= CS_DBLCLKS;};
		if self.parentDC {ms_style |= CS_PARENTDC;};
		if self.save_bits {ms_style |= CS_SAVEBITS;};
		if self.byte_ailgn_client {ms_style |= CS_BYTEALIGNCLIENT;};
		if self.byte_ailgn_window {ms_style |= CS_BYTEALIGNWINDOW;};
		if self.drop_shadrow {ms_style |= CS_DROPSHADOW;};
        ms_style | self.DCtype.into()
    }
}
#[derive(Clone, PartialEq, Default)]
pub enum DCtype {
	#[default]
    DefaultDC,  //NULL
    WindowDC, //CS_OWNDC
    ClassDC,  //CS_CLASSDC
}
impl Into<WNDCLASS_STYLES> for DCtype {
	fn into(self) -> WNDCLASS_STYLES {
		use DCtype::*;
        match self {
            WindowDC => CS_OWNDC,
            ClassDC => CS_CLASSDC,
            DefaultDC => WNDCLASS_STYLES(0u32), 
        }
	}
}
impl From<WNDCLASS_STYLES> for DCtype {
	fn from(ms_style: WNDCLASS_STYLES) -> Self {
		use DCtype::*;
		if ms_style.contains(CS_OWNDC) {
			WindowDC
		} else if ms_style.contains(CS_CLASSDC) {
			ClassDC
		} else {
			DefaultDC
		}
	}
}
//-------------------------------------------------------------------------------
static WS_ONLYCAPTION: WINDOW_STYLE = WINDOW_STYLE(4194304u32);
#[derive(Clone, PartialEq, Default)]
pub enum WindowSizeState {
	#[default]
	None, //NULL
	Min, //WS_MINIMIZE
	Max, //WS_MAXIMIZE
}
impl From<WINDOW_STYLE> for WindowSizeState {
	fn from(ms_style: WINDOW_STYLE) -> Self {
		if ms_style.contains(WS_MAXIMIZE) {
			Self::Max
		} else if ms_style.contains(WS_MINIMIZE) {
			Self::Min
		} else {
			Self::None
		}
	}
}
impl Into<WINDOW_STYLE> for WindowSizeState {
	fn into(self) -> WINDOW_STYLE {
		match self {
			Self::Max => WS_MAXIMIZE, 
			Self::Min => WS_MINIMIZE, 
			Self::None => WINDOW_STYLE(0u32)
		}
	}
}
#[derive(Clone, PartialEq, Default)]
pub enum WindowContextBarButton {
	NoButton, //NULL
	Minimize, //WS_MINIMIZEBOX
	Maximize, //WS_MAXIMIZEBOX
	#[default]
	MinimizeAndMaximize, //WS_MINIMIZEBOX | WS_MAXIMIZEBOX
	Help, //WS_EX_CONTEXTHELP
}
impl From<(WINDOW_STYLE, WINDOW_EX_STYLE)> for WindowContextBarButton {
	fn from(rstyle: (WINDOW_STYLE, WINDOW_EX_STYLE)) -> Self {
		use WindowContextBarButton::*;
		let (ms_style, ms_style_ex) = rstyle;  
		if ms_style_ex.contains(WS_EX_CONTEXTHELP) {
			Help
		} else if ms_style.contains(WS_MINIMIZEBOX | WS_MAXIMIZEBOX){
			MinimizeAndMaximize
		} else if ms_style.contains(WS_MAXIMIZEBOX) {
			Maximize
		} else if ms_style.contains(WS_MINIMIZEBOX) {
			Minimize
		} else {
			NoButton
		}
	}
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE)> for WindowContextBarButton {
	fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
		use WindowContextBarButton::*;
		match self {
			NoButton => (WINDOW_STYLE(0u32), WINDOW_EX_STYLE(0u32)), 
			Minimize => (WS_MINIMIZEBOX, WINDOW_EX_STYLE(0u32)), 
			Maximize => (WS_MAXIMIZEBOX, WINDOW_EX_STYLE(0u32)), 
			MinimizeAndMaximize => (WS_MINIMIZEBOX | WS_MAXIMIZEBOX, WINDOW_EX_STYLE(0u32)), 
			Help => (WINDOW_STYLE(0u32), WS_EX_CONTEXTHELP), 
		}
	}
}
#[derive(Clone, PartialEq)]
pub enum WindowBorderType {
	NoBorder,  //NULL
	ThinLine, //WS_BORDER
	Caption,  //WS_CAPTION
	SystemMenu(WindowContextBarButton), //WS_SYSMENU | WS_CAPTION
}
impl Default for WindowBorderType{
	fn default() -> Self {
		Self::SystemMenu(Default::default())
	}
}
impl From<(WINDOW_STYLE, WINDOW_EX_STYLE)> for WindowBorderType {
	fn from(rstyle: (WINDOW_STYLE, WINDOW_EX_STYLE)) -> Self {
		let (ms_style, ms_style_ex) = rstyle;
		use WindowBorderType::*;
		if ms_style.contains(WS_SYSMENU) {
			SystemMenu((ms_style, ms_style_ex).into())
		} else if ms_style.contains(WS_CAPTION) {
			Caption
		} else if ms_style.contains(WS_BORDER) {
			ThinLine
		} else {
			NoBorder
		}
	}
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE)> for WindowBorderType {
	fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
		use WindowBorderType::*;
		match self {
			NoBorder => (WINDOW_STYLE(0u32), WINDOW_EX_STYLE(0u32)), 
			ThinLine => (WS_BORDER, WINDOW_EX_STYLE(0u32)), 
			Caption => (WS_CAPTION, WINDOW_EX_STYLE(0u32)), 
			SystemMenu(x) => {let (z, y) = x.into();(WS_SYSMENU | WS_CAPTION | z, y)}
		}
	}
}
#[derive(Clone, PartialEq)]
pub struct NormalWindowStyles {
	//https://learn.microsoft.com/zh-cn/windows/win32/winmsg/window-styles
	pub visble: bool,          //WS_VISIBLE
	pub disabled: bool,        //WS_DISABLED
	pub vertical_roll: bool,   //WS_VSCROLL
	pub horizontal_roll: bool, //WS_HSCROLL
	pub size_box: bool,        //WS_SIZEBOX 或 WS_THICKFRAME
	pub dlg_frame: bool, //WS_DLGFRAME
	pub clip_children: bool, //WS_CLIPCHILDREN
	pub size_state: WindowSizeState,
	pub border_type: WindowBorderType,
}
impl Default for NormalWindowStyles {
    fn default() -> Self {
    	Self {
			visble: false,
			disabled: false,
			vertical_roll: false,
			horizontal_roll: false,
			size_box: false,
			dlg_frame: false,
			clip_children: false,
			size_state: WindowSizeState::None,
			border_type: WindowBorderType::SystemMenu(WindowContextBarButton::MinimizeAndMaximize),
    	}
	}
}
impl From<(WINDOW_STYLE, WINDOW_EX_STYLE)> for NormalWindowStyles {
	fn from(rstyle: (WINDOW_STYLE, WINDOW_EX_STYLE)) -> Self {
		let (ms_style, ex) = rstyle;
		Self {
			visble: ms_style.contains(WS_VISIBLE),
			disabled: ms_style.contains(WS_DISABLED),
			vertical_roll: ms_style.contains(WS_VSCROLL),
			horizontal_roll: ms_style.contains(WS_HSCROLL),
			size_box: ms_style.contains(WS_SIZEBOX),
			dlg_frame: ms_style.contains(WS_DLGFRAME),
			clip_children: ms_style.contains(WS_CLIPCHILDREN),
			size_state: ms_style.into(),
			border_type: (ms_style, ex).into(),
		}
	}
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE)> for NormalWindowStyles {
	fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
		let mut ms_style = WINDOW_STYLE(0u32);
		if self.visble {ms_style |=  WS_VISIBLE;};
		if self.disabled {ms_style |= WS_DISABLED;};
		if self.vertical_roll {ms_style |= WS_VSCROLL;};
		if self.horizontal_roll {ms_style |= WS_HSCROLL;};
		if self.size_box {ms_style |= WS_SIZEBOX;};
		if self.dlg_frame {ms_style |= WS_DLGFRAME;};
		if self.clip_children {ms_style |= WS_CLIPCHILDREN;};
		ms_style |= self.size_state.into();
		let (b, a) = self.border_type.into();
		ms_style |= b;
		(ms_style, a)
	}
}
#[derive(Clone, PartialEq)]
pub struct ChildWindowStyles {
	//https://learn.microsoft.com/zh-cn/windows/win32/winmsg/window-styles
	pub visble: bool,          //WS_VISIBLE
	pub disabled: bool,        //WS_DISABLED
	pub vertical_roll: bool,   //WS_VSCROLL
	pub horizontal_roll: bool, //WS_HSCROLL
	pub size_box: bool,        //WS_SIZEBOX 或 WS_THICKFRAME
	pub tab_stop: bool,        //WS_TABSTOP
	pub dlg_frame: bool,      //WS_DLGFRAME
	pub clip_isbling: bool,   //WS_CLIPSIBLINGS
	pub clip_children: bool, //WS_CLIPCHILDREN
	pub size_state: WindowSizeState,
	pub border_type: WindowBorderType,
}
impl ChildWindowStyles {
    pub fn null() -> Self {
    	Self {
			visble: false,
			disabled: false,
			vertical_roll: false,
			horizontal_roll: false,
			size_box: false,
			tab_stop: false,
			dlg_frame: false,
			clip_isbling: false, 
			clip_children: false,
			size_state: WindowSizeState::None,
			border_type: WindowBorderType::NoBorder,
    	}
    }
}
impl Default for ChildWindowStyles {
    fn default() -> Self {
    	Self {
			visble: true,
			disabled: false,
			vertical_roll: false,
			horizontal_roll: false,
			size_box: false,
			tab_stop: true,
			dlg_frame: false,
			clip_isbling: false, 
			clip_children: false,
			size_state: WindowSizeState::None,
			border_type: WindowBorderType::NoBorder,
    	}
    }
}
impl From<(WINDOW_STYLE, WINDOW_EX_STYLE)> for ChildWindowStyles {
	fn from(rstyle: (WINDOW_STYLE, WINDOW_EX_STYLE)) -> Self {
		let (ms_style, ex) = rstyle;
		Self {
			visble: ms_style.contains(WS_VISIBLE),
			disabled: ms_style.contains(WS_DISABLED),
			vertical_roll: ms_style.contains(WS_VSCROLL),
			horizontal_roll: ms_style.contains(WS_HSCROLL),
			size_box: ms_style.contains(WS_THICKFRAME),
			tab_stop: ms_style.contains(WS_TABSTOP),
			dlg_frame: ms_style.contains(WS_DLGFRAME),
			clip_isbling: ms_style.contains(WS_CLIPSIBLINGS),
			clip_children: ms_style.contains(WS_CLIPCHILDREN),
			size_state: ms_style.into(),
			border_type: (ms_style, ex).into(),
		}
	}
}
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE)> for ChildWindowStyles {
	fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
		let mut ms_style = WINDOW_STYLE(0u32);
		if self.visble {ms_style |= WS_VISIBLE;};
		if self.disabled {ms_style |= WS_DISABLED;};
		if self.vertical_roll {ms_style |= WS_VSCROLL;};
		if self.horizontal_roll {ms_style |= WS_HSCROLL;};
		if self.size_box {ms_style |= WS_THICKFRAME;};
		if self.tab_stop {ms_style |= WS_TABSTOP;};
		if self.dlg_frame {ms_style |= WS_DLGFRAME;};
		if self.clip_isbling {ms_style |= WS_CLIPSIBLINGS;};
		if self.clip_children {ms_style |= WS_CLIPCHILDREN;};
		ms_style |= self.size_state.into();
		let (b, a) = self.border_type.into();
		ms_style |= b;
		(ms_style, a)
	}
}
#[derive(Clone, PartialEq, Default)]
pub struct NormalWindowExStyles {
	//https://learn.microsoft.com/zh-cn/windows/win32/winmsg/extended-window-styles
	edge: bool,                  //WS_EX_WINDOWEDGE
	transparent: bool,           //WS_EX_TRANSPARENT
	top_most: bool,              //WS_EX_TOPMOST
	tool_window: bool,           //WS_EX_TOOLWINDOW
	static_edge: bool,           //WS_EX_STATICEDGE
	right_reading: bool,         //WS_EX_RTLREADING
	right: bool,                 //WS_EX_RIGHT
	no_redirection_bitmap: bool, //WS_EX_NOREDIRECTIONBITMAP
	no_inherit_layout: bool,     //WS_EX_NOINHERITLAYOUT
	no_auto_active: bool,        //WS_EX_NOACTIVATE
	left_scrroll_bar: bool,      //WS_EX_LEFTSCROLLBAR
	right_layout: bool,          //WS_EX_LAYOUTRTL
	accept_files: bool,          //WS_EX_ACCEPTFILES
	app_window: bool,            //WS_EX_APPWINDOW
	clint_edge: bool,            //WS_EX_CLIENTEDGE
	dlg_modal_frame: bool, //WS_EX_DLGMODALFRAME
	com_posited: bool,     //WS_EX_COMPOSITED
	//WS_EX_CONTROLPARENT is in style
}
impl From<WINDOW_EX_STYLE> for NormalWindowExStyles {
	fn from(ms_style: WINDOW_EX_STYLE) -> Self {
		Self {
			edge: ms_style.contains(WS_EX_WINDOWEDGE),
			transparent: ms_style.contains(WS_EX_TRANSPARENT),
			top_most: ms_style.contains(WS_EX_TOPMOST),
			tool_window: ms_style.contains(WS_EX_TOOLWINDOW),
			static_edge: ms_style.contains(WS_EX_STATICEDGE),
			right_reading: ms_style.contains(WS_EX_RTLREADING),
			right: ms_style.contains(WS_EX_RIGHT),
			no_redirection_bitmap: ms_style.contains(WS_EX_NOREDIRECTIONBITMAP),
			no_inherit_layout: ms_style.contains(WS_EX_NOINHERITLAYOUT),
			no_auto_active: ms_style.contains(WS_EX_NOACTIVATE),
			left_scrroll_bar: ms_style.contains(WS_EX_LEFTSCROLLBAR),
			right_layout: ms_style.contains(WS_EX_LAYOUTRTL),
			accept_files: ms_style.contains(WS_EX_ACCEPTFILES),
			app_window: ms_style.contains(WS_EX_APPWINDOW),
			clint_edge: ms_style.contains(WS_EX_CLIENTEDGE),
			dlg_modal_frame: ms_style.contains(WS_EX_DLGMODALFRAME),
			com_posited: ms_style.contains(WS_EX_COMPOSITED),
		}
	}
}
impl Into<WINDOW_EX_STYLE> for NormalWindowExStyles {
	fn into(self) -> WINDOW_EX_STYLE {
		let mut ms_style = WINDOW_EX_STYLE(0u32);
		if self.edge {ms_style |= WS_EX_WINDOWEDGE;};
		if self.transparent {ms_style |= WS_EX_TRANSPARENT;};
		if self.top_most {ms_style |= WS_EX_TOPMOST;};
		if self.tool_window {ms_style |= WS_EX_TOOLWINDOW;};
		if self.static_edge {ms_style |= WS_EX_STATICEDGE;};
		if self.right_reading {ms_style |= WS_EX_RTLREADING;};
		if self.right {ms_style |= WS_EX_RIGHT;};
		if self.no_redirection_bitmap {ms_style |= WS_EX_NOREDIRECTIONBITMAP;};
		if self.no_inherit_layout {ms_style |= WS_EX_NOINHERITLAYOUT;};
		if self.no_auto_active {ms_style |= WS_EX_NOACTIVATE;};
		if self.left_scrroll_bar {ms_style |= WS_EX_LEFTSCROLLBAR;};
		if self.right_layout {ms_style |= WS_EX_LAYOUTRTL;};
		if self.accept_files {ms_style |= WS_EX_ACCEPTFILES;};
		if self.app_window {ms_style |= WS_EX_APPWINDOW;};
		if self.clint_edge {ms_style |= WS_EX_CLIENTEDGE;};
		if self.dlg_modal_frame {ms_style |= WS_EX_DLGMODALFRAME;};
		if self.com_posited {ms_style |= WS_EX_COMPOSITED;};
		ms_style
	}
}
#[derive(Clone, PartialEq)]
pub enum WindowType {
    Overlapped{
        style: NormalWindowStyles,
        syle_ex: NormalWindowExStyles,
        menu: Option<Menu>, 
        onwer: Option<Window>, 
        is_layered: bool, //WS_EX_LAYERED
    }, //重叠窗口
    Popup{
        style: NormalWindowStyles,
        syle_ex: NormalWindowExStyles,
        menu: Option<Menu>, 
        onwer:Option<Window>, 
        is_layered: bool, //WS_EX_LAYERED
    },
    Child{
        style: ChildWindowStyles,
        syle_ex: NormalWindowExStyles,
        identifier: WindowID, 
        parent:Window, 
        no_send_notify_to_parent: bool,//WS_EX_NOPARENTNOTIFY
        is_layered: bool, //WS_EX_LAYERED
    },
    MessageOnly,
}
impl Default for WindowType {
    fn default() -> Self {
        Self::Overlapped{style: Default::default(), syle_ex: Default::default(), menu: None, onwer: None, is_layered: false}
    }
}//WS_OVERLAPPEDWINDOW
impl Into<(WINDOW_STYLE, WINDOW_EX_STYLE, Option<HMENU>, Option<HWND>)> for WindowType {
    fn into(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE, Option<HMENU>, Option<HWND>) {
        use WindowType::*;
        match self {
            Overlapped{style: x, syle_ex: mut syle_exx, menu: z, onwer: w, is_layered: b} => {
            	let mut xx: WINDOW_EX_STYLE = syle_exx.into();
            	let (yy, zz) = x.into();
				(yy, xx | zz, match z {Some(x) => Some(x.handle), None => None, }, match w {Some(x) => Some(x.handle), None => None, })
            }, 
            Popup{style: x, syle_ex: mut ms_style, menu: z, onwer: w, is_layered: b} => {
            	let mut xx: WINDOW_EX_STYLE = ms_style.into();
            	let (yy, zz) = x.into();
				if b {
					xx |= WS_EX_LAYERED;
				};
				(yy| WS_POPUP, xx | zz, match z {Some(x) => Some(x.handle), None => None, }, match w {Some(x) => Some(x.handle), None => None, })
            },
            Child{style: x, syle_ex: mut ms_style, identifier: z, parent: w, is_layered: b, no_send_notify_to_parent:c} => {
				let mut xx: WINDOW_EX_STYLE = ms_style.into();
            	let (yy, zz) = x.into();
				if b {
					xx |= WS_EX_LAYERED;
				};
				if c {
					xx |= WS_EX_NOPARENTNOTIFY;
				};
				(yy | WS_CHILD, xx| zz, Some(HMENU(z as *mut c_void)), Some(w.handle))
            },
            MessageOnly => (WINDOW_STYLE(0), WINDOW_EX_STYLE(0), None, Some(HWND_MESSAGE))
        }
    }
}
impl From<(WINDOW_STYLE, WINDOW_EX_STYLE, Option<HMENU>, Option<HWND>)> for WindowType {
    fn from(data: (WINDOW_STYLE, WINDOW_EX_STYLE, Option<HMENU>, Option<HWND>)) -> Self {
    	use WindowType::*;
    	let (style, style_ex, menu, w) = data;
    	if w == Some(HWND_MESSAGE) {
    		MessageOnly
    	} else if style.contains(WS_CHILD) && w.is_some(){
			Child{
				style: (style, style_ex).into(), 
				identifier: menu.unwrap_or_default().0 as u16, 
				parent: Window { handle:w.unwrap_or_default()}, 
				is_layered: style_ex.contains(WS_EX_LAYERED), 
				no_send_notify_to_parent:style_ex.contains(WS_EX_NOPARENTNOTIFY), 
				syle_ex: style_ex.into(), 
			}
		} else if style.contains(WS_POPUP) {
			let w = match w { Some(x) if !x.is_invalid() => Some(Window {handle:x}), _ => None, };
			let menu = match menu { Some(x) if !x.is_invalid() => Some(Menu {handle:x, mtype:MenuType::Menu}), _ => None, };
			Popup{
				style: (style, style_ex).into(), 
				menu: menu, 
				onwer: w, 
				is_layered: style_ex.contains(WS_EX_LAYERED), 
				syle_ex: style_ex.into(), 
			}
		} else {
			let w = match w { Some(x) if !x.is_invalid() => Some(Window {handle:x}), _ => None, };
			let menu = match menu { Some(x) if !x.is_invalid() => Some(Menu {handle:x, mtype:MenuType::Menu}), _ => None, };
			Overlapped{
				style: (style, style_ex).into(), 
				menu: menu, 
				onwer: w, 
				is_layered: style_ex.contains(WS_EX_LAYERED), 
				syle_ex: style_ex.into(), 
			}
		}
    }
}