const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "melodious",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // libvaxis
    const vaxis_dep = b.dependency("vaxis", .{
        .target = target,
        .optimize = optimize,
    });
    exe.root_module.addImport("vaxis", vaxis_dep.module("vaxis"));

    // libmpv
    exe.linkLibC();
    exe.linkSystemLibrary("mpv");
    exe.addIncludePath(.{ .cwd_relative = "." });
    exe.addCSourceFile(.{ .file = b.path("stub.c"), .flags = &.{
        "-I/usr/include/freetype2",
        "-I/usr/include/vapoursynth",
        "-I/usr/include/spa-0.2",
        "-I/usr/include/fribidi",
        "-I/usr/include/cdio",
        "-I/usr/include/ffmpeg",
        "-I/usr/include/libpng16",
        "-I/usr/include/harfbuzz",
        "-I/usr/include/glib-2.0",
        "-I/usr/lib64/glib-2.0/include",
        "-I/usr/include/sysprof-6",
        "-pthread",
        "-I/usr/include/libxml2",
        "-I/usr/include/lua-5.1",
        "-I/usr/include/SDL2",
        "-I/usr/include/uchardet",
        "-I/usr/include/python3.12",
        "-DWITH_GZFILEOP",
        "-I/usr/include/AL",
        "-I/usr/include/pipewire-0.3",
        "-D_REENTRANT",
        "-I/usr/include/libdrm",
        "-I/usr/include/ffnvcodec",
        "-lmpv" }
    });

    // zig install
    const install_artifact = b.addInstallArtifact(exe, .{
        .dest_dir = .{ .override = .prefix },
    });
    b.getInstallStep().dependOn(&install_artifact.step);

    // zig run
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    // zig test
    const exe_unit_tests = b.addTest(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });
    const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_exe_unit_tests.step);
}
