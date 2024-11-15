const c = @cImport({
    @cInclude("miniaudio.h");
});

pub fn main() void {
    var result: c.ma_result = undefined;
    var engine: c.ma_engine = undefined;

    result = c.ma_engine_init(null, &engine);
    if (result != c.MA_SUCCESS) {
        return;
    }

    const file = "./test.mp3";
    _ = c.ma_engine_play_sound(&engine, file, null);

    while (true) {}

    c.ma_engine_uninit(&engine);
}
