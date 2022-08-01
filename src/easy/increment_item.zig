const std = @import("std");

pub fn incrementItems(arr: [_]i32) [_]i32 {
    for (&arr) |value, i| {
        arr[i] = value + 1;
    }
    return arr;
}

test "all elements in array increase by 1" {
    try std.testing.expectEqual(incrementItems(&.{ 0, 1, 2, 3 }), [_]i32{ 1, 2, 3, 4 });
    try std.testing.expectEqual(incrementItems(&.{ 2, 4, 6, 8 }), [_]i32{ 3, 5, 7, 9 });
    try std.testing.expectEqual(incrementItems(&.{ -1, -2, -3, -4 }), [_]i32{ 0, -1, -2, -3 });
}
