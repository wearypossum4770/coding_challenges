///
/// https://edabit.com/challenge/d9suvbchE2bnHNQuK
///
///
pub fn frames_per_second(minutes: u32, fps: u32) -> u32 {
    fps * 60 * minutes
}
#[cfg(test)]
mod tests {
    use super::frames_per_second;
    #[test]
    fn test_frames_per_second() {
        assert_eq!(frames_per_second(1, 1), 60);
        assert_eq!(frames_per_second(10, 1), 600);
        assert_eq!(frames_per_second(10, 25), 15000);
        assert_eq!(frames_per_second(500, 60), 1800000);
        assert_eq!(frames_per_second(0, 60), 0);
        assert_eq!(frames_per_second(99, 1), 5940);
        assert_eq!(frames_per_second(419, 70), 1759800);
        assert_eq!(frames_per_second(52, 33), 102960);
    }
}
