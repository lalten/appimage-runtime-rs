load("@rules_rust//rust:defs.bzl", "rust_binary")

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

rust_binary(
    name = "runtime_binary",
    srcs = ["src/bin/runtime.rs"],
    visibility = ["//tests:__subpackages__"],
    deps = [
        "//appimage-mount",
        "//appimage-runtime",
        "@crates//:exec",
        "@crates//:glob",
        "@crates//:tempfile",
    ],
)
