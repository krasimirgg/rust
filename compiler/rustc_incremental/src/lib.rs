//! Support for serializing the dep-graph and reloading it.

// tidy-alphabetical-start
#![allow(internal_features)]
#![deny(missing_docs)]
#![doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
#![doc(rust_logo)]
#![feature(rustdoc_internals)]
// tidy-alphabetical-end

mod assert_dep_graph;
mod errors;
mod persist;

pub use persist::copy_cgu_workproduct_to_incr_comp_cache_dir;
pub use persist::finalize_session_directory;
pub use persist::in_incr_comp_dir;
pub use persist::in_incr_comp_dir_sess;
pub use persist::load_query_result_cache;
pub use persist::save_dep_graph;
pub use persist::save_work_product_index;
pub use persist::setup_dep_graph;
pub use persist::LoadResult;

rustc_fluent_macro::fluent_messages! { "../messages.ftl" }
