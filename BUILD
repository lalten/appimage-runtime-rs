load("@aspect_bazel_lib//lib:transitions.bzl", "platform_transition_binary")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "runtime_binary",
    srcs = ["src/bin/runtime.rs"],
    visibility = ["//tests:__subpackages__"],
    deps = [
        "//appimage-mount",
        "//appimage-runtime",
        "@crates//:exec",
        "@crates//:glob",
    ],
)

genrule(
    name = "add_magic",
    srcs = [":runtime_binary"],
    outs = ["runtime"],
    cmd = "\n".join([
        "head --bytes 8 $< >$@",
        "printf 'AI\\x02' >>$@",
        "tail --bytes +12 $< >>$@",
    ]),
    visibility = ["//visibility:public"],
)

TARGET_TRIPLES = [
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
]

[
    platform_transition_binary(
        name = "runtime_" + platform,
        binary = ":runtime",
        target_platform = "//platforms:" + platform,
        visibility = ["//visibility:public"],
    )
    for platform in TARGET_TRIPLES
]

filegroup(
    name = "runtimes",
    srcs = [":runtime_" + platform for platform in TARGET_TRIPLES],
    visibility = ["//visibility:public"],
)
