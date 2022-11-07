const std = @import("std");

fn addition(x: i32, y: i32) i32 {
    return x + y;
}
test "returns the sum of two integers" {
    try std.testing.expect(addition(1, 2) == 3);
    try std.testing.expect(addition(2, 3) == 5);
    try std.testing.expect(addition(-3, -6) == -9);
    try std.testing.expect(addition(7, 3) == 10);
    try std.testing.expect(addition(88, 2) == 90);
}
