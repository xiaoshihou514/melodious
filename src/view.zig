const vxfw = @import("vaxis").vxfw;
const State = @import("state.zig").State;
const std = @import("std");

pub const Focus = enum { songs, search_bar };

pub fn draw(ptr: *anyopaque, ctx: vxfw.DrawContext) std.mem.Allocator.Error!vxfw.Surface {
    const state: *State = @ptrCast(@alignCast(ptr));
    _ = state.arena.reset(.free_all);

    const max = ctx.max.size();

    var list_view: vxfw.SubSurface = .{
        .origin = .{ .row = 2, .col = 0 },
        .surface = try state.songs_view.draw(ctx.withConstraints(
            ctx.min,
            .{ .width = max.width, .height = max.height - 3 },
        )),
    };
    list_view.surface.focusable = false;

    const text_field: vxfw.SubSurface = .{
        .origin = .{ .row = 0, .col = 2 },
        .surface = try state.search_bar.draw(ctx.withConstraints(
            ctx.min,
            .{ .width = max.width, .height = 1 },
        )),
    };

    const children = try ctx.arena.alloc(vxfw.SubSurface, 3);
    children[0] = list_view;
    children[1] = text_field;

    return .{
        .size = max,
        .widget = state.widget(),
        .focusable = true,
        .buffer = &.{},
        .children = children,
    };
}
