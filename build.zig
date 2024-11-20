const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // libvaxis
    const vaxis_dep = b.dependency("vaxis", .{
        .target = target,
        .optimize = optimize,
    });

    const exe = b.addExecutable(.{
        .name = "melodious",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    exe.root_module.addImport("vaxis", vaxis_dep.module("vaxis"));

    // libmpv
    exe.linkLibC();
    exe.linkSystemLibrary("mpv");
    // pkg-config --libs --cflags mpv
    // zig fmt: off
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
    // zig fmt: on

    b.installArtifact(exe);
}
