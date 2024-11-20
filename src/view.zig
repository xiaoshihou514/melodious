const vaxis = @import("vaxis");
const std = @import("std");
const vxfw = vaxis.vxfw;
const State = @import("state.zig").State;

pub const View = struct {
    // TODO
    pub fn draw(ptr: *anyopaque, ctx: vxfw.DrawContext) std.mem.Allocator.Error!vxfw.Surface {
        const state: *State = @ptrCast(@alignCast(ptr));
        const children = try ctx.arena.alloc(vxfw.SubSurface, 3);

        return .{
            .size = ctx.max.size(),
            .widget = state.widget(),
            .focusable = true,
            .buffer = &.{},
            .children = children,
        };
    }
};
