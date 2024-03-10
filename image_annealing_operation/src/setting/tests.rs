mod invalid_backend_name_error {
    use super::super::InvalidBackendNameError;
    #[test]
    fn check_display_string() {
        let value = "Invalid";
        assert_eq!(
            InvalidBackendNameError(value.into()).to_string(),
            format!(
                "GPU backend name, \"{value}\", is not a case-insensitive match with \"directx12\", \"dx12\", \"d3d12\", \"metal\", \"mtl\", \"opengl\", \"gles\", \"gl\", \"vulkan\", \"vk\", \"primary_native\", \"secondary_native\" or \"any\""
            )
        );
    }
}

mod parse_backend_from_string {
    use super::super::{Backend, InvalidBackendNameError};

    #[test]
    fn parse_from_all_valid_strings() {
        for (string, value) in [
            ("directx12", Backend::DirectX12),
            ("dx12", Backend::DirectX12),
            ("d3d12", Backend::DirectX12),
            ("metal", Backend::Metal),
            ("mtl", Backend::Metal),
            ("opengl", Backend::OpenGL),
            ("gles", Backend::OpenGL),
            ("gl", Backend::OpenGL),
            ("vulkan", Backend::Vulkan),
            ("vk", Backend::Vulkan),
            ("primary_native", Backend::PrimaryNative),
            ("secondary_native", Backend::SecondaryNative),
            ("any", Backend::Any),
        ]
        .into_iter()
        {
            assert_eq!(Backend::try_from(string), Ok(value));
        }
    }

    #[test]
    fn parsing_from_a_string_is_case_insensitive() {
        for (string, value) in [
            ("DIRECTX12", Backend::DirectX12),
            ("DX12", Backend::DirectX12),
            ("D3D12", Backend::DirectX12),
            ("METAL", Backend::Metal),
            ("MTL", Backend::Metal),
            ("OPENGL", Backend::OpenGL),
            ("GLES", Backend::OpenGL),
            ("GL", Backend::OpenGL),
            ("VULKAN", Backend::Vulkan),
            ("VK", Backend::Vulkan),
            ("PRIMARY_NATIVE", Backend::PrimaryNative),
            ("SECONDARY_NATIVE", Backend::SecondaryNative),
            ("ANY", Backend::Any),
        ]
        .into_iter()
        {
            assert_eq!(Backend::try_from(string), Ok(value));
        }
    }

    #[test]
    fn parsing_from_a_string_ignores_padding_whitespace() {
        assert_eq!(Backend::try_from(" directx12 "), Ok(Backend::DirectX12));
    }

    #[test]
    fn an_invalid_string_results_in_a_parsing_error() {
        let invalid_input = "Invalid";
        let error = Backend::try_from(invalid_input).unwrap_err();
        assert_eq!(error, InvalidBackendNameError(invalid_input.to_string()));
    }
}

mod convert_backend_to_wgpu_backends {
    use super::super::Backend;

    #[test]
    fn all_conversions() {
        for (input_value, output_value) in [
            (Backend::DirectX12, wgpu::Backends::DX12),
            (Backend::Metal, wgpu::Backends::METAL),
            (Backend::OpenGL, wgpu::Backends::GL),
            (Backend::Vulkan, wgpu::Backends::VULKAN),
            (
                Backend::PrimaryNative,
                wgpu::Backends::PRIMARY.difference(wgpu::Backends::BROWSER_WEBGPU),
            ),
            (Backend::SecondaryNative, wgpu::Backends::SECONDARY),
            (
                Backend::Any,
                wgpu::Backends::all().difference(wgpu::Backends::BROWSER_WEBGPU),
            ),
        ]
        .into_iter()
        {
            assert_eq!(wgpu::Backends::from(input_value), output_value);
        }
    }
}
