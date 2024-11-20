const vxfw = @import("vaxis").vxfw;

const Model = @import("model.zig").Model;
const View = @import("view.zig").View;
const Controller = @import("controller.zig").Controller;

pub const State = struct {
    model: Model,
    view: View,
    controller: Controller,

    pub fn init(self: *State) void {
        _ = self;
    }

    pub fn widget(self: *State) vxfw.Widget {
        return .{
            .userdata = self,
            .eventHandler = Controller.eventHandler,
            .drawFn = View.draw,
        };
    }
};
