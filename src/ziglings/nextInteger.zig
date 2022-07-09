const std = @import("std");

fn nextInteger(num: i32) i32 {
    return num + 1;
}
test "returns the next integer from a given integer" {
    try std.testing.expect(nextInteger(41) == 42);
    try std.testing.expect(nextInteger(2) == 3);
    try std.testing.expect(nextInteger(-9) == -8);
    try std.testing.expect(nextInteger(0) == 1);
    try std.testing.expect(nextInteger(999) == 1000);
    try std.testing.expect(nextInteger(73) == 74);
}
