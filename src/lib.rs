//! # SMesh
//! A polygon mesh manipulation library

pub mod adapters;
pub mod smesh;
mod test_utils;
mod tests;

pub mod prelude {
    pub use crate::smesh::{
        edit_operations::*, error::*, iterators::*, mesh_query::*, model::connectivity::*,
        model::mesh::*, model::mesh_elements::*, selection::*, topological_operations::*, util::*,
        *,
    };

    pub use slotmap::SecondaryMap;
}
