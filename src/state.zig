const vxfw = @import("vaxis").vxfw;
const std = @import("std");

const update = @import("update.zig").update;
const draw = @import("view.zig").draw;

pub const State = struct {
    // UI components
    songs_view: vxfw.ListView,
    search_bar: vxfw.TextField,

    // State
    music_dir: []u8,
    songs: std.ArrayList(vxfw.Text),
    filtered_songs: std.ArrayList(vxfw.RichText),

    // Misc
    allocator: std.mem.Allocator,
    arena: std.heap.ArenaAllocator,

    pub fn init(self: *State, allocator: std.mem.Allocator) void {
        self.allocator = allocator;
        self.arena = std.heap.ArenaAllocator.init(allocator);
    }

    pub fn widget(self: *State) vxfw.Widget {
        return .{
            .userdata = self,
            .eventHandler = update,
            .drawFn = draw,
        };
    }

    fn deinit() void {
        // cleanup memory
    }
};
