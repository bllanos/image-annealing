pub mod cli;
pub mod config;

#[derive(Debug, PartialEq, Eq)]
pub struct CandidatePermutationPath(pub String);
#[derive(Debug, PartialEq, Eq)]
pub struct DisplacementGoalPath(pub String);
#[derive(Debug, PartialEq, Eq)]
pub struct ImagePath(pub String);
