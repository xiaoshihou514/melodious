const c = @cImport({
    @cInclude("stddef.h");
    @cInclude("stdio.h");
    @cInclude("stdlib.h");
    @cInclude("client.h");
});

const std = @import("std");

pub fn main() void {
    const ctx: ?*c.mpv_handle = c.mpv_create();
    if (ctx == null) {
        _ = c.printf("failed creating context\n");
    }

    // Enable default key bindings, so the user can actually interact with
    // the player (and e.g. close the window).
    _ = c.mpv_set_option_string(ctx, "input-default-bindings", "yes");
    _ = c.mpv_set_option_string(ctx, "input-vo-keyboard", "yes");
    var val: c_int = 1;
    _ = c.mpv_set_option(ctx, "osc", c.MPV_FORMAT_FLAG, &val);
    _ = c.mpv_set_option_string(ctx, "video", "no");

    // Done setting up options.
    _ = c.mpv_initialize(ctx);

    // Play this file.
    var cmd: [3][*c]const u8 = [3][*c]const u8{
        "loadfile",
        "test.mp3",
        null,
    };
    _ = c.mpv_command(ctx, @as([*c][*c]const u8, @ptrCast(@alignCast(&cmd))));

    // Let it play, and wait until the user quits.
    while (true) {
        const event: *c.mpv_event = c.mpv_wait_event(ctx, 10000);
        _ = c.printf("event: %s\n", c.mpv_event_name(event.event_id));
        if (event.event_id == c.MPV_EVENT_SHUTDOWN)
            break;
    }

    c.mpv_terminate_destroy(ctx);
}
