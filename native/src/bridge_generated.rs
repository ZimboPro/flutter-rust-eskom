#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.77.1.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_platform_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "platform",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(platform()),
    )
}
fn wire_rust_release_mode_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "rust_release_mode",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(rust_release_mode()),
    )
}
fn wire_tick_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "tick",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| tick(task_callback.stream_sink()),
    )
}
fn wire_test_api_key_impl(port_: MessagePort, api_key: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "test_api_key",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_api_key = api_key.wire2api();
            move |task_callback| Ok(test_api_key(api_api_key))
        },
    )
}
fn wire_set_api_key_impl(port_: MessagePort, api_key: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_api_key",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_api_key = api_key.wire2api();
            move |task_callback| Ok(set_api_key(api_api_key))
        },
    )
}
fn wire_set_cache_dir_impl(port_: MessagePort, cache_dir: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_cache_dir",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_cache_dir = cache_dir.wire2api();
            move |task_callback| Ok(set_cache_dir(api_cache_dir))
        },
    )
}
fn wire_allowance_impl(port_: MessagePort, api_key: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "allowance",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_api_key = api_key.wire2api();
            move |task_callback| allowance(api_api_key)
        },
    )
}
fn wire_area_search_impl(port_: MessagePort, search_term: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "area_search",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_search_term = search_term.wire2api();
            move |task_callback| area_search(api_search_term)
        },
    )
}
fn wire_area_info_impl(port_: MessagePort, area_id: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "area_info",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_area_id = area_id.wire2api();
            move |task_callback| area_info(api_area_id)
        },
    )
}
fn wire_add_area_impl(port_: MessagePort, area_id: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "add_area",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_area_id = area_id.wire2api();
            move |task_callback| add_area(api_area_id)
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for AllowanceUsage {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.limit.into_dart(),
            self.count.into_dart(),
            self.account_type.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for AllowanceUsage {}

impl support::IntoDart for AreaInfoResponse {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.events.into_dart(),
            self.info.into_dart(),
            self.schedule.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for AreaInfoResponse {}

impl support::IntoDart for AreaSearchResult {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.area_id.into_dart(),
            self.name.into_dart(),
            self.region.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for AreaSearchResult {}

impl support::IntoDart for Day {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.date.into_dart(),
            self.name.into_dart(),
            self.stages.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Day {}

impl support::IntoDart for Event {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.end.into_dart(),
            self.note.into_dart(),
            self.start.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Event {}

impl support::IntoDart for Info {
    fn into_dart(self) -> support::DartAbi {
        vec![self.name.into_dart(), self.region.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Info {}

impl support::IntoDart for Platform {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Unknown => 0,
            Self::Android => 1,
            Self::Ios => 2,
            Self::Windows => 3,
            Self::Unix => 4,
            Self::MacIntel => 5,
            Self::MacApple => 6,
            Self::Wasm => 7,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Platform {}
impl support::IntoDart for Schedule {
    fn into_dart(self) -> support::DartAbi {
        vec![self.days.into_dart(), self.source.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Schedule {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
