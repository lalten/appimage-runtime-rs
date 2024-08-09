load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test_suite")

rust_library(
    name = "appimage-runtime",
    srcs = [
        "src/extract.rs",
        "src/lib.rs",
        "src/util.rs",
    ],
    visibility = ["//visibility:public"],
    deps = ["@crates//:elf"],
)

rust_test_suite(
    name = "integration",
    timeout = "short",
    srcs = ["tests/util_test.rs"],
    data = ["@squashfuse//:ll_main"],
    deps = [
        ":appimage-runtime",
        "@rules_rust//tools/runfiles",
    ],
)
