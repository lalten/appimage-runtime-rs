genrule(
    name = "make_test_sqfs",
    outs = ["test.sqfs"],
    cmd = "\n".join([
        'cat <<"EOF" >AppRun',
        "#!/bin/sh",
        'echo args: "$$@"',
        "env | sort",
        "EOF",
        "chmod +x AppRun",
        "mkdir -p other/path",
        "echo 'I am data' > other/path/file.txt",
        "export SOURCE_DATE_EPOCH=1424879120",
        "$(execpath @squashfs-tools//:mksquashfs) AppRun other/path/file.txt $@" +
        " -quiet -no-progress -no-strip -root-owned -noappend -comp zstd",
    ]),
    tools = ["@squashfs-tools//:mksquashfs"],
    visibility = ["//:__subpackages__"],
)

genrule(
    name = "make_test_appimage",
    srcs = [
        "//:runtime",
        ":test.sqfs",
    ],
    outs = ["test.appimage"],
    cmd = "cat $(execpath //:runtime) $(execpath :test.sqfs) >$@",
    executable = True,
)

sh_test(
    name = "args_test",
    timeout = "short",
    srcs = ["args_test.sh"],
    data = [":test.appimage"],
    env = {"TEST_APPIMAGE": "$(rootpath :test.appimage)"},
)

sh_test(
    name = "extract_test",
    timeout = "short",
    srcs = ["extract_test.sh"],
    data = [":test.appimage"],
    env = {"TEST_APPIMAGE": "$(rootpath :test.appimage)"},
)

sh_test(
    name = "extract_and_run_test",
    timeout = "short",
    srcs = ["extract_and_run_test.sh"],
    data = [":test.appimage"],
    env = {"TEST_APPIMAGE": "$(rootpath :test.appimage)"},
)

sh_test(
    name = "list_test",
    timeout = "short",
    srcs = ["list_test.sh"],
    data = [":test.appimage"],
    env = {"TEST_APPIMAGE": "$(rootpath :test.appimage)"},
)

sh_test(
    name = "magic_test",
    timeout = "short",
    srcs = ["magic_test.sh"],
    data = [
        "//:runtime",
        "//:runtime_binary",
    ],
    env = {
        "RUNTIME": "$(rootpath //:runtime)",
        "RUNTIME_BIN": "$(rootpath //:runtime_binary)",
    },
)

sh_test(
    name = "mount_test",
    timeout = "short",
    srcs = ["mount_test.sh"],
    data = [":test.appimage"],
    env = {"TEST_APPIMAGE": "$(rootpath :test.appimage)"},
    tags = ["requires-fakeroot"],
)

sh_test(
    name = "mount_and_run_test",
    timeout = "short",
    srcs = ["mount_and_run_test.sh"],
    data = [":test.appimage"],
    env = {"TEST_APPIMAGE": "$(rootpath :test.appimage)"},
    tags = ["requires-fakeroot"],
)

sh_test(
    name = "print_offset_test",
    timeout = "short",
    srcs = ["print_offset_test.sh"],
    data = [
        ":test.appimage",
        "//:runtime",
    ],
    env = {
        "TEST_APPIMAGE": "$(rootpath :test.appimage)",
        "RUNTIME_PATH": "$(rootpath //:runtime)",
    },
)
