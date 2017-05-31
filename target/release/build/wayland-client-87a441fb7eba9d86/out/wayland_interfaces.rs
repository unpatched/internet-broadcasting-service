//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2008-2011 Kristian Høgsberg
    Copyright © 2010-2011 Intel Corporation
    Copyright © 2012-2013 Collabora, Ltd.

    Permission is hereby granted, free of charge, to any person
    obtaining a copy of this software and associated documentation files
    (the "Software"), to deal in the Software without restriction,
    including without limitation the rights to use, copy, modify, merge,
    publish, distribute, sublicense, and/or sell copies of the Software,
    and to permit persons to whom the Software is furnished to do so,
    subject to the following conditions:

    The above copyright notice and this permission notice (including the
    next paragraph) shall be included in all copies or substantial
    portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
    BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
    ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
    CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

use wayland_sys::common::*;

use std::os::raw::{c_void, c_char};

const NULLPTR : *const c_void = 0 as *const c_void;

static mut types_null: [*const wl_interface; 8] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];

// wl_display

static mut wl_display_requests_sync_types: [*const wl_interface; 1] = [
    unsafe { &wl_callback_interface as *const wl_interface },
];
static mut wl_display_requests_get_registry_types: [*const wl_interface; 1] = [
    unsafe { &wl_registry_interface as *const wl_interface },
];
pub static mut wl_display_requests: [wl_message; 2] = [
    wl_message { name: b"sync\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &wl_display_requests_sync_types as *const _ } },
    wl_message { name: b"get_registry\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &wl_display_requests_get_registry_types as *const _ } },
];
pub static mut wl_display_events: [wl_message; 2] = [
    wl_message { name: b"error\0" as *const u8 as *const c_char, signature: b"ous\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"delete_id\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_display_interface: wl_interface = wl_interface {
    name: b"wl_display\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &wl_display_requests as *const _ },
    event_count: 2,
    events: unsafe { &wl_display_events as *const _ },
};

// wl_registry

pub static mut wl_registry_requests: [wl_message; 1] = [
    wl_message { name: b"bind\0" as *const u8 as *const c_char, signature: b"usun\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut wl_registry_events: [wl_message; 2] = [
    wl_message { name: b"global\0" as *const u8 as *const c_char, signature: b"usu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"global_remove\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_registry_interface: wl_interface = wl_interface {
    name: b"wl_registry\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &wl_registry_requests as *const _ },
    event_count: 2,
    events: unsafe { &wl_registry_events as *const _ },
};

// wl_callback

pub static mut wl_callback_events: [wl_message; 1] = [
    wl_message { name: b"done\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_callback_interface: wl_interface = wl_interface {
    name: b"wl_callback\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 0,
    requests: NULLPTR as *const wl_message,
    event_count: 1,
    events: unsafe { &wl_callback_events as *const _ },
};

// wl_compositor

static mut wl_compositor_requests_create_surface_types: [*const wl_interface; 1] = [
    unsafe { &wl_surface_interface as *const wl_interface },
];
static mut wl_compositor_requests_create_region_types: [*const wl_interface; 1] = [
    unsafe { &wl_region_interface as *const wl_interface },
];
pub static mut wl_compositor_requests: [wl_message; 2] = [
    wl_message { name: b"create_surface\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &wl_compositor_requests_create_surface_types as *const _ } },
    wl_message { name: b"create_region\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &wl_compositor_requests_create_region_types as *const _ } },
];

pub static mut wl_compositor_interface: wl_interface = wl_interface {
    name: b"wl_compositor\0" as *const u8  as *const c_char,
    version: 4,
    request_count: 2,
    requests: unsafe { &wl_compositor_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// wl_shm_pool

static mut wl_shm_pool_requests_create_buffer_types: [*const wl_interface; 6] = [
    unsafe { &wl_buffer_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut wl_shm_pool_requests: [wl_message; 3] = [
    wl_message { name: b"create_buffer\0" as *const u8 as *const c_char, signature: b"niiiiu\0" as *const u8 as *const c_char, types: unsafe { &wl_shm_pool_requests_create_buffer_types as *const _ } },
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"resize\0" as *const u8 as *const c_char, signature: b"i\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_shm_pool_interface: wl_interface = wl_interface {
    name: b"wl_shm_pool\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 3,
    requests: unsafe { &wl_shm_pool_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// wl_shm

static mut wl_shm_requests_create_pool_types: [*const wl_interface; 3] = [
    unsafe { &wl_shm_pool_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut wl_shm_requests: [wl_message; 1] = [
    wl_message { name: b"create_pool\0" as *const u8 as *const c_char, signature: b"nhi\0" as *const u8 as *const c_char, types: unsafe { &wl_shm_requests_create_pool_types as *const _ } },
];
pub static mut wl_shm_events: [wl_message; 1] = [
    wl_message { name: b"format\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_shm_interface: wl_interface = wl_interface {
    name: b"wl_shm\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &wl_shm_requests as *const _ },
    event_count: 1,
    events: unsafe { &wl_shm_events as *const _ },
};

// wl_buffer

pub static mut wl_buffer_requests: [wl_message; 1] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut wl_buffer_events: [wl_message; 1] = [
    wl_message { name: b"release\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_buffer_interface: wl_interface = wl_interface {
    name: b"wl_buffer\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &wl_buffer_requests as *const _ },
    event_count: 1,
    events: unsafe { &wl_buffer_events as *const _ },
};

// wl_data_offer

pub static mut wl_data_offer_requests: [wl_message; 5] = [
    wl_message { name: b"accept\0" as *const u8 as *const c_char, signature: b"u?s\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"receive\0" as *const u8 as *const c_char, signature: b"sh\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"finish\0" as *const u8 as *const c_char, signature: b"3\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_actions\0" as *const u8 as *const c_char, signature: b"3uu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut wl_data_offer_events: [wl_message; 3] = [
    wl_message { name: b"offer\0" as *const u8 as *const c_char, signature: b"s\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"source_actions\0" as *const u8 as *const c_char, signature: b"3u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"action\0" as *const u8 as *const c_char, signature: b"3u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_data_offer_interface: wl_interface = wl_interface {
    name: b"wl_data_offer\0" as *const u8  as *const c_char,
    version: 3,
    request_count: 5,
    requests: unsafe { &wl_data_offer_requests as *const _ },
    event_count: 3,
    events: unsafe { &wl_data_offer_events as *const _ },
};

// wl_data_source

pub static mut wl_data_source_requests: [wl_message; 3] = [
    wl_message { name: b"offer\0" as *const u8 as *const c_char, signature: b"s\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_actions\0" as *const u8 as *const c_char, signature: b"3u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut wl_data_source_events: [wl_message; 6] = [
    wl_message { name: b"target\0" as *const u8 as *const c_char, signature: b"?s\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"send\0" as *const u8 as *const c_char, signature: b"sh\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"cancelled\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"dnd_drop_performed\0" as *const u8 as *const c_char, signature: b"3\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"dnd_finished\0" as *const u8 as *const c_char, signature: b"3\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"action\0" as *const u8 as *const c_char, signature: b"3u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_data_source_interface: wl_interface = wl_interface {
    name: b"wl_data_source\0" as *const u8  as *const c_char,
    version: 3,
    request_count: 3,
    requests: unsafe { &wl_data_source_requests as *const _ },
    event_count: 6,
    events: unsafe { &wl_data_source_events as *const _ },
};

// wl_data_device

static mut wl_data_device_requests_start_drag_types: [*const wl_interface; 4] = [
    unsafe { &wl_data_source_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
static mut wl_data_device_requests_set_selection_types: [*const wl_interface; 2] = [
    unsafe { &wl_data_source_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
pub static mut wl_data_device_requests: [wl_message; 3] = [
    wl_message { name: b"start_drag\0" as *const u8 as *const c_char, signature: b"?oo?ou\0" as *const u8 as *const c_char, types: unsafe { &wl_data_device_requests_start_drag_types as *const _ } },
    wl_message { name: b"set_selection\0" as *const u8 as *const c_char, signature: b"?ou\0" as *const u8 as *const c_char, types: unsafe { &wl_data_device_requests_set_selection_types as *const _ } },
    wl_message { name: b"release\0" as *const u8 as *const c_char, signature: b"2\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
static mut wl_data_device_events_data_offer_types: [*const wl_interface; 1] = [
    unsafe { &wl_data_offer_interface as *const wl_interface },
];
static mut wl_data_device_events_enter_types: [*const wl_interface; 5] = [
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    unsafe { &wl_data_offer_interface as *const wl_interface },
];
static mut wl_data_device_events_selection_types: [*const wl_interface; 1] = [
    unsafe { &wl_data_offer_interface as *const wl_interface },
];
pub static mut wl_data_device_events: [wl_message; 6] = [
    wl_message { name: b"data_offer\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &wl_data_device_events_data_offer_types as *const _ } },
    wl_message { name: b"enter\0" as *const u8 as *const c_char, signature: b"uoff?o\0" as *const u8 as *const c_char, types: unsafe { &wl_data_device_events_enter_types as *const _ } },
    wl_message { name: b"leave\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"motion\0" as *const u8 as *const c_char, signature: b"uff\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"drop\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"selection\0" as *const u8 as *const c_char, signature: b"?o\0" as *const u8 as *const c_char, types: unsafe { &wl_data_device_events_selection_types as *const _ } },
];

pub static mut wl_data_device_interface: wl_interface = wl_interface {
    name: b"wl_data_device\0" as *const u8  as *const c_char,
    version: 3,
    request_count: 3,
    requests: unsafe { &wl_data_device_requests as *const _ },
    event_count: 6,
    events: unsafe { &wl_data_device_events as *const _ },
};

// wl_data_device_manager

static mut wl_data_device_manager_requests_create_data_source_types: [*const wl_interface; 1] = [
    unsafe { &wl_data_source_interface as *const wl_interface },
];
static mut wl_data_device_manager_requests_get_data_device_types: [*const wl_interface; 2] = [
    unsafe { &wl_data_device_interface as *const wl_interface },
    unsafe { &wl_seat_interface as *const wl_interface },
];
pub static mut wl_data_device_manager_requests: [wl_message; 2] = [
    wl_message { name: b"create_data_source\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &wl_data_device_manager_requests_create_data_source_types as *const _ } },
    wl_message { name: b"get_data_device\0" as *const u8 as *const c_char, signature: b"no\0" as *const u8 as *const c_char, types: unsafe { &wl_data_device_manager_requests_get_data_device_types as *const _ } },
];

pub static mut wl_data_device_manager_interface: wl_interface = wl_interface {
    name: b"wl_data_device_manager\0" as *const u8  as *const c_char,
    version: 3,
    request_count: 2,
    requests: unsafe { &wl_data_device_manager_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// wl_shell

static mut wl_shell_requests_get_shell_surface_types: [*const wl_interface; 2] = [
    unsafe { &wl_shell_surface_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut wl_shell_requests: [wl_message; 1] = [
    wl_message { name: b"get_shell_surface\0" as *const u8 as *const c_char, signature: b"no\0" as *const u8 as *const c_char, types: unsafe { &wl_shell_requests_get_shell_surface_types as *const _ } },
];

pub static mut wl_shell_interface: wl_interface = wl_interface {
    name: b"wl_shell\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &wl_shell_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// wl_shell_surface

static mut wl_shell_surface_requests_move_types: [*const wl_interface; 2] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
static mut wl_shell_surface_requests_resize_types: [*const wl_interface; 3] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut wl_shell_surface_requests_set_transient_types: [*const wl_interface; 4] = [
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut wl_shell_surface_requests_set_fullscreen_types: [*const wl_interface; 3] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    unsafe { &wl_output_interface as *const wl_interface },
];
static mut wl_shell_surface_requests_set_popup_types: [*const wl_interface; 6] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut wl_shell_surface_requests_set_maximized_types: [*const wl_interface; 1] = [
    unsafe { &wl_output_interface as *const wl_interface },
];
pub static mut wl_shell_surface_requests: [wl_message; 10] = [
    wl_message { name: b"pong\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"move\0" as *const u8 as *const c_char, signature: b"ou\0" as *const u8 as *const c_char, types: unsafe { &wl_shell_surface_requests_move_types as *const _ } },
    wl_message { name: b"resize\0" as *const u8 as *const c_char, signature: b"ouu\0" as *const u8 as *const c_char, types: unsafe { &wl_shell_surface_requests_resize_types as *const _ } },
    wl_message { name: b"set_toplevel\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_transient\0" as *const u8 as *const c_char, signature: b"oiiu\0" as *const u8 as *const c_char, types: unsafe { &wl_shell_surface_requests_set_transient_types as *const _ } },
    wl_message { name: b"set_fullscreen\0" as *const u8 as *const c_char, signature: b"uu?o\0" as *const u8 as *const c_char, types: unsafe { &wl_shell_surface_requests_set_fullscreen_types as *const _ } },
    wl_message { name: b"set_popup\0" as *const u8 as *const c_char, signature: b"ouoiiu\0" as *const u8 as *const c_char, types: unsafe { &wl_shell_surface_requests_set_popup_types as *const _ } },
    wl_message { name: b"set_maximized\0" as *const u8 as *const c_char, signature: b"?o\0" as *const u8 as *const c_char, types: unsafe { &wl_shell_surface_requests_set_maximized_types as *const _ } },
    wl_message { name: b"set_title\0" as *const u8 as *const c_char, signature: b"s\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_class\0" as *const u8 as *const c_char, signature: b"s\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut wl_shell_surface_events: [wl_message; 3] = [
    wl_message { name: b"ping\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"configure\0" as *const u8 as *const c_char, signature: b"uii\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"popup_done\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_shell_surface_interface: wl_interface = wl_interface {
    name: b"wl_shell_surface\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 10,
    requests: unsafe { &wl_shell_surface_requests as *const _ },
    event_count: 3,
    events: unsafe { &wl_shell_surface_events as *const _ },
};

// wl_surface

static mut wl_surface_requests_attach_types: [*const wl_interface; 3] = [
    unsafe { &wl_buffer_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut wl_surface_requests_frame_types: [*const wl_interface; 1] = [
    unsafe { &wl_callback_interface as *const wl_interface },
];
static mut wl_surface_requests_set_opaque_region_types: [*const wl_interface; 1] = [
    unsafe { &wl_region_interface as *const wl_interface },
];
static mut wl_surface_requests_set_input_region_types: [*const wl_interface; 1] = [
    unsafe { &wl_region_interface as *const wl_interface },
];
pub static mut wl_surface_requests: [wl_message; 10] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"attach\0" as *const u8 as *const c_char, signature: b"?oii\0" as *const u8 as *const c_char, types: unsafe { &wl_surface_requests_attach_types as *const _ } },
    wl_message { name: b"damage\0" as *const u8 as *const c_char, signature: b"iiii\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"frame\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &wl_surface_requests_frame_types as *const _ } },
    wl_message { name: b"set_opaque_region\0" as *const u8 as *const c_char, signature: b"?o\0" as *const u8 as *const c_char, types: unsafe { &wl_surface_requests_set_opaque_region_types as *const _ } },
    wl_message { name: b"set_input_region\0" as *const u8 as *const c_char, signature: b"?o\0" as *const u8 as *const c_char, types: unsafe { &wl_surface_requests_set_input_region_types as *const _ } },
    wl_message { name: b"commit\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_buffer_transform\0" as *const u8 as *const c_char, signature: b"2i\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_buffer_scale\0" as *const u8 as *const c_char, signature: b"3i\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"damage_buffer\0" as *const u8 as *const c_char, signature: b"4iiii\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
static mut wl_surface_events_enter_types: [*const wl_interface; 1] = [
    unsafe { &wl_output_interface as *const wl_interface },
];
static mut wl_surface_events_leave_types: [*const wl_interface; 1] = [
    unsafe { &wl_output_interface as *const wl_interface },
];
pub static mut wl_surface_events: [wl_message; 2] = [
    wl_message { name: b"enter\0" as *const u8 as *const c_char, signature: b"o\0" as *const u8 as *const c_char, types: unsafe { &wl_surface_events_enter_types as *const _ } },
    wl_message { name: b"leave\0" as *const u8 as *const c_char, signature: b"o\0" as *const u8 as *const c_char, types: unsafe { &wl_surface_events_leave_types as *const _ } },
];

pub static mut wl_surface_interface: wl_interface = wl_interface {
    name: b"wl_surface\0" as *const u8  as *const c_char,
    version: 4,
    request_count: 10,
    requests: unsafe { &wl_surface_requests as *const _ },
    event_count: 2,
    events: unsafe { &wl_surface_events as *const _ },
};

// wl_seat

static mut wl_seat_requests_get_pointer_types: [*const wl_interface; 1] = [
    unsafe { &wl_pointer_interface as *const wl_interface },
];
static mut wl_seat_requests_get_keyboard_types: [*const wl_interface; 1] = [
    unsafe { &wl_keyboard_interface as *const wl_interface },
];
static mut wl_seat_requests_get_touch_types: [*const wl_interface; 1] = [
    unsafe { &wl_touch_interface as *const wl_interface },
];
pub static mut wl_seat_requests: [wl_message; 4] = [
    wl_message { name: b"get_pointer\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &wl_seat_requests_get_pointer_types as *const _ } },
    wl_message { name: b"get_keyboard\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &wl_seat_requests_get_keyboard_types as *const _ } },
    wl_message { name: b"get_touch\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &wl_seat_requests_get_touch_types as *const _ } },
    wl_message { name: b"release\0" as *const u8 as *const c_char, signature: b"5\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut wl_seat_events: [wl_message; 2] = [
    wl_message { name: b"capabilities\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"name\0" as *const u8 as *const c_char, signature: b"2s\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_seat_interface: wl_interface = wl_interface {
    name: b"wl_seat\0" as *const u8  as *const c_char,
    version: 5,
    request_count: 4,
    requests: unsafe { &wl_seat_requests as *const _ },
    event_count: 2,
    events: unsafe { &wl_seat_events as *const _ },
};

// wl_pointer

static mut wl_pointer_requests_set_cursor_types: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut wl_pointer_requests: [wl_message; 2] = [
    wl_message { name: b"set_cursor\0" as *const u8 as *const c_char, signature: b"u?oii\0" as *const u8 as *const c_char, types: unsafe { &wl_pointer_requests_set_cursor_types as *const _ } },
    wl_message { name: b"release\0" as *const u8 as *const c_char, signature: b"3\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
static mut wl_pointer_events_enter_types: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut wl_pointer_events_leave_types: [*const wl_interface; 2] = [
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut wl_pointer_events: [wl_message; 9] = [
    wl_message { name: b"enter\0" as *const u8 as *const c_char, signature: b"uoff\0" as *const u8 as *const c_char, types: unsafe { &wl_pointer_events_enter_types as *const _ } },
    wl_message { name: b"leave\0" as *const u8 as *const c_char, signature: b"uo\0" as *const u8 as *const c_char, types: unsafe { &wl_pointer_events_leave_types as *const _ } },
    wl_message { name: b"motion\0" as *const u8 as *const c_char, signature: b"uff\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"button\0" as *const u8 as *const c_char, signature: b"uuuu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"axis\0" as *const u8 as *const c_char, signature: b"uuf\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"frame\0" as *const u8 as *const c_char, signature: b"5\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"axis_source\0" as *const u8 as *const c_char, signature: b"5u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"axis_stop\0" as *const u8 as *const c_char, signature: b"5uu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"axis_discrete\0" as *const u8 as *const c_char, signature: b"5ui\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_pointer_interface: wl_interface = wl_interface {
    name: b"wl_pointer\0" as *const u8  as *const c_char,
    version: 5,
    request_count: 2,
    requests: unsafe { &wl_pointer_requests as *const _ },
    event_count: 9,
    events: unsafe { &wl_pointer_events as *const _ },
};

// wl_keyboard

pub static mut wl_keyboard_requests: [wl_message; 1] = [
    wl_message { name: b"release\0" as *const u8 as *const c_char, signature: b"3\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
static mut wl_keyboard_events_enter_types: [*const wl_interface; 3] = [
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
static mut wl_keyboard_events_leave_types: [*const wl_interface; 2] = [
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut wl_keyboard_events: [wl_message; 6] = [
    wl_message { name: b"keymap\0" as *const u8 as *const c_char, signature: b"uhu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"enter\0" as *const u8 as *const c_char, signature: b"uoa\0" as *const u8 as *const c_char, types: unsafe { &wl_keyboard_events_enter_types as *const _ } },
    wl_message { name: b"leave\0" as *const u8 as *const c_char, signature: b"uo\0" as *const u8 as *const c_char, types: unsafe { &wl_keyboard_events_leave_types as *const _ } },
    wl_message { name: b"key\0" as *const u8 as *const c_char, signature: b"uuuu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"modifiers\0" as *const u8 as *const c_char, signature: b"uuuuu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"repeat_info\0" as *const u8 as *const c_char, signature: b"4ii\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_keyboard_interface: wl_interface = wl_interface {
    name: b"wl_keyboard\0" as *const u8  as *const c_char,
    version: 5,
    request_count: 1,
    requests: unsafe { &wl_keyboard_requests as *const _ },
    event_count: 6,
    events: unsafe { &wl_keyboard_events as *const _ },
};

// wl_touch

pub static mut wl_touch_requests: [wl_message; 1] = [
    wl_message { name: b"release\0" as *const u8 as *const c_char, signature: b"3\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
static mut wl_touch_events_down_types: [*const wl_interface; 6] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut wl_touch_events: [wl_message; 5] = [
    wl_message { name: b"down\0" as *const u8 as *const c_char, signature: b"uuoiff\0" as *const u8 as *const c_char, types: unsafe { &wl_touch_events_down_types as *const _ } },
    wl_message { name: b"up\0" as *const u8 as *const c_char, signature: b"uui\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"motion\0" as *const u8 as *const c_char, signature: b"uiff\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"frame\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"cancel\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_touch_interface: wl_interface = wl_interface {
    name: b"wl_touch\0" as *const u8  as *const c_char,
    version: 5,
    request_count: 1,
    requests: unsafe { &wl_touch_requests as *const _ },
    event_count: 5,
    events: unsafe { &wl_touch_events as *const _ },
};

// wl_output

pub static mut wl_output_requests: [wl_message; 1] = [
    wl_message { name: b"release\0" as *const u8 as *const c_char, signature: b"3\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut wl_output_events: [wl_message; 4] = [
    wl_message { name: b"geometry\0" as *const u8 as *const c_char, signature: b"iiiiissi\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"mode\0" as *const u8 as *const c_char, signature: b"uiii\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"done\0" as *const u8 as *const c_char, signature: b"2\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"scale\0" as *const u8 as *const c_char, signature: b"2i\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_output_interface: wl_interface = wl_interface {
    name: b"wl_output\0" as *const u8  as *const c_char,
    version: 3,
    request_count: 1,
    requests: unsafe { &wl_output_requests as *const _ },
    event_count: 4,
    events: unsafe { &wl_output_events as *const _ },
};

// wl_region

pub static mut wl_region_requests: [wl_message; 3] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"add\0" as *const u8 as *const c_char, signature: b"iiii\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"subtract\0" as *const u8 as *const c_char, signature: b"iiii\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_region_interface: wl_interface = wl_interface {
    name: b"wl_region\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 3,
    requests: unsafe { &wl_region_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// wl_subcompositor

static mut wl_subcompositor_requests_get_subsurface_types: [*const wl_interface; 3] = [
    unsafe { &wl_subsurface_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut wl_subcompositor_requests: [wl_message; 2] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"get_subsurface\0" as *const u8 as *const c_char, signature: b"noo\0" as *const u8 as *const c_char, types: unsafe { &wl_subcompositor_requests_get_subsurface_types as *const _ } },
];

pub static mut wl_subcompositor_interface: wl_interface = wl_interface {
    name: b"wl_subcompositor\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &wl_subcompositor_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// wl_subsurface

static mut wl_subsurface_requests_place_above_types: [*const wl_interface; 1] = [
    unsafe { &wl_surface_interface as *const wl_interface },
];
static mut wl_subsurface_requests_place_below_types: [*const wl_interface; 1] = [
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut wl_subsurface_requests: [wl_message; 6] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_position\0" as *const u8 as *const c_char, signature: b"ii\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"place_above\0" as *const u8 as *const c_char, signature: b"o\0" as *const u8 as *const c_char, types: unsafe { &wl_subsurface_requests_place_above_types as *const _ } },
    wl_message { name: b"place_below\0" as *const u8 as *const c_char, signature: b"o\0" as *const u8 as *const c_char, types: unsafe { &wl_subsurface_requests_place_below_types as *const _ } },
    wl_message { name: b"set_sync\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_desync\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wl_subsurface_interface: wl_interface = wl_interface {
    name: b"wl_subsurface\0" as *const u8  as *const c_char,
    version: 1,
    request_count: 6,
    requests: unsafe { &wl_subsurface_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

