mod invalid_instance_config_environment_variable_value_error {

    mod source {
        use super::super::super::InvalidInstanceConfigEnvironmentVariableValueError;
        use image_annealing_cli_util::env::InvalidBooleanEnvironmentVariableValueError;
        use image_annealing_operation::instance::{
            InvalidDirectX12ShaderCompilerNameError, InvalidOpengles3MinorVersionStringError,
        };
        use image_annealing_operation::InvalidBackendNameError;
        use std::error::Error;

        #[test]
        fn backend_name() {
            let inner_error = InvalidBackendNameError("Invalid".into());
            let expected_string = format!("{inner_error:?}");

            assert_eq!(
                format!(
                    "{:?}",
                    InvalidInstanceConfigEnvironmentVariableValueError::BackendName(inner_error)
                        .source()
                        .unwrap()
                ),
                expected_string
            );
        }

        #[test]
        fn boolean() {
            let inner_error = InvalidBooleanEnvironmentVariableValueError("Invalid".into());
            let expected_string = format!("{inner_error:?}");

            assert_eq!(
                format!(
                    "{:?}",
                    InvalidInstanceConfigEnvironmentVariableValueError::Boolean(inner_error)
                        .source()
                        .unwrap()
                ),
                expected_string
            );
        }

        #[test]
        fn direct_x12_shader_compiler_name() {
            let inner_error = InvalidDirectX12ShaderCompilerNameError("Invalid".into());
            let expected_string = format!("{inner_error:?}");

            assert_eq!(
                format!(
                    "{:?}",
                    InvalidInstanceConfigEnvironmentVariableValueError::DirectX12ShaderCompilerName(inner_error)
                        .source()
                        .unwrap()
                ),
                expected_string
            );
        }

        #[test]
        fn opengles3_minor_version_string() {
            let inner_error = InvalidOpengles3MinorVersionStringError("Invalid".into());
            let expected_string = format!("{inner_error:?}");

            assert_eq!(
                format!(
                    "{:?}",
                    InvalidInstanceConfigEnvironmentVariableValueError::Opengles3MinorVersionString(inner_error)
                        .source()
                        .unwrap()
                ),
                expected_string
            );
        }
    }
}

mod invalid_instance_config_environment_variable_error {
    use super::super::{
        InvalidInstanceConfigEnvironmentVariableError,
        InvalidInstanceConfigEnvironmentVariableValueError,
    };
    use image_annealing_cli_util::env::InvalidBooleanEnvironmentVariableValueError;
    use std::error::Error;

    mod display {
        use super::super::super::{
            InvalidInstanceConfigEnvironmentVariableError,
            InvalidInstanceConfigEnvironmentVariableValueError,
        };
        use image_annealing_cli_util::env::InvalidBooleanEnvironmentVariableValueError;
        use image_annealing_operation::instance::{
            InvalidDirectX12ShaderCompilerNameError, InvalidOpengles3MinorVersionStringError,
        };
        use image_annealing_operation::InvalidBackendNameError;

        #[test]
        fn backend_name() {
            let key = "TEST_KEY";
            let value = "Invalid";
            let error_string = InvalidInstanceConfigEnvironmentVariableError::new(
                key,
                InvalidInstanceConfigEnvironmentVariableValueError::BackendName(
                    InvalidBackendNameError(value.into()),
                ),
            )
            .to_string();

            let expected_string = format!("error interpreting environment variable \"{key}\": GPU backend name, \"{value}\", is not a case-insensitive match with \"directx12\", \"dx12\", \"d3d12\", \"metal\", \"mtl\", \"opengl\", \"gles\", \"gl\", \"vulkan\", \"vk\", \"primary_native\", \"secondary_native\" or \"any\"");
            assert_eq!(error_string, expected_string)
        }

        #[test]
        fn boolean() {
            let key = "TEST_KEY";
            let value = "Invalid";
            let error_string = InvalidInstanceConfigEnvironmentVariableError::new(
                key,
                InvalidInstanceConfigEnvironmentVariableValueError::Boolean(
                    InvalidBooleanEnvironmentVariableValueError(value.into()),
                ),
            )
            .to_string();

            let expected_string = format!("error interpreting environment variable \"{key}\": Boolean environment variable value, \"{value}\", is not a case-insensitive match with \"true\", \"1\", \"false\" or \"0\"");
            assert_eq!(error_string, expected_string)
        }

        #[test]
        fn direct_x12_shader_compiler_name() {
            let key = "TEST_KEY";
            let value = "Invalid";
            let error_string = InvalidInstanceConfigEnvironmentVariableError::new(
                key,
                InvalidInstanceConfigEnvironmentVariableValueError::DirectX12ShaderCompilerName(
                    InvalidDirectX12ShaderCompilerNameError(value.into()),
                ),
            )
            .to_string();

            let expected_string = format!("error interpreting environment variable \"{key}\": DirectX12 shader compiler name, \"{value}\", is not a case-insensitive match with \"dxc\" or \"fxc\"");
            assert_eq!(error_string, expected_string)
        }

        #[test]
        fn opengles3_minor_version_string() {
            let key = "TEST_KEY";
            let value = "Invalid";
            let error_string = InvalidInstanceConfigEnvironmentVariableError::new(
                key,
                InvalidInstanceConfigEnvironmentVariableValueError::Opengles3MinorVersionString(
                    InvalidOpengles3MinorVersionStringError(value.into()),
                ),
            )
            .to_string();

            let expected_string = format!("error interpreting environment variable \"{key}\": minor OpenGL ES 3 version, \"{value}\", is not a case-insensitive match with \"automatic\", \"0\", \"1\" or \"2\"");
            assert_eq!(error_string, expected_string)
        }
    }

