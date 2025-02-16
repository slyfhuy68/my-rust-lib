use super::*;
use super::button::*;
#[derive(Clone)]
pub struct RadioButton (HWND);//PUSHBUTTON
unsafe impl Send for RadioButton {}
unsafe impl Sync for RadioButton {}
pub struct RadioButtonStyle{
	pub extra_msg:bool, //BS_NOTIFY
	pub auto:bool, //if light BS_AUTORADIOBUTTON else BS_RADIOBUTTON
	pub flat:bool, //BS_FLAT
	pub like_button:bool,//BS_PUSHLIKE
	pub left_text:bool,//BS_LEFTTEXT
}
impl Into<WINDOW_STYLE> for RadioButtonStyle {
	fn into(self) -> WINDOW_STYLE {
		let mut ms_style = WINDOW_STYLE(0u32);
		if self.extra_msg {ms_style |= WINDOW_STYLE(BS_NOTIFY as u32);};
		if self.flat {ms_style |=  WINDOW_STYLE(BS_FLAT as u32);};
		if self.auto {ms_style |=  WINDOW_STYLE(BS_AUTORADIOBUTTON as u32);} else {ms_style |=  WINDOW_STYLE(BS_RADIOBUTTON as u32);};
		if self.like_button {ms_style |=  WINDOW_STYLE(BS_PUSHLIKE as u32);};
		if self.left_text {ms_style |=  WINDOW_STYLE(BS_LEFTTEXT as u32);};
		
		ms_style
	}
}
impl Default for RadioButtonStyle {
	fn default() -> Self {
		Self {
			extra_msg:false,
			auto:true,
			flat:true, 
			like_button:false,
			left_text:false,
		} 
	} 
}
pub struct RadioButtonMsg{
	hwnd:HWND, 
	pub bm_type:ButtonMsgType, 
}
impl Control for RadioButton{
	type MsgType = RadioButtonMsg;
	fn from_window(wnd:Window) -> Self{
		Self(wnd.handle)
	}
	fn to_window(self) -> Window {
		Window{handle:self.0}
	}
}
impl ControlMsg for RadioButtonMsg{ 
	type ControlType = RadioButton;
	unsafe fn from_msg(ptr:usize) -> Option<Box<Self>>{
		let nmhdr = *(ptr as *mut NMHDR);
		let code = nmhdr.code;
		let w = nmhdr.hwndFrom.clone();
		let _ = nmhdr;
		use ButtonMsgType::*;
		let bmtype = match code {
			BCN_HOTITEMCHANGE => {
				let data = *(ptr as *mut NMBCHOTITEM);
				if data.dwFlags == HICF_MOUSE | HICF_ENTERING {
					MouseEntering
				} else if data.dwFlags == HICF_MOUSE | HICF_LEAVING {
					MouseLaveing
				} else {
					return None;
				}
			}, 
			BN_CLICKED => {
				Clicked
			}, 
			BN_DBLCLK => {
				DoubleClicked
			}, 
			BN_KILLFOCUS => {
				LoseKeyboardFocus
			}, 
			BN_SETFOCUS => {
				GetKeyboardFocus
			}, 
			NM_CUSTOMDRAW => {
				Draw(ptr)
			}
			_ => return None, 
		};
		Some(Box::new(Self {
			hwnd:w, 
			bm_type:bmtype
		}))
	}
	fn get_control(&self) -> Self::ControlType{
		RadioButton(self.hwnd)
	}
}
pub enum RadioButtonDrawType {
	ParentDraw, //BS_OWNERDRAW
	AutoDraw(ButtonAutoDrawType, RadioButtonStyle), //NULL
}
impl Default for RadioButtonDrawType {
	fn default() -> Self {
		Self::AutoDraw(ButtonAutoDrawType::TextOnly(false), Default::default())
	} 
}
impl Into<(WINDOW_STYLE, Option<BitmapOrIcon>)> for RadioButtonDrawType {
	fn into(self) -> (WINDOW_STYLE, Option<BitmapOrIcon>) {
		match self {
			RadioButtonDrawType::ParentDraw => (WINDOW_STYLE(BS_OWNERDRAW as u32), None), 
			RadioButtonDrawType::AutoDraw(dtype, bstyle) => {
				let mut wstyle = WINDOW_STYLE(0);
				let ditype = match dtype {
					ButtonAutoDrawType::IconOnly(boi) => Some(boi), 
					ButtonAutoDrawType::TextOnly(a) => {if a {wstyle |= WINDOW_STYLE(BS_MULTILINE as u32);}; None}, 
					ButtonAutoDrawType::IconAndText(boi, a) => {
						if a {wstyle |= WINDOW_STYLE(BS_MULTILINE as u32)};
						Some(boi)
					}
				};
				wstyle |= bstyle.into();
				(wstyle, ditype)
			}
		}
	}
}
// impl From(WINDOW_STYLE, Option<BitmapOrIcon>) for RadioButtonDrawType {
// 	fn from(data: (WINDOW_STYLE, Option<BitmapOrIcon>)) -> Self {
// 		
// 	}
// }
impl RadioButton {
	fn is_radio_button(&self) -> bool {
        let style = WINDOW_STYLE(unsafe {
            GetWindowLongW(self.0, GWL_STYLE) as u32
        });
        style.contains(WINDOW_STYLE(BS_AUTORADIOBUTTON as u32)) | style.contains(WINDOW_STYLE(BS_RADIOBUTTON as u32))
	}
	pub fn new(wnd:&mut Window,name:&str, 
		pos: Option<RectangleWH>, 
		identifier: WindowID, 
		control_style:RadioButtonDrawType, 
		style:ChildWindowStyles, 
		style_ex: NormalWindowExStyles, 
		font:bool, no_notify: bool) -> Result<Self> {
			let (control_style_ms, draw) = control_style.into();
			let hwnd = new_button(wnd, name, pos, identifier, style, style_ex, control_style_ms, font, no_notify, draw)?;
			Ok(RadioButton(hwnd))
		}
	pub fn is_checked(&self) -> Result<bool> {
		if !self.is_radio_button() {
			return Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), ""));
		}
		let result = unsafe {
			SendMessageW(self.0, BM_GETCHECK, Some(WPARAM(0)), Some(LPARAM(0))).0
		};
		match DLG_BUTTON_CHECK_STATE(match result.try_into() { Ok(x)=> x,Err(_) => return Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), ""))}) {
			BST_CHECKED => Ok(true),
			BST_UNCHECKED => Ok(false),
			BST_INDETERMINATE => Err(Error::new(ERROR_NOT_SUPPORTED.to_hresult(), "")),
			_ => return Err(Error::from_win32()),
		}
	}
}