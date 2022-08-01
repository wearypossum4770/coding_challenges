const std = @import("std");
pub fn addition(x: i32, y: i32) i33 {
    return x + y;
}

test "addition functions value" {
    try std.testing.expectEqual(addition(2, 3), 5);
    try std.testing.expectEqual(addition(-3, -6), -9);
    try std.testing.expectEqual(addition(7, 3), 10);
    try std.testing.expectEqual(addition(88, 2), 90);
}
