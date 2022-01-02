mod displacement_goal {
    use super::super::validation;
    use super::super::DisplacementGoal;
    use crate::CandidatePermutation;
    use std::error::Error;
    use test_utils::permutation::{self, DimensionsAndPermutation};

    #[test]
    fn from_vector_field() {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = permutation.clone();
        let displacement_goal = DisplacementGoal::from_vector_field(permutation);
        assert_eq!(*displacement_goal.as_ref(), expected);
    }

    #[test]
    fn from_candidate_permutation() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = validation::validate_permutation(permutation.clone())?.inverse();
        let displacement_goal =
            DisplacementGoal::from_candidate_permutation(CandidatePermutation(permutation))?;
        assert_eq!(displacement_goal.as_ref(), expected.as_ref());
        Ok(())
    }

    #[test]
    fn from_validated_permutation() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let validated_permutation = validation::validate_permutation(permutation)?;
        let expected = validated_permutation.inverse();
        let displacement_goal = DisplacementGoal::from(validated_permutation);
        assert_eq!(displacement_goal.as_ref(), expected.as_ref());
        Ok(())
    }
}