    #[test]
    fn source_is_inner_error() {
        let key = "TEST_KEY";
        let value = "Invalid";
        let inner_error = InvalidInstanceConfigEnvironmentVariableValueError::Boolean(
            InvalidBooleanEnvironmentVariableValueError(value.into()),
        );
        let expected_string = format!("{inner_error:?}");

        assert_eq!(
            format!(
                "{:?}",
                InvalidInstanceConfigEnvironmentVariableError::new(key, inner_error)
                    .source()
                    .unwrap()
            ),
            expected_string
        );
    }
}

mod environment_variables_to_instance_config {
    use super::super::{
        InvalidInstanceConfigEnvironmentVariableError,
        InvalidInstanceConfigEnvironmentVariableValueError,
    };
    use image_annealing_cli_util::env::InvalidBooleanEnvironmentVariableValueError;
    use image_annealing_operation::instance::{
        DirectX12ShaderCompiler, InstanceConfig, InvalidDirectX12ShaderCompilerNameError,
        InvalidOpengles3MinorVersionStringError, Opengles3MinorVersion,
    };
    use image_annealing_operation::{Backend, InvalidBackendNameError};

    #[test]
    fn no_environment_variables_produce_default_config() {
        assert_eq!(
            super::super::environment_variables_to_instance_config(Default::default()),
            Ok(Default::default())
        );
    }

    #[test]
    fn custom_environment_variables_produce_custom_config() {
        assert_eq!(
            super::super::environment_variables_to_instance_config([
                Some("vulkan"),
                Some("1"),
                Some("1"),
                Some("1"),
                Some("dxc"),
                Some("2"),
            ]),
            Ok(InstanceConfig {
                backend: Backend::Vulkan,
                allow_underlying_noncompliant_adapter: true,
                debug: true,
                validation: true,
                directx12_shader_compiler: DirectX12ShaderCompiler::Dxc,
                opengles3_minor_version: Opengles3MinorVersion::Version2,
            })
        );
    }

    #[test]
    fn invalid_backend_name_environment_variable() {
        let value = "Invalid";
        assert_eq!(
            super::super::environment_variables_to_instance_config([
                Some(value),
                None,
                None,
                None,
                None,
                None,
            ]),
            Err(InvalidInstanceConfigEnvironmentVariableError {
                key: "WGPU_BACKEND_NAME".into(),
                error: InvalidInstanceConfigEnvironmentVariableValueError::BackendName(
                    InvalidBackendNameError(value.into()),
                ),
            })
        );
    }

    #[test]
    fn invalid_allow_underlying_noncompliant_adapter_environment_variable() {
        let value = "Invalid";
        assert_eq!(
            super::super::environment_variables_to_instance_config([
                None,
                Some(value),
                None,
                None,
                None,
                None,
            ]),
            Err(InvalidInstanceConfigEnvironmentVariableError {
                key: "WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER".into(),
                error: InvalidInstanceConfigEnvironmentVariableValueError::Boolean(
                    InvalidBooleanEnvironmentVariableValueError(value.into()),
                ),
            })
        );
    }

    #[test]
    fn invalid_debug_environment_variable() {
        let value = "Invalid";
        assert_eq!(
            super::super::environment_variables_to_instance_config([
                None,
                None,
                Some(value),
                None,
                None,
                None,
            ]),
            Err(InvalidInstanceConfigEnvironmentVariableError {
                key: "WGPU_DEBUG".into(),
                error: InvalidInstanceConfigEnvironmentVariableValueError::Boolean(
                    InvalidBooleanEnvironmentVariableValueError(value.into()),
                ),
            })
        );
    }

    #[test]
    fn invalid_validation_environment_variable() {
        let value = "Invalid";
        assert_eq!(
            super::super::environment_variables_to_instance_config([
                None,
                None,
                None,
                Some(value),
                None,
                None,
            ]),
            Err(InvalidInstanceConfigEnvironmentVariableError {
                key: "WGPU_VALIDATION".into(),
                error: InvalidInstanceConfigEnvironmentVariableValueError::Boolean(
                    InvalidBooleanEnvironmentVariableValueError(value.into()),
                ),
            })
        );
    }

    #[test]
    fn invalid_dx12_compiler_environment_variable() {
        let value = "Invalid";
        assert_eq!(
            super::super::environment_variables_to_instance_config([
                None,
                None,
                None,
                None,
                Some(value),
                None,
            ]),
            Err(InvalidInstanceConfigEnvironmentVariableError {
                key: "WGPU_DX12_COMPILER".into(),
                error:
                    InvalidInstanceConfigEnvironmentVariableValueError::DirectX12ShaderCompilerName(
                        InvalidDirectX12ShaderCompilerNameError(value.into()),
                    ),
            })
        );
    }

    #[test]
    fn invalid_gles_minor_version_environment_variable() {
        let value = "Invalid";
        assert_eq!(
            super::super::environment_variables_to_instance_config([
                None,
                None,
                None,
                None,
                None,
                Some(value),
            ]),
            Err(InvalidInstanceConfigEnvironmentVariableError {
                key: "WGPU_GLES_MINOR_VERSION".into(),
                error:
                    InvalidInstanceConfigEnvironmentVariableValueError::Opengles3MinorVersionString(
                        InvalidOpengles3MinorVersionStringError(value.into()),
                    ),
            })
        );
    }
}
