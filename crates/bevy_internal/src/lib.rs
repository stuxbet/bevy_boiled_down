#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://bevyengine.org/assets/icon.png",
    html_favicon_url = "https://bevyengine.org/assets/icon.png"
)]

//! This module is separated into its own crate to enable simple dynamic linking for Bevy, and should not be used directly

/// `use bevy::prelude::*;` to import common components, bundles, and plugins.
pub mod prelude;

mod default_plugins;
pub use default_plugins::*;

pub use bevy_a11y as a11y;
pub use bevy_app as app;
pub use bevy_asset as asset;
pub use bevy_core as core;
pub use bevy_dev_tools as dev_tools;
pub use bevy_diagnostic as diagnostic;
pub use bevy_ecs as ecs;
pub use bevy_log as log;
pub use bevy_math as math;
pub use bevy_ptr as ptr;
pub use bevy_reflect as reflect;
pub use bevy_state as state;
pub use bevy_tasks as tasks;
pub use bevy_time as time;
pub use bevy_utils as utils;

