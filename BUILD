load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "runtime",
    srcs = ["src/bin/runtime.rs"],
    visibility = ["//visibility:public"],
    deps = [
        "//appimage-mount",
        "//appimage-runtime",
        "@crates//:exec",
    ],
)

sh_test(
    name = "integration_test",
    timeout = "short",
    srcs = ["tests/integration_test.sh"],
    data = [
        ":runtime",
        "@squashfs-tools//:mksquashfs",
    ],
    env = {"MKSQUASHFS": "$(rootpath @squashfs-tools//:mksquashfs)"},
    tags = ["requires-fakeroot"],
)
