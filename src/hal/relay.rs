use ::raw::*;
use hal::handle::*;
use ::error::*;

pub fn hal_initialize_relay_port(port_handle: PortHandle, fwd: bool) -> HalResult<RelayHandle> {
    hal_call![ ptr HAL_InitializeRelayPort(port_handle.get_handle(), fwd as HAL_Bool) ]
        .map(|n| RelayHandle(n))
}

pub fn free_relay_port(handle: RelayHandle) {
    unsafe { HAL_FreeRelayPort(handle.get_handle()) }
}

pub fn check_relay_channel(channel: i32) -> bool {
    unsafe { HAL_CheckRelayChannel(channel) != 0 }
}

pub fn set_relay(handle: RelayHandle, on: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetRelay(handle.get_handle(), on as HAL_Bool) ]
}

pub fn get_relay(handle: RelayHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetRelay(handle.get_handle()) ].map(|n| n != 0)
}