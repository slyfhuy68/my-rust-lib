use super::*;
use std::sync::mpsc::*;
use std::thread::*;
use windows::Win32::UI::WindowsAndMessaging::*;
pub struct PropCounter {
    sender: Option<Sender<bool>>,
    thread: Option<JoinHandle<windows::core::Result<()>>>,
    result_receiver: Receiver<Box<(std::result::Result<String, FromUtf16Error>, usize)>>, //wHANDLE)>>
}
impl Iterator for PropCounter {
    type Item = (String, usize);
    fn next(&mut self) -> Option<Self::Item> {
        match &self.sender {
            Some(x) => x.send(true).ok()?,
            None => return None,
        }
        let result = self.result_receiver.recv().ok()?;
        Some(unsafe { ((result.0).ok()?, result.1) })
    }
}
impl Window {
    pub fn prop_iter(&self) -> PropCounter {
        let (send_s, recv) = channel();
        let self_handle = self.handle.0 as usize;
        let (result_sender, result_receiver) = channel();
        let join = std::thread::spawn(move || {
            let boxed_recv = Box::new((recv, result_sender));
            let receiver_ptr: *mut (
                Receiver<bool>,
                Sender<Box<(std::result::Result<String, FromUtf16Error>, usize)>>,
            ) = Box::into_raw(boxed_recv);
            let receiver_usize: usize = receiver_ptr as usize;
            unsafe extern "system" fn callback(
                hwnd: HWND,
                pcwstr: PCWSTR,
                handle: wHANDLE,
                ptr: usize,
            ) -> BOOL { unsafe {
                let box_data = Box::from_raw(
                    ptr as *mut (
                        Receiver<bool>,
                        Sender<Box<(std::result::Result<String, FromUtf16Error>, usize)>>,
                    ),
                );
                match ((*box_data).0).recv() {
                    Ok(x) => {
                        if x {
                            let boxed_result = Box::new((pcwstr.to_string(), handle.0 as usize));
                            ((*box_data).1).send(boxed_result);
                            Box::into_raw(box_data); //防止rust释放
                            return BOOL(1);
                        } else {
                            drop(box_data);
                            return BOOL(0);
                        }
                    }
                    Err(y) => {
                        drop(box_data);
                        return BOOL(0);
                    }
                }
            }}
            match unsafe {
                EnumPropsExW(
                    HWND(self_handle as *mut c_void),
                    Some(callback),
                    LPARAM(receiver_usize as isize),
                )
            } {
                -1 => Err(Error::empty()),
                _ => Ok(()),
            }
        });
        PropCounter {
            sender: Some(send_s),
            thread: Some(join),
            result_receiver: result_receiver,
        }
    }
    pub fn get_prop(&self, key: &str) -> windows::core::Result<usize> {
        let (name, vecname) = str_to_pcwstr(key);
        match unsafe { GetPropW(self.handle, name).0 } as usize {
            0 => Err(Error::new(HRESULT(ERROR_NOT_FOUND.0 as i32), "")),
            x => Ok(x),
        }
    }
    pub fn set_prop(&mut self, key: &str, value: usize) -> windows::core::Result<()> {
        let (name, vecname) = str_to_pcwstr(key);
        match value {
            0 => Err(Error::new(HRESULT(ERROR_INVALID_PARAMETER.0 as i32), "")),
            x => unsafe { SetPropW(self.handle, name, Some(HANDLE(x as *mut c_void))) },
        }
    }
    pub fn remove_prop(&mut self, key: &str, value: usize) -> windows::core::Result<usize> {
        let (name, vecname) = str_to_pcwstr(key);
        match unsafe { RemovePropW(self.handle, name)?.0 } as usize {
            0 => Err(Error::new(HRESULT(ERROR_NOT_FOUND.0 as i32), "")),
            x => Ok(x),
        }
    }
}
impl Drop for PropCounter {
    fn drop(&mut self) {
        self.sender.take();
        if let Some(x) = self.thread.take() {
            x.join();
        };
    }
}
