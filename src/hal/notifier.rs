use ::raw::*;
use hal::handle::*;
use ::error::*;

// pub type HAL_NotifierProcessFunction = ::std::option::Option<unsafe extern "C" fn(currentTime: u64, handle: HAL_NotifierHandle)>;

// extern "C" fn notifier_cb<F>(time: u64, handle: HAL_NotifierHandle)
//     where F: Fn(u64, HAL_NotifierHandle)
// {
//     let opt_closure = closure as *mut Option<F>;
// }
//
// pub fn initialize_notifier(process: HAL_NotifierProcessFunction,
//                            param: *mut ::std::os::raw::c_void)
//                            -> HalResult<NotifierHandle> {
//     hal_call![ ptr HAL_InitializeNotifier() ]
// }
//
// pub fn initialize_notifier_threaded(process: HAL_NotifierProcessFunction,
//                                     param: *mut ::std::os::raw::c_void)
//                                     -> HalResult<NotifierHandle> {
//     hal_call![ ptr HAL_InitializeNotifierThreaded() ]
// }

pub fn clean_notifier(handle: NotifierHandle) -> HalResult<()> {
    hal_call![ ptr HAL_CleanNotifier(handle.get_handle()) ]
}

// Oh fuck
pub fn get_notifier_param(handle: NotifierHandle) -> HalResult<*mut ::std::os::raw::c_void> {
    hal_call![ ptr HAL_GetNotifierParam(handle.get_handle()) ]
}

pub fn update_notifier_alarm(handle: NotifierHandle, trigger_time: u64) -> HalResult<()> {
    hal_call![ ptr HAL_UpdateNotifierAlarm(handle.get_handle(), trigger_time) ]
}

pub fn stop_notifier_alarm(handle: NotifierHandle) -> HalResult<()> {
    hal_call![ ptr HAL_StopNotifierAlarm(handle.get_handle()) ]
}