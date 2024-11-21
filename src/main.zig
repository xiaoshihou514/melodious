const std = @import("std");
const vxfw = @import("vaxis").vxfw;

const State = @import("state.zig").State;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var app = try vxfw.App.init(allocator);
    errdefer app.deinit();

    const state = try allocator.create(State);
    defer allocator.destroy(state);

    state.init(allocator);

    try app.run(state.widget(), .{});
    app.deinit();
}
