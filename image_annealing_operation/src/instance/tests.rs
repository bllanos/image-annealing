mod invalid_direct_x12_shader_compiler_name_error {
    use super::super::InvalidDirectX12ShaderCompilerNameError;
    #[test]
    fn check_display_string() {
        let value = "Invalid";
        assert_eq!(
            InvalidDirectX12ShaderCompilerNameError(value.into()).to_string(),
            format!(
                "DirectX12 shader compiler name, \"{value}\", is not a case-insensitive match with \"dxc\" or \"fxc\""
            )
        );
    }
}

mod parse_direct_x12_shader_compiler_from_string {
    use super::super::{DirectX12ShaderCompiler, InvalidDirectX12ShaderCompilerNameError};

    #[test]
    fn parse_from_all_valid_strings() {
        for (string, value) in [
            ("dxc", DirectX12ShaderCompiler::Dxc),
            ("fxc", DirectX12ShaderCompiler::Fxc),
        ]
        .into_iter()
        {
            assert_eq!(DirectX12ShaderCompiler::try_from(string), Ok(value));
        }
    }

    #[test]
    fn parsing_from_a_string_is_case_insensitive() {
        for (string, value) in [
            ("DXC", DirectX12ShaderCompiler::Dxc),
            ("FXC", DirectX12ShaderCompiler::Fxc),
        ]
        .into_iter()
        {
            assert_eq!(DirectX12ShaderCompiler::try_from(string), Ok(value));
        }
    }

    #[test]
    fn parsing_from_a_string_ignores_padding_whitespace() {
        assert_eq!(
            DirectX12ShaderCompiler::try_from(" dxc "),
            Ok(DirectX12ShaderCompiler::Dxc)
        );
    }

    #[test]
    fn an_invalid_string_results_in_a_parsing_error() {
        let invalid_input = "Invalid";
        let error = DirectX12ShaderCompiler::try_from(invalid_input).unwrap_err();
        assert_eq!(
            error,
            InvalidDirectX12ShaderCompilerNameError(invalid_input.to_string())
        );
    }
}

mod convert_direct_x12_shader_compiler_to_wgpu_direct_x12_shader_compiler {
    use super::super::DirectX12ShaderCompiler;

    #[test]
    fn dxc() {
        let converted_value = wgpu::Dx12Compiler::from(DirectX12ShaderCompiler::Dxc);
        assert!(matches!(
            converted_value,
            wgpu::Dx12Compiler::Dxc {
                dxil_path: None,
                dxc_path: None,
            }
        ));
    }

    #[test]
    fn fxc() {
        let converted_value = wgpu::Dx12Compiler::from(DirectX12ShaderCompiler::Fxc);
        assert!(matches!(converted_value, wgpu::Dx12Compiler::Fxc));
    }
}

mod invalid_opengles3_minor_version_string_error {
    use super::super::InvalidOpengles3MinorVersionStringError;
    #[test]
    fn check_display_string() {
        let value = "Invalid";
        assert_eq!(
            InvalidOpengles3MinorVersionStringError(value.into()).to_string(),
            format!(
                "minor OpenGL ES 3 version, \"{value}\", is not a case-insensitive match with \"automatic\", \"0\", \"1\" or \"2\""
            )
        );
    }
}

mod parse_opengles3_minor_version_from_string {
    use super::super::{InvalidOpengles3MinorVersionStringError, Opengles3MinorVersion};

    #[test]
    fn parse_from_all_valid_strings() {
        for (string, value) in [
            ("automatic", Opengles3MinorVersion::Automatic),
            ("0", Opengles3MinorVersion::Version0),
            ("1", Opengles3MinorVersion::Version1),
            ("2", Opengles3MinorVersion::Version2),
        ]
        .into_iter()
        {
            assert_eq!(Opengles3MinorVersion::try_from(string), Ok(value));
        }
    }

    #[test]
    fn parsing_from_a_string_is_case_insensitive() {
        assert_eq!(
            Opengles3MinorVersion::try_from("AUTOMATIC"),
            Ok(Opengles3MinorVersion::Automatic)
        );
    }

