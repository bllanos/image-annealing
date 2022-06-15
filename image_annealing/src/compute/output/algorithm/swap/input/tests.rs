mod swap_parameters {
    use super::super::{SwapParameters, SwapPass, SwapPassSequence};
    use std::error::Error;

    #[test]
    fn from_sequence() -> Result<(), Box<dyn Error>> {
        let sequence =
            SwapPassSequence::from_passes([SwapPass::OffsetHorizontal, SwapPass::OffsetVertical])?;
        let parameters = SwapParameters::from_sequence(sequence);
        assert_eq!(
            parameters,
            SwapParameters {
                sequence,
                swap_acceptance_threshold: Default::default(),
                count_swap: false
            }
        );
        Ok(())
    }

    #[test]
    fn from_sequence_and_threshold() -> Result<(), Box<dyn Error>> {
        let sequence =
            SwapPassSequence::from_passes([SwapPass::OffsetHorizontal, SwapPass::OffsetVertical])?;
        let swap_acceptance_threshold = 3.0;
        let parameters =
            SwapParameters::from_sequence_and_threshold(sequence, swap_acceptance_threshold);
        assert_eq!(
            parameters,
            SwapParameters {
                sequence,
                swap_acceptance_threshold,
                count_swap: false
            }
        );
        Ok(())
    }

    #[test]
    fn default() {
        assert_eq!(
            <SwapParameters as Default>::default(),
            SwapParameters {
                sequence: SwapPassSequence::all(),
                swap_acceptance_threshold: Default::default(),
                count_swap: Default::default(),
            }
        );
    }
}
