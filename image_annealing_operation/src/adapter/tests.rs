mod invalid_power_preference_string_error {
    use super::super::InvalidPowerPreferenceStringError;
    #[test]
    fn check_display_string() {
        let value = "Invalid";
        assert_eq!(
            InvalidPowerPreferenceStringError(value.into()).to_string(),
            format!(
                "adapter power preference, \"{value}\", is not a case-insensitive match with \"none\", \"low\" or \"high\""
            )
        );
    }
}

mod parse_power_preference_from_string {
    use super::super::{InvalidPowerPreferenceStringError, PowerPreference};

    #[test]
    fn parse_from_all_valid_strings() {
        for (string, value) in [
            ("none", PowerPreference::None),
            ("low", PowerPreference::LowPower),
            ("high", PowerPreference::HighPerformance),
        ]
        .into_iter()
        {
            assert_eq!(PowerPreference::try_from(string), Ok(value));
        }
    }

    #[test]
    fn parsing_from_a_string_is_case_insensitive() {
        for (string, value) in [
            ("NONE", PowerPreference::None),
            ("LOW", PowerPreference::LowPower),
            ("HIGH", PowerPreference::HighPerformance),
        ]
        .into_iter()
        {
            assert_eq!(PowerPreference::try_from(string), Ok(value));
        }
    }

    #[test]
    fn parsing_from_a_string_ignores_padding_whitespace() {
        assert_eq!(
            PowerPreference::try_from(" none "),
            Ok(PowerPreference::None)
        );
    }

    #[test]
    fn an_invalid_string_results_in_a_parsing_error() {
        let invalid_input = "Invalid";
        let error = PowerPreference::try_from(invalid_input).unwrap_err();
        assert_eq!(
            error,
            InvalidPowerPreferenceStringError(invalid_input.to_string())
        );
    }
}

mod convert_power_preference_to_wgpu_power_preference {
    use super::super::PowerPreference;

    #[test]
    fn all_conversions() {
        for (input_value, output_value) in [
            (PowerPreference::None, wgpu::PowerPreference::None),
            (PowerPreference::LowPower, wgpu::PowerPreference::LowPower),
            (
                PowerPreference::HighPerformance,
                wgpu::PowerPreference::HighPerformance,
            ),
        ]
        .into_iter()
        {
            assert_eq!(wgpu::PowerPreference::from(input_value), output_value);
        }
    }
}
