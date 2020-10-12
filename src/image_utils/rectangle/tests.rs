mod from_corners {
    use crate::image_utils::rectangle::Rectangle;
    use std::error::Error;

    #[test]
    fn empty_rect() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(0, 0, 0, 0)?;
        assert_eq!(r.x, 0);
        assert_eq!(r.y, 0);
        assert_eq!(r.width, 0);
        assert_eq!(r.height, 0);
        Ok(())
    }

    #[test]
    fn nonempty_rect() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(0, 1, 2, 3)?;
        assert_eq!(r.x, 0);
        assert_eq!(r.y, 1);
        assert_eq!(r.width, 2);
        assert_eq!(r.height, 2);
        Ok(())
    }

    #[test]
    fn disordered_x() {
        let r = Rectangle::from_corners(1, 0, 0, 3);
        r.expect_err(
            "An error should be raised if the x-coordinates of the corners are out of order",
        );
    }

    #[test]
    fn disordered_y() {
        let r = Rectangle::from_corners(5, 6, 5, 5);
        r.expect_err(
            "An error should be raised if the y-coordinates of the corners are out of order",
        );
    }
}

mod is_empty {
    use crate::image_utils::rectangle::Rectangle;
    use std::error::Error;

    #[test]
    fn empty_rect() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(0, 0, 0, 0)?;
        assert!(r.is_empty());
        Ok(())
    }

    #[test]
    fn empty_rect_x() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(1, 0, 1, 2)?;
        assert!(r.is_empty());
        Ok(())
    }

    #[test]
    fn empty_rect_y() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(0, 2, 1, 2)?;
        assert!(r.is_empty());
        Ok(())
    }

    #[test]
    fn nonempty_rect() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(0, 1, 1, 2)?;
        assert!(!r.is_empty());
        Ok(())
    }
}

mod eq {
    use crate::image_utils::rectangle::Rectangle;
    use std::error::Error;

    #[test]
    fn equal_self() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(1, 2, 3, 4)?;
        assert_eq!(r, r);
        Ok(())
    }

    #[test]
    fn equal_other() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(1, 2, 3, 4)?;
        let s = Rectangle::from_corners(1, 2, 3, 4)?;
        assert_eq!(r, s);
        Ok(())
    }

    #[test]
    fn equal_empty() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(1, 2, 1, 4)?;
        let s = Rectangle::from_corners(1, 2, 1, 4)?;
        assert_eq!(r, s);
        Ok(())
    }

    #[test]
    fn nonequal_empty() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(1, 2, 1, 4)?;
        let s = Rectangle::from_corners(1, 2, 2, 2)?;
        assert_ne!(r, s);
        Ok(())
    }

    #[test]
    fn nonequal_same_size() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(1, 2, 3, 4)?;
        let s = Rectangle::from_corners(2, 2, 4, 4)?;
        assert_ne!(r, s);
        Ok(())
    }

    #[test]
    fn nonequal_same_position() -> Result<(), Box<dyn Error>> {
        let r = Rectangle::from_corners(1, 2, 3, 4)?;
        let s = Rectangle::from_corners(1, 2, 4, 4)?;
        assert_ne!(r, s);
        Ok(())
    }
}
