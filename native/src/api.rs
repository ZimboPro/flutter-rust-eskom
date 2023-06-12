// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use std::time::Duration;

use eskom_se_push_api::allowance::Allowance;
use eskom_se_push_api::Endpoint;
use flutter_rust_bridge::StreamSink;

// A plain enum without any fields. This is similar to Dart- or C-style enums.
// flutter_rust_bridge is capable of generating code for enums with fields
// (@freezed classes in Dart and tagged unions in C).
pub enum Platform {
    Unknown,
    Android,
    Ios,
    Windows,
    Unix,
    MacIntel,
    MacApple,
    Wasm,
}

// A function definition in Rust. Similar to Dart, the return type must always be named
// and is never inferred.
pub fn platform() -> Platform {
    // This is a macro, a special expression that expands into code. In Rust, all macros
    // end with an exclamation mark and can be invoked with all kinds of brackets (parentheses,
    // brackets and curly braces). However, certain conventions exist, for example the
    // vector macro is almost always invoked as vec![..].
    //
    // The cfg!() macro returns a boolean value based on the current compiler configuration.
    // When attached to expressions (#[cfg(..)] form), they show or hide the expression at compile time.
    // Here, however, they evaluate to runtime values, which may or may not be optimized out
    // by the compiler. A variety of configurations are demonstrated here which cover most of
    // the modern oeprating systems. Try running the Flutter application on different machines
    // and see if it matches your expected OS.
    //
    // Furthermore, in Rust, the last expression in a function is the return value and does
    // not have the trailing semicolon. This entire if-else chain forms a single expression.
    if cfg!(windows) {
        Platform::Windows
    } else if cfg!(target_os = "android") {
        Platform::Android
    } else if cfg!(target_os = "ios") {
        Platform::Ios
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Platform::MacApple
    } else if cfg!(target_os = "macos") {
        Platform::MacIntel
    } else if cfg!(target_family = "wasm") {
        Platform::Wasm
    } else if cfg!(unix) {
        Platform::Unix
    } else {
        Platform::Unknown
    }
}

// The convention for Rust identifiers is the snake_case,
// and they are automatically converted to camelCase on the Dart side.
pub fn rust_release_mode() -> bool {
    cfg!(not(debug_assertions))
}

const TIME: Duration = Duration::from_secs(1);

pub fn tick(sink: StreamSink<usize>) -> anyhow::Result<()> {
    let mut ticks = 0;
    loop {
        sink.add(ticks);
        std::thread::sleep(TIME);
        if ticks == usize::MAX {
            break;
        }
        ticks += 1;
    }
    Ok(())
}

pub fn test_api_key(api_key: String) -> bool {
    let t = eskom_se_push_api::allowance::AllowanceCheckURLBuilder::default()
        .build()
        .unwrap();

    let resp = t.ureq(api_key.as_str());
    if let Ok(_response) = resp {
        return true;
    }
    false
}

#[derive(Debug, Default)]
pub struct AllowanceUsage {
    pub limit: i64,
    pub count: i64,
    pub account_type: String,
}

impl From<Allowance> for AllowanceUsage {
    fn from(value: Allowance) -> Self {
        Self {
            limit: value.limit,
            count: value.count,
            account_type: value.type_field,
        }
    }
}

pub fn allowance(api_key: String) -> anyhow::Result<AllowanceUsage> {
    let t = eskom_se_push_api::allowance::AllowanceCheckURLBuilder::default()
        .build()
        .unwrap();
    let resp = t.ureq(api_key.as_str())?;
    Ok(resp.allowance.into())
}

pub struct AreaSearchResult {
    pub area_id: String,
    pub name: String,
    pub region: String,
}

impl From<eskom_se_push_api::area_search::Area> for AreaSearchResult {
    fn from(value: eskom_se_push_api::area_search::Area) -> Self {
        Self {
            name: value.name,
            area_id: value.id,
            region: value.region,
        }
    }
}

