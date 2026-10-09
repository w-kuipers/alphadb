mod combine;
mod parse;

pub use combine::{build_version_source_from_dir, combine_version_source_files, gather_version_source_files};
pub use parse::parse_version_source;
