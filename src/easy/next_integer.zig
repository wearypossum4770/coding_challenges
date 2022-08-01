const std = @import("std");
pub fn nextInt(x: i33) i33 {
    return x + 1;
}

test "next integer is returned" {
    try std.testing.expectEqual(nextInt(2), 3); //"2 plus 1 equals 3.")
    try std.testing.expectEqual(nextInt(-9), -8); // "-8 plus 1 equals -9.")
    try std.testing.expectEqual(nextInt(0), 1); // "0 plus 1 equals 1.")
    try std.testing.expectEqual(nextInt(999), 1000); // "999 plus 1 equals 1000.")
    try std.testing.expectEqual(
        nextInt(73),
        74,
    ); // "73 plus 1 equals 74.")
}
