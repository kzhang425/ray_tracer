use crate::vec3::Color;
use std::fmt::{Write, Error};

/// Writes a RGB value represented by a vector to a String buffer for output.
pub fn write_color(buf: &mut String, pixel_color: Color) -> Result<(), Error> {
    write!(buf, "{} {} {}", 
        (255.999 * pixel_color.x()) as i32, 
        (255.999 * pixel_color.y()) as i32, 
        (255.999 * pixel_color.z()) as i32)?;
    Ok(())
}

// Module tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3;

    #[test]
    fn cast() {
        let a = vector3!(1, 2, 3);
        let mut s = String::new();
        write_color(&mut s, a).unwrap();
        assert_eq!(s, String::from("255 511 767"));
    }
}