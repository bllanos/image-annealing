mod swap_operation_input {
    use super::super::super::super::super::link::swap::SwapPass;
    use super::super::SwapOperationInput;

    #[test]
    fn from_pass() {
        let pass = SwapPass::OffsetVertical;
        let input = SwapOperationInput::from_pass(SwapPass::OffsetVertical);
        assert_eq!(
            input,
            SwapOperationInput {
                pass,
                permutation: None,
                displacement_goal: None,
            }
        );
    }
}
