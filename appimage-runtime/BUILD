load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test", "rust_test_suite")

rust_library(
    name = "appimage-runtime",
    srcs = [
        "src/extract.rs",
        "src/lib.rs",
        "src/util.rs",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:anyhow",
        "@crates//:backhand",
        "@crates//:elf",
    ],
)

rust_test(
    name = "unit",
    timeout = "short",
    crate = ":appimage-runtime",
)

rust_test_suite(
    name = "integration",
    timeout = "short",
    srcs = [
        "tests/extract_test.rs",
        "tests/util_test.rs",
    ],
    data = [
        "//tests:test.sqfs",
        "@squashfuse//:ll_main",
    ],
    deps = [
        ":appimage-runtime",
        "@rules_rust//tools/runfiles",
    ],
)
