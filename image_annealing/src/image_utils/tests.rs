mod vector_field {
    mod is_identity {
        use super::super::super::VectorField;
        use crate::compute::format;
        use crate::compute::format::{VectorFieldImageBuffer, VectorFieldImageBufferComponent};
        use crate::{ImageDimensions, ImageDimensionsHolder};
        use std::error::Error;

        struct IdentityVectorField(VectorFieldImageBuffer);

        impl AsRef<VectorFieldImageBuffer> for IdentityVectorField {
            fn as_ref(&self) -> &VectorFieldImageBuffer {
                &self.0
            }
        }

        impl PartialEq<VectorFieldImageBuffer> for IdentityVectorField {
            fn eq(&self, _other: &VectorFieldImageBuffer) -> bool {
                unreachable!()
            }
        }

        impl ImageDimensionsHolder for IdentityVectorField {
            fn dimensions(&self) -> &ImageDimensions {
                unreachable!()
            }
        }

        impl VectorField for IdentityVectorField {
            fn identity(dimensions: &ImageDimensions) -> Self {
                Self(format::identity(dimensions))
            }

            fn into_inner(self) -> VectorFieldImageBuffer {
                unreachable!()
            }

            fn as_raw_slice(&self) -> &[VectorFieldImageBufferComponent] {
                unreachable!()
            }
        }

        struct NonIdentityVectorField(VectorFieldImageBuffer);

        impl NonIdentityVectorField {
            fn new() -> Self {
                let mut image = image::RgbaImage::from_pixel(2, 3, image::Rgba([0; 4]));
                image.put_pixel(1, 2, image::Rgba([0, 0, 0, 1]));
                Self(image)
            }
        }

        impl AsRef<VectorFieldImageBuffer> for NonIdentityVectorField {
            fn as_ref(&self) -> &VectorFieldImageBuffer {
                &self.0
            }
        }

        impl PartialEq<VectorFieldImageBuffer> for NonIdentityVectorField {
            fn eq(&self, _other: &VectorFieldImageBuffer) -> bool {
                unreachable!()
            }
        }

        impl ImageDimensionsHolder for NonIdentityVectorField {
            fn dimensions(&self) -> &ImageDimensions {
                unreachable!()
            }
        }

        impl VectorField for NonIdentityVectorField {
            fn identity(_dimensions: &ImageDimensions) -> Self {
                unreachable!()
            }

            fn into_inner(self) -> VectorFieldImageBuffer {
                unreachable!()
            }

            fn as_raw_slice(&self) -> &[VectorFieldImageBufferComponent] {
                unreachable!()
            }
        }

        #[test]
        fn identity() -> Result<(), Box<dyn Error>> {
            assert!(IdentityVectorField::identity(&ImageDimensions::try_new(2, 3)?).is_identity());
            Ok(())
        }

        #[test]
        fn non_identity() {
            assert!(!NonIdentityVectorField::new().is_identity());
        }
    }
}
