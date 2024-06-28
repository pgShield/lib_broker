use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use crate::{Broker, Config, BrokerError, Message};
use std::sync::Arc;
use tokio::runtime::Runtime;

struct FfiMessage(String);
impl Message for FfiMessage {}

type MessageHandlerCallback = extern "C" fn(*const c_char) -> c_int;


#[no_mangle]
pub extern "C" fn lib_broker_init(config_path: *const c_char) -> *mut libc::c_void {
    let config_path = unsafe {
        assert!(!config_path.is_null());
        CStr::from_ptr(config_path).to_str().unwrap()
    };

    let config = match Config::from_file(config_path) {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };

    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return std::ptr::null_mut(),
    };

    let broker = match runtime.block_on(async { Broker::new(config).await }) {
        Ok(b) => b,
        Err(_) => return std::ptr::null_mut(),
    };

    let broker_arc = Arc::new(broker);
    Box::into_raw(Box::new((broker_arc, runtime))) as *mut libc::c_void
}


#[no_mangle]
pub extern "C" fn lib_broker_send(
    broker_ptr: *mut libc::c_void,
    message: *const c_char,
) -> c_int {
    let (broker, runtime) = unsafe {
        assert!(!broker_ptr.is_null());
        &mut *(broker_ptr as *mut (Arc<Broker>, Runtime))
    };

    let message = unsafe {
        assert!(!message.is_null());
        CStr::from_ptr(message).to_str().unwrap()
    };

    let result: Result<(), BrokerError> = runtime.block_on(async {
        broker.send(FfiMessage(message.to_string())).await
    });

    match result {
        Ok(_) => 0,
        Err(_) => -1,
    }
}



// #[no_mangle]
// pub extern "C" fn lib_broker_register_handler(
//     _broker_ptr: *mut libc::c_void,
//     _message_type: *const c_char,
//     _callback: MessageHandlerCallback,
// ) -> c_int {
//     let (broker, _) = unsafe {
//         assert!(!broker_ptr.is_null());
//         &mut *(broker_ptr as *mut (Arc<Broker>, Runtime))
//     };

//     let message_type = unsafe {
//         assert!(!message_type.is_null());
//         CStr::from_ptr(message_type).to_str().unwrap()
//     };
        
//         broker.send(dynamic_message).await

//     0;
// }

#[no_mangle]
pub extern "C" fn lib_broker_free(broker_ptr: *mut libc::c_void) {
    if !broker_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(broker_ptr as *mut (Arc<Broker>, Runtime));
        }
    }
}