mod output_status {
    use super::super::OutputStatus;

    #[test]
    fn is_final() {
        assert!(!OutputStatus::NoNewOutput.is_final());
        assert!(!OutputStatus::NewPartialOutput.is_final());
        assert!(!OutputStatus::NewFullOutput.is_final());
        assert!(!OutputStatus::NewPartialAndFullOutput.is_final());
        assert!(OutputStatus::FinalPartialOutput.is_final());
        assert!(OutputStatus::FinalFullOutput.is_final());
        assert!(OutputStatus::FinalPartialAndFullOutput.is_final());
    }

    #[test]
    fn is_partial() {
        assert!(!OutputStatus::NoNewOutput.is_partial());
        assert!(OutputStatus::NewPartialOutput.is_partial());
        assert!(!OutputStatus::NewFullOutput.is_partial());
        assert!(OutputStatus::NewPartialAndFullOutput.is_partial());
        assert!(OutputStatus::FinalPartialOutput.is_partial());
        assert!(!OutputStatus::FinalFullOutput.is_partial());
        assert!(OutputStatus::FinalPartialAndFullOutput.is_partial());
    }

    #[test]
    fn is_full() {
        assert!(!OutputStatus::NoNewOutput.is_full());
        assert!(!OutputStatus::NewPartialOutput.is_full());
        assert!(OutputStatus::NewFullOutput.is_full());
        assert!(OutputStatus::NewPartialAndFullOutput.is_full());
        assert!(!OutputStatus::FinalPartialOutput.is_full());
        assert!(OutputStatus::FinalFullOutput.is_full());
        assert!(OutputStatus::FinalPartialAndFullOutput.is_full());
    }
}

mod algorithm {
    use super::super::{Algorithm, OutputStatus};
    use async_trait::async_trait;
    use std::error::Error;
    use std::task::{Context, Poll};

    fn diverging_poll_fn(_cx: &mut Context<'_>) -> Poll<Option<()>> {
        unreachable!()
    }

    async fn make_diverging_future() -> Option<()> {
        std::future::poll_fn(diverging_poll_fn).await
    }

    struct ZeroStepAlgorithm(usize);

    impl ZeroStepAlgorithm {
        pub fn new() -> Self {
            Self(0)
        }
    }

    #[async_trait]
    impl Algorithm<(), ()> for ZeroStepAlgorithm {
        fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
            if self.0 == 0 {
                self.0 = 1;
                Err(Box::<dyn Error>::from("ZeroStepAlgorithm error"))
            } else {
                unreachable!("step() should never be called after it returns an error");
            }
        }

        async fn partial_output(&mut self) -> Option<()> {
            make_diverging_future().await
        }

        fn partial_output_block(&mut self) -> Option<()> {
            unreachable!()
        }

        async fn full_output(&mut self) -> Option<()> {
            make_diverging_future().await
        }

