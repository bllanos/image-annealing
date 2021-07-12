pub mod binding;
mod compute;
mod function;
pub mod output;
mod resource;
pub mod shader;

pub use compute::WorkgroupDimensions;
pub use function::main::SHADER_ENTRY_POINT;
