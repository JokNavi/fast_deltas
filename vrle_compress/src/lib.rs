pub mod encoder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_u16() {
        dbg!(format!("{:016b}", 0));
        dbg!(format!("{:016b}", 1));
        dbg!(format!("{:016b}", 2));
        dbg!(format!("{:016b}", 3));
        dbg!(format!("{:016b}", 255));
        dbg!(format!("{:016b}", 256));
    }
}
