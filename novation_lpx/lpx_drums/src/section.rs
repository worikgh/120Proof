/// A `Section` is a collection of pads on a LPX that constitutes on
/// "drum".  ALl the pads in it ar one colour and do the same thing
use crate::lpx_drum_error::LpxDrumError;
#[allow(unused)]
struct Section {
    pad: usize,    // 11-88
    width: usize,  // 0-7
    height: usize, // 0-7
    main_colour: [usize; 3],
    active_colour: [usize; 3],
}

impl Section {
    /// FIXME: This must validate and return an error for invalid values
    #[allow(unused)]
    pub fn new(
        pad: usize,
        width: usize,
        height: usize,
        main_colour: [usize; 3],
        active_colour: [usize; 3],
    ) -> Result<Self, LpxDrumError> {
        // -> Result<Self, LpxDrumError>
        let result = Self {
            pad,
            width,
            height,
            main_colour,
            active_colour,
        };
        if result.valid() {
            // Ok(result)
            Ok(result)
        } else {
            // Err(LpxDrumError::invalid_section)
            Err(LpxDrumError::InvalidSection)
        }
    }

    /// Check the constraints on a `Section`:
    /// `pad` must be valid
    /// Width and height MUST BE VALID    
    fn valid(&self) -> bool {
        !(self.pad < 11
          || self.pad > 88
          || self.row() == 0
          || self.row() >= 9
          || self.col() == 0
          || self.col() >= 9
          
          // `width` and `height` are set so a single pad has width ==
          // height == 1, not zero
          || self.col() + self.width - 1 > 8
          || self.row() + self.height - 1 > 8
          
          || !self.main_colour.iter().all(|x| x <= &127)
          || !self.active_colour.iter().all(|x| x <= &127))
    }

    #[allow(dead_code)]
    fn intersect(&self, other: &Self) -> bool {
        let self_x = self.pad / 10; //  5
        let self_y = self.pad % 10; //  4
        let other_x = other.pad / 10; //  5
        let other_y = other.pad % 10; //  4

        !(self_y + self.height < other_y
            || self_y > other_y + other.height
            || self_x + self.width < other_x
            || self_x > other_x + other.width)
    }

    fn row(&self) -> usize {
        self.pad / 10
    }
    fn col(&self) -> usize {
        self.pad % 10
    }
}

use std::fmt;
#[allow(unused)]
impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Section - Pad: {}, Width: {}, Height: {}, Main Colour:[{}, {}, {}], Active Colour: [{}, {}, {}]",
            self.pad,
            self.width,
            self.height,
            self.main_colour[0],
            self.main_colour[1],
            self.main_colour[2],
            self.active_colour[0],
            self.active_colour[1],
            self.active_colour[2]
        )
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid() {
        // this is an invalid pad (1).
        let test = move || -> Result<Section, LpxDrumError> {
            let section = Section::new(1, 1, 1, [0, 0, 0], [0, 0, 0])?;
            Ok(section)
        };
        assert!(test().is_err());

        // this is an invalid main_colour (128).
        let test = move || -> Result<Section, LpxDrumError> {
            let section = Section::new(11, 1, 1, [128, 0, 0], [0, 0, 0])?;
            Ok(section)
        };
        let test = test();
        assert!(test.is_err());

        // this is an invalid activate_colour (128).
        let test = move || -> Result<Section, LpxDrumError> {
            let section = Section::new(11, 1, 1, [0, 0, 0], [0, 128, 0])?;
            Ok(section)
        };
        assert!(test().is_err());
        // this is a valid section
        let test = move || -> Result<Section, LpxDrumError> {
            let section = Section::new(11, 1, 1, [0, 0, 0], [0, 12, 0])?;
            Ok(section)
        };
        assert!(test().is_ok());
    }

    #[test]
    fn test_intersect() {
        // Test two sections that intersect
        let test = move || -> Result<bool, LpxDrumError> {
            let section_1 = Section::new(11, 8, 8, [0, 0, 0], [0, 0, 0])?;
            let section_2 = Section::new(11, 8, 8, [0, 0, 0], [0, 0, 0])?;
            Ok(section_1.intersect(&section_2))
        };
        let test = test();
        assert!(test.is_ok());
        assert!(test.unwrap());

        // Two that do not
        let test = move || -> Result<bool, LpxDrumError> {
            let section_1 = Section::new(11, 4, 3, [0, 0, 0], [0, 0, 0])?;
            let section_2 = Section::new(15, 3, 3, [0, 0, 0], [0, 0, 0])?;
            Ok(section_1.intersect(&section_2))
        };
        let test = test();
        assert!(test.is_ok());
        assert!(!test.unwrap());
    }
}
