const std = @import("std");
pub fn minutesToSeconds(minutes: i32) i32 {
    return minutes * 60;
}

test "minites are converted to seconds" {
    try std.testing.expectEqual(minutesToSeconds(6), 360);
    try std.testing.expectEqual(minutesToSeconds(4), 240);
    try std.testing.expectEqual(minutesToSeconds(8), 480);
    try std.testing.expectEqual(minutesToSeconds(60), 3600);
}
