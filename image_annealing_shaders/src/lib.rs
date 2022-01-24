pub mod binding;
mod compute;
pub mod constant;
mod function;
mod global;
pub mod output;
mod resource;
pub mod shader;
mod type_definitions;

pub use compute::WorkgroupDimensions;
pub use function::main::SHADER_ENTRY_POINT;