pub fn area_search(api_key: String, search_term: String) -> anyhow::Result<Vec<AreaSearchResult>> {
    let t = eskom_se_push_api::area_search::AreaSearchURLBuilder::default()
        .search_term(search_term)
        .build()
        .unwrap();
    let resp = t.ureq(api_key.as_str())?;
    Ok(resp.areas.into_iter().map(|x| x.into()).collect())
}

pub fn area_info(api_key: String, area_id: String) -> anyhow::Result<AreaInfoResponse> {
    let t = eskom_se_push_api::area_info::AreaInfoURLBuilder::default()
        .area_id(area_id)
        .build()
        .unwrap();
    let resp = t.ureq(api_key.as_str())?;
    Ok(resp.into())
}

pub fn add_area(api_key: String, area_id: String) -> anyhow::Result<AreaInfoResponse> {
    // TODO add area to list/config
    area_info(api_key, area_id)
}

impl From<eskom_se_push_api::area_info::AreaInfo> for AreaInfoResponse {
    fn from(value: eskom_se_push_api::area_info::AreaInfo) -> Self {
        Self {
            events: value.events.into_iter().map(|x| x.into()).collect(),
            info: value.info.into(),
            schedule: value.schedule.into(),
        }
    }
}

impl From<eskom_se_push_api::area_info::Schedule> for Schedule {
    fn from(value: eskom_se_push_api::area_info::Schedule) -> Self {
        Self {
            days: value.days.into_iter().map(|x| x.into()).collect(),
            source: value.source,
        }
    }
}

impl From<eskom_se_push_api::area_info::Day> for Day {
    fn from(value: eskom_se_push_api::area_info::Day) -> Self {
        Self {
            date: value.date,
            name: value.name,
            stages: value.stages,
        }
    }
}

impl From<eskom_se_push_api::area_info::Info> for Info {
    fn from(value: eskom_se_push_api::area_info::Info) -> Self {
        Self {
            name: value.name,
            region: value.region,
        }
    }
}

impl From<eskom_se_push_api::area_info::Event> for Event {
    fn from(value: eskom_se_push_api::area_info::Event) -> Self {
        Self {
            end: value.end,
            note: value.note,
            start: value.start,
        }
    }
}

pub struct AreaInfoResponse {
    /// List of sorted events. Will be an empty list if not impacted
    pub events: Vec<Event>,
    /// Info of the region requested for
    pub info: Info,
    /// Raw loadshedding schedule, per stage (1-8)
    /// `Note`: An empty list means no events for that stage
    /// `Note`: Some Municipalities/Regions don't have Stage 5-8 schedules (and there will be 4 records instead of 8 in this list. Stage 5 upwards you can assume Stage 4 schedule impact.
    pub schedule: Schedule,
}

pub struct Event {
    /// End time of the event eg `2022-08-08T22:30:00+02:00`
    pub end: String,
    /// The stage of the event eg `Stage 2`
    pub note: String,
    /// Start time of the event eg `2022-08-08T20:00:00+02:00`
    pub start: String,
}

pub struct Info {
    pub name: String,
    pub region: String,
}

pub struct Schedule {
    /// Vec of the days and there stages
    pub days: Vec<Day>,
    /// Where the data was retrieved from.
    pub source: String,
}

pub struct Day {
    /// Date the stages are relevant to eg `2022-08-08`
    pub date: String,
    /// Day of week eg `Monday`
    pub name: String,
    /// Raw loadshedding schedule, per stage (1-8).
    /// Index 0 refers to `Stage 1`, index 1 is `Stage 2` and so on and so forth.
    /// Formatted for display purposes `(i.e. 20:00-22:30)`.
    /// Any adjacent events have been merged into a single event `(e.g. 12:00-14:30 & 14:00-16:30 become 12:00-16:30)`.
    ///  * `Note`: An empty list means no events for that stage
    ///  * `Note`: Some Municipalities/Regions don't have Stage 5-8 schedules (and there will be 4 records instead of 8 in this list. Stage 5 upwards you can assume Stage 4 schedule impact.
    pub stages: Vec<Vec<String>>,
}
