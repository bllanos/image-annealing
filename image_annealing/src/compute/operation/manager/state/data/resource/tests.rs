mod resource_state_machine_wrapper {
    use super::super::{ResourceStateMachine, ResourceStateMachineWrapper, Zero};
    use std::marker::PhantomData;

    #[test]
    fn new() {
        let state = ResourceStateMachineWrapper::new();
        assert_eq!(
            state,
            ResourceStateMachineWrapper::Zero(ResourceStateMachine {
                state: PhantomData::<Zero>
            })
        );
    }

    mod write {
        use super::super::super::{
            ResourceStateMachine, ResourceStateMachineWrapper, Stale, Written, Zero,
        };
        use std::marker::PhantomData;

        #[test]
        fn zero() {
            let state = ResourceStateMachineWrapper::Zero(ResourceStateMachine {
                state: PhantomData::<Zero>,
            });
            assert_eq!(
                state.write(),
                ResourceStateMachineWrapper::Written(ResourceStateMachine {
                    state: PhantomData::<Written>
                })
            );
        }

        #[test]
        fn written() {
            let state = ResourceStateMachineWrapper::Written(ResourceStateMachine {
                state: PhantomData::<Written>,
            });
            assert_eq!(
                state.write(),
                ResourceStateMachineWrapper::Written(ResourceStateMachine {
                    state: PhantomData::<Written>
                })
            );
        }

        #[test]
        fn stale() {
            let state = ResourceStateMachineWrapper::Stale(ResourceStateMachine {
                state: PhantomData::<Stale>,
            });
            assert_eq!(
                state.write(),
                ResourceStateMachineWrapper::Written(ResourceStateMachine {
                    state: PhantomData::<Written>
                })
            );
        }
    }

    mod clear {
        use super::super::super::{
            ResourceStateMachine, ResourceStateMachineWrapper, Stale, Written, Zero,
        };
        use std::marker::PhantomData;

        #[test]
        fn zero() {
            let state = ResourceStateMachineWrapper::Zero(ResourceStateMachine {
                state: PhantomData::<Zero>,
            });
            assert_eq!(
                state.clear(),
                ResourceStateMachineWrapper::Stale(ResourceStateMachine {
                    state: PhantomData::<Stale>
                })
            );
        }

        #[test]
        fn written() {
            let state = ResourceStateMachineWrapper::Written(ResourceStateMachine {
                state: PhantomData::<Written>,
            });
            assert_eq!(
                state.clear(),
                ResourceStateMachineWrapper::Stale(ResourceStateMachine {
                    state: PhantomData::<Stale>
                })
            );
        }

        #[test]
        fn stale() {
            let state = ResourceStateMachineWrapper::Stale(ResourceStateMachine {
                state: PhantomData::<Stale>,
            });
            assert_eq!(
                state.clear(),
                ResourceStateMachineWrapper::Stale(ResourceStateMachine {
                    state: PhantomData::<Stale>
                })
            );
        }
    }

    mod observers {
        use super::super::super::{
            ResourceStateMachine, ResourceStateMachineWrapper, Stale, Written, Zero,
        };
        use std::marker::PhantomData;

        #[test]
        fn zero() {
            let state = ResourceStateMachineWrapper::Zero(ResourceStateMachine {
                state: PhantomData::<Zero>,
            });
            assert!(state.is_zero());
            assert!(!state.is_written());
            assert!(state.is_valid());
        }

        #[test]
        fn written() {
            let state = ResourceStateMachineWrapper::Written(ResourceStateMachine {
                state: PhantomData::<Written>,
            });
            assert!(!state.is_zero());
            assert!(state.is_written());
            assert!(state.is_valid());
        }

        #[test]
        fn stale() {
            let state = ResourceStateMachineWrapper::Stale(ResourceStateMachine {
                state: PhantomData::<Stale>,
            });
            assert!(!state.is_zero());
            assert!(!state.is_written());
            assert!(!state.is_valid());
        }
    }
}
