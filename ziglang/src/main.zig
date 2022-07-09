// https://github.com/ratfactor/ziglings/blob/main/exercises/001_hello.zig
// https://ziglearn.org/chapter-1/
// https://github.com/ziglang/zig/wiki/Zig-Newcomer-Programming-FAQs
// https://ziglang.org/documentation/master/#String-Literals-and-Unicode-Code-Point-Literals

const std = @import("std");
const Hello = @import("modules/hello.zig");

pub fn main() anyerror!void {
    Hello.Hello();
    std.log.info("All your codebase are belong to us.", .{});
}

test "basic test" {
    try std.testing.expectEqual(10, 3 + 7);
}
