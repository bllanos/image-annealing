mod data;
mod io;

pub use data::{
    AlgorithmConfig, Config, CreateDisplacementGoalInputConfig, DisplacementGoalPath, ImagePath,
    InvalidIterationCountError, InvalidNonnegativeProperFractionError,
    InvalidNonnegativeRationalNumberError, IterationCount, LosslessImagePath,
    NonnegativeProperFraction, NonnegativeRationalNumber, PermutationPath, SwapParametersConfig,
    SwapStopConfig, SwapStopThreshold, UnverifiedConfig,
    UnverifiedCreateDisplacementGoalInputConfig, UnverifiedCreateDisplacementGoalInputDataConfig,
    UnverifiedImageDimensionsConfig, UnverifiedIterationCount, UnverifiedSwapParametersConfig,
    UnverifiedSwapStopConfig, UnverifiedSwapStopThreshold,
};
pub use io::parse_config_file;
