use std::ops::Range;

pub fn clamp<T: PartialOrd + Copy>(range: &Range<T>, x: T) -> T {
    if x < range.start {
        range.start
    } else if x > range.end {
        range.end
    } else {
        x
    }
}