        fn full_output_block(&mut self) -> Option<()> {
            unreachable!()
        }
    }

    struct OneStepAlgorithm(usize);

    impl OneStepAlgorithm {
        pub fn new() -> Self {
            Self(0)
        }
    }

    #[async_trait]
    impl Algorithm<(), ()> for OneStepAlgorithm {
        fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
            match self.0 {
                0 => {
                    self.0 = 1;
                    Ok(OutputStatus::FinalPartialAndFullOutput)
                }
                1 => {
                    self.0 = 2;
                    Err(Box::<dyn Error>::from("OneStepAlgorithm error"))
                }
                _ => unreachable!("step() should never be called after it returns an error"),
            }
        }

        async fn partial_output(&mut self) -> Option<()> {
            make_diverging_future().await
        }

        fn partial_output_block(&mut self) -> Option<()> {
            unreachable!()
        }

        async fn full_output(&mut self) -> Option<()> {
            make_diverging_future().await
        }

        fn full_output_block(&mut self) -> Option<()> {
            unreachable!()
        }
    }

    struct TwoStepAlgorithm(usize);

    impl TwoStepAlgorithm {
        pub fn new() -> Self {
            Self(0)
        }
    }

    #[async_trait]
    impl Algorithm<(), ()> for TwoStepAlgorithm {
        fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
            match self.0 {
                0 => {
                    self.0 = 1;
                    Ok(OutputStatus::NoNewOutput)
                }
                1 => {
                    self.0 = 2;
                    Ok(OutputStatus::FinalPartialAndFullOutput)
                }
                2 => {
                    self.0 = 3;
                    Err(Box::<dyn Error>::from("TwoStepAlgorithm error"))
                }
                _ => unreachable!("step() should never be called after it returns an error"),
            }
        }

        async fn partial_output(&mut self) -> Option<()> {
            make_diverging_future().await
        }

        fn partial_output_block(&mut self) -> Option<()> {
            unreachable!()
        }

        async fn full_output(&mut self) -> Option<()> {
            make_diverging_future().await
        }

        fn full_output_block(&mut self) -> Option<()> {
            unreachable!()
        }
    }

    mod step_until {
        use super::super::super::{Algorithm, OutputStatus};
        use std::error::Error;

        #[test]
        fn zero_step() {
            test_util::assert_error_contains(
                super::ZeroStepAlgorithm::new().step_until(OutputStatus::NoNewOutput),
                "ZeroStepAlgorithm error",
            );
        }

        #[test]
        fn one_step() -> Result<(), Box<dyn Error>> {
            let mut algorithm = super::OneStepAlgorithm::new();
            algorithm.step_until(OutputStatus::FinalPartialAndFullOutput)?;
            test_util::assert_error_contains(
                super::OneStepAlgorithm::new().step_until(OutputStatus::FinalFullOutput),
                "OneStepAlgorithm error",
            );
            Ok(())
        }

        #[test]
        fn two_step() -> Result<(), Box<dyn Error>> {
            let mut algorithm = super::TwoStepAlgorithm::new();
            algorithm.step_until(OutputStatus::FinalPartialAndFullOutput)?;
            test_util::assert_error_contains(
                super::TwoStepAlgorithm::new().step_until(OutputStatus::FinalFullOutput),
                "TwoStepAlgorithm error",
            );
            Ok(())
        }
    }

    mod step_until_finished {
        use super::super::super::{Algorithm, OutputStatus};
        use async_trait::async_trait;
        use std::error::Error;

        struct OneStepUnfinishedAlgorithm(usize);

        impl OneStepUnfinishedAlgorithm {
            pub fn new() -> Self {
                Self(0)
            }
        }

        #[async_trait]
        impl Algorithm<(), ()> for OneStepUnfinishedAlgorithm {
            fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
                match self.0 {
                    0 => {
                        self.0 = 1;
                        Ok(OutputStatus::NoNewOutput)
                    }
                    1 => {
                        self.0 = 2;
                        Err(Box::<dyn Error>::from("OneStepUnfinishedAlgorithm error"))
                    }
                    _ => unreachable!("step() should never be called after it returns an error"),
                }
            }

            async fn partial_output(&mut self) -> Option<()> {
                super::make_diverging_future().await
            }

            fn partial_output_block(&mut self) -> Option<()> {
                unreachable!()
            }

            async fn full_output(&mut self) -> Option<()> {
                super::make_diverging_future().await
            }

            fn full_output_block(&mut self) -> Option<()> {
                unreachable!()
            }
        }

        #[test]
        fn zero_step() {
            test_util::assert_error_contains(
                super::ZeroStepAlgorithm::new().step_until_finished(),
                "ZeroStepAlgorithm error",
            );
        }

        #[test]
        fn one_step() -> Result<(), Box<dyn Error>> {
            let mut algorithm = super::OneStepAlgorithm::new();
            assert_eq!(
                algorithm.step_until_finished()?,
                OutputStatus::FinalPartialAndFullOutput
            );
            Ok(())
        }

        #[test]
        fn one_step_unfinished() {
            test_util::assert_error_contains(
                OneStepUnfinishedAlgorithm::new().step_until_finished(),
                "OneStepUnfinishedAlgorithm error",
            );
        }

        #[test]
        fn two_step() -> Result<(), Box<dyn Error>> {
            let mut algorithm = super::TwoStepAlgorithm::new();
            assert_eq!(
                algorithm.step_until_finished()?,
                OutputStatus::FinalPartialAndFullOutput
            );
            Ok(())
        }
    }
}
