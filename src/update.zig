const vxfw = @import("vaxis").vxfw;
const State = @import("state.zig").State;

pub fn update(ptr: *anyopaque, ctx: *vxfw.EventContext, event: vxfw.Event) anyerror!void {
    _ = ctx;
    _ = event;
    const state: *State = @ptrCast(@alignCast(ptr));
    _ = state;
}