    #[test]
    fn parsing_from_a_string_ignores_padding_whitespace() {
        assert_eq!(
            Opengles3MinorVersion::try_from(" automatic "),
            Ok(Opengles3MinorVersion::Automatic)
        );
    }

    #[test]
    fn an_invalid_string_results_in_a_parsing_error() {
        let invalid_input = "Invalid";
        let error = Opengles3MinorVersion::try_from(invalid_input).unwrap_err();
        assert_eq!(
            error,
            InvalidOpengles3MinorVersionStringError(invalid_input.to_string())
        );
    }
}

mod convert_opengles3_minor_versio_to_wgpu_gles3_minor_version {
    use super::super::Opengles3MinorVersion;

    #[test]
    fn all_conversions() {
        for (input_value, output_value) in [
            (
                Opengles3MinorVersion::Automatic,
                wgpu::Gles3MinorVersion::Automatic,
            ),
            (
                Opengles3MinorVersion::Version0,
                wgpu::Gles3MinorVersion::Version0,
            ),
            (
                Opengles3MinorVersion::Version1,
                wgpu::Gles3MinorVersion::Version1,
            ),
            (
                Opengles3MinorVersion::Version2,
                wgpu::Gles3MinorVersion::Version2,
            ),
        ]
        .into_iter()
        {
            assert_eq!(wgpu::Gles3MinorVersion::from(input_value), output_value);
        }
    }
}

mod instance_config {
    use super::super::DirectX12ShaderCompiler;
    use super::super::InstanceConfig;
    use super::super::Opengles3MinorVersion;
    use crate::setting::Backend;

    #[test]
    fn default_value() {
        assert_eq!(
            InstanceConfig::default(),
            InstanceConfig {
                backend: Backend::PrimaryNative,
                allow_underlying_noncompliant_adapter: false,
                debug: false,
                validation: false,
                directx12_shader_compiler: DirectX12ShaderCompiler::Fxc,
                opengles3_minor_version: Opengles3MinorVersion::Automatic
            }
        )
    }
}

mod convert_instance_config_to_wgpu_instance_descriptor {
    use super::super::DirectX12ShaderCompiler;
    use super::super::InstanceConfig;
    use super::super::Opengles3MinorVersion;
    use crate::setting::Backend;

    #[test]
    fn true_flags_produce_all_relevant_instance_flags() {
        let backend = Backend::PrimaryNative;
        let directx12_shader_compiler = DirectX12ShaderCompiler::Fxc;
        let opengles3_minor_version = Opengles3MinorVersion::Automatic;
        let converted_value = wgpu::InstanceDescriptor::from(InstanceConfig {
            backend,
            allow_underlying_noncompliant_adapter: true,
            debug: true,
            validation: true,
            directx12_shader_compiler,
            opengles3_minor_version,
        });
        assert_eq!(converted_value.backends, wgpu::Backends::from(backend));
        assert_eq!(
            converted_value.flags,
            wgpu::InstanceFlags::all().difference(wgpu::InstanceFlags::DISCARD_HAL_LABELS)
        );
        assert!(matches!(
            converted_value.dx12_shader_compiler,
            wgpu::Dx12Compiler::Fxc
        ));
        assert_eq!(
            converted_value.gles_minor_version,
            wgpu::Gles3MinorVersion::from(opengles3_minor_version)
        );
    }

    #[test]
    fn false_flags_produce_empty_instance_flags() {
        let backend = Backend::PrimaryNative;
        let directx12_shader_compiler = DirectX12ShaderCompiler::Fxc;
        let opengles3_minor_version = Opengles3MinorVersion::Automatic;
        let converted_value = wgpu::InstanceDescriptor::from(InstanceConfig {
            backend,
            allow_underlying_noncompliant_adapter: false,
            debug: false,
            validation: false,
            directx12_shader_compiler,
            opengles3_minor_version,
        });
        assert_eq!(converted_value.backends, wgpu::Backends::from(backend));
        assert_eq!(converted_value.flags, wgpu::InstanceFlags::empty());
        assert!(matches!(
            converted_value.dx12_shader_compiler,
            wgpu::Dx12Compiler::Fxc
        ));
        assert_eq!(
            converted_value.gles_minor_version,
            wgpu::Gles3MinorVersion::from(opengles3_minor_version)
        );
    }
}
