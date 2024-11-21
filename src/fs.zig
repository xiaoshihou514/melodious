const std = @import("std");
const vxfw = @import("vaxis").vxfw;

pub fn get_music_directory(allocator: std.mem.Allocator) ![]u8 {
    // use arena for func local stuff
    var arena_handle = std.heap.ArenaAllocator.init(allocator);
    const arena = arena_handle.allocator();
    defer arena_handle.deinit();

    const env = try arena.create(std.process.EnvMap);

    env.* = try std.process.getEnvMap(arena);

    const home = env.get("HOME") orelse try std.process.getCwdAlloc(arena);

    return try std.fs.path.join(allocator, &[_][]const u8{ home, "Music" });
}

pub fn add_songs(dest: std.ArrayList(vxfw.Text)) !void {
    _ = dest;
    suspend {}
}
