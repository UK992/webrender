#![feature(step_by, convert)]
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(serde_macros)]
#![feature(drain)]
#![feature(hashmap_hasher)]
#![feature(vec_push_all)]

extern crate app_units;
extern crate euclid;
extern crate fnv;
extern crate freetype;
extern crate gleam;
extern crate libc;
extern crate time;
extern crate string_cache;
extern crate serde;
extern crate bit_vec;
extern crate scoped_threadpool;
extern crate simd;

pub mod types;
pub mod renderer;
pub mod render_api;

mod aabbtree;
mod clipper;
mod device;
mod font;
mod internal_types;
mod layer;
mod renderbatch;
mod render_backend;
mod resource_list;
mod stats;
mod texture_cache;
mod util;

pub use types::{ImageID, StackingLevel, DisplayListID, StackingContext, DisplayListBuilder};
pub use types::{ColorF, ImageFormat, GradientStop, PipelineId, GlyphInstance, RenderNotifier};
pub use types::{BorderSide, BorderRadius, BorderStyle, Epoch, BoxShadowClipMode, ClipRegion};
pub use types::{ScrollLayerId, ScrollPolicy, MixBlendMode, ComplexClipRegion, FilterOp};
pub use render_api::RenderApi;
pub use renderer::Renderer;


#[doc(hidden)]
pub mod bench {
    // to make private modules available to the benchmarks
    pub use clipper::{clip_rect_pos_uv, clip_polygon, ClipBuffers};
    pub use internal_types::WorkVertex;
}