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
