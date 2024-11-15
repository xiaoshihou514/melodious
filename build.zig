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

    // miniaudio
    // download miniaudio header file
    const miniaudioVersion = "0.11.21";
    const curl = b.addSystemCommand(&.{"curl"});
    curl.addArgs(&.{ b.fmt("https://raw.githubusercontent.com/mackron/miniaudio/refs/tags/{s}/miniaudio.h", .{miniaudioVersion}), "-o", "miniaudio.h", "-C", "-" });

    exe.step.dependOn(&curl.step);

    exe.linkLibC();
    exe.addIncludePath(.{ .cwd_relative = "." });
    exe.addCSourceFile(.{ .file = b.path("stub.c"), .flags = &.{ "-lpthread", "-lm", "-ldl", "-fno-sanitize=undefined" } });

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
    const lib_unit_tests = b.addTest(.{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
    });
    const run_lib_unit_tests = b.addRunArtifact(lib_unit_tests);
    const exe_unit_tests = b.addTest(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });
    const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_lib_unit_tests.step);
    test_step.dependOn(&run_exe_unit_tests.step);
}
