mod nonnegative_rational_number {
    use super::super::NonnegativeRationalNumber;
    use std::convert::TryFrom;
    use std::error::Error;

    #[test]
    fn new() -> Result<(), Box<dyn Error>> {
        assert_eq!(NonnegativeRationalNumber::new(2.0)?.get(), 2.0);
        Ok(())
    }

    #[test]
    fn zero() -> Result<(), Box<dyn Error>> {
        assert_eq!(NonnegativeRationalNumber::new(0.0)?.get(), 0.0);
        Ok(())
    }

    #[test]
    fn infinite() {
        test_utils::assert_error_contains(
            NonnegativeRationalNumber::new(f64::INFINITY),
            "inf is not finite",
        );
    }

    #[test]
    fn nan() {
        test_utils::assert_error_contains(
            NonnegativeRationalNumber::new(f64::NAN),
            "NaN is not finite",
        );
    }

    #[test]
    fn negative() {
        test_utils::assert_error_contains(NonnegativeRationalNumber::new(-1.0), "-1 is negative");
    }

    #[test]
    fn try_from() -> Result<(), Box<dyn Error>> {
        assert_eq!(NonnegativeRationalNumber::try_from(2.0)?.get(), 2.0);
        Ok(())
    }
}

mod nonnegative_proper_fraction {
    use super::super::NonnegativeProperFraction;
    use std::convert::TryFrom;
    use std::error::Error;

    #[test]
    fn zero() -> Result<(), Box<dyn Error>> {
        assert_eq!(NonnegativeProperFraction::new(0.0)?.get(), 0.0);
        Ok(())
    }

    #[test]
    fn close_to_one() -> Result<(), Box<dyn Error>> {
        let value = 1.0 - f64::EPSILON;
        assert_eq!(NonnegativeProperFraction::new(value)?.get(), value);
        Ok(())
    }

    #[test]
    fn one() {
        test_utils::assert_error_contains(
            NonnegativeProperFraction::new(1.0),
            "1 is not less than one",
        );
    }

    #[test]
    fn negative() {
        test_utils::assert_error_contains(NonnegativeProperFraction::new(-1.0), "-1 is negative");
    }

    #[test]
    fn try_from() -> Result<(), Box<dyn Error>> {
        assert_eq!(NonnegativeProperFraction::try_from(0.5)?.get(), 0.5);
        Ok(())
    }
}
