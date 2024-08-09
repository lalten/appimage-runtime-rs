load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test_suite")

rust_library(
    name = "appimage-mount",
    srcs = [
        "src/lib.rs",
        "src/mount.rs",
    ],
    data = ["@squashfuse//:ll_main"],
    rustc_env = {"COMPILE_DATA_PATH": "$(execpath @squashfuse//:ll_main)"},
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:memfd-exec",
        "@crates//:nix",
        "@crates//:selecting",
        "@crates//:tempfile",
    ],
)

rust_test_suite(
    name = "integration",
    timeout = "short",
    srcs = ["tests/mount_test.rs"],
    tags = ["requires-fakeroot"],
    deps = [
        ":appimage-mount",
        "@crates//:backhand",
        "@crates//:tempfile",
    ],
)
