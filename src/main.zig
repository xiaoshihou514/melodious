const std = @import("std");
const vaxis = @import("vaxis");
const vxfw = vaxis.vxfw;

const State = @import("state.zig").State;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // enter alt screen
    var tty = try vaxis.Tty.init();
    defer tty.deinit();
    var vx = try vaxis.init(allocator, .{});
    defer vx.deinit(allocator, tty.anyWriter());
    try vx.enterAltScreen(tty.anyWriter());

    // run event loop
    var app = try vxfw.App.init(allocator);
    errdefer app.deinit();

    const state = try allocator.create(State);
    defer allocator.destroy(state);

    state.init(allocator);

    try app.run(state.widget(), .{});
    app.deinit();
}
