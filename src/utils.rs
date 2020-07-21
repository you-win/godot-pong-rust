#[allow(dead_code)]
pub fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    // Don't do anything if they are equal
    if start == end {
        return start;
    }
    let lerp_amount = (start - end).abs() * amount;
    if start < end {
        start + lerp_amount
    } else {
        start - lerp_amount
    }
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    let mut x = value;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}
