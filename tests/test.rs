#![allow(clippy::disallowed_methods)]

use crate::support::Test;

mod support;

// Some tests check that a flag is *not* present.  These tests might fail if the flag is set in the
// CFLAGS or CXXFLAGS environment variables.  This function clears the CFLAGS and CXXFLAGS
// variables to make sure that the tests can run correctly.
fn reset_env() {
    std::env::set_var("CFLAGS", "");
    std::env::set_var("CXXFLAGS", "");
}

#[test]
fn gnu_smoke() {
    reset_env();

    let test = Test::gnu();
    test.gcc().file("foo.c").compile("foo");

    test.cmd(0)
        .must_have("-O2")
        .must_have("foo.c")
        .must_not_have("-gdwarf-4")
        .must_have("-c")
        .must_have("-ffunction-sections")
        .must_have("-fdata-sections");
    test.cmd(1)
        .must_have(test.td.path().join("db3b6bfb95261072-foo.o"));
}

#[test]
fn gnu_opt_level_1() {
    reset_env();

    let test = Test::gnu();
    test.gcc().opt_level(1).file("foo.c").compile("foo");

    test.cmd(0).must_have("-O1").must_not_have("-O2");
}

#[test]
fn gnu_opt_level_s() {
    reset_env();

    let test = Test::gnu();
    test.gcc().opt_level_str("s").file("foo.c").compile("foo");

    test.cmd(0)
        .must_have("-Os")
        .must_not_have("-O1")
        .must_not_have("-O2")
        .must_not_have("-O3")
        .must_not_have("-Oz");
}

#[test]
fn gnu_debug() {
    let test = Test::gnu();
    test.gcc()
        .target("x86_64-unknown-linux-none")
        .debug(true)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-gdwarf-4");

    let test = Test::gnu();
    test.gcc()
        .target("x86_64-apple-darwin")
        .debug(true)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-gdwarf-2");

    let test = Test::gnu();
    test.gcc()
        .target("e2k-unknown-linux-gnu")
        .debug(true)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-gdwarf-2");
}

#[test]
fn gnu_debug_fp_auto() {
    let test = Test::gnu();
    test.gcc()
        .target("x86_64-unknown-linux-none")
        .debug(true)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-gdwarf-4");
    test.cmd(0).must_have("-fno-omit-frame-pointer");
}

#[test]
fn gnu_debug_fp() {
    let test = Test::gnu();
    test.gcc()
        .target("x86_64-unknown-linux-none")
        .debug(true)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-gdwarf-4");
    test.cmd(0).must_have("-fno-omit-frame-pointer");
}

#[test]
fn gnu_debug_nofp() {
    reset_env();

    let test = Test::gnu();
    test.gcc()
        .target("x86_64-unknown-linux-none")
        .debug(true)
        .force_frame_pointer(false)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-gdwarf-4");
    test.cmd(0).must_not_have("-fno-omit-frame-pointer");

    let test = Test::gnu();
    test.gcc()
        .target("x86_64-unknown-linux-none")
        .force_frame_pointer(false)
        .debug(true)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-gdwarf-4");
    test.cmd(0).must_not_have("-fno-omit-frame-pointer");
}

#[test]
fn gnu_warnings_into_errors() {
    let test = Test::gnu();
    test.gcc()
        .warnings_into_errors(true)
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_have("-Werror");
}

#[test]
fn gnu_warnings() {
    let test = Test::gnu();
    test.gcc()
        .warnings(true)
        .flag("-Wno-missing-field-initializers")
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_have("-Wall").must_have("-Wextra");
}

#[test]
fn gnu_extra_warnings0() {
    reset_env();

    let test = Test::gnu();
    test.gcc()
        .warnings(true)
        .extra_warnings(false)
        .flag("-Wno-missing-field-initializers")
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_have("-Wall").must_not_have("-Wextra");
}

#[test]
fn gnu_extra_warnings1() {
    reset_env();

    let test = Test::gnu();
    test.gcc()
        .warnings(false)
        .extra_warnings(true)
        .flag("-Wno-missing-field-initializers")
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_not_have("-Wall").must_have("-Wextra");
}

#[test]
fn gnu_warnings_overridable() {
    reset_env();

    let test = Test::gnu();
    test.gcc()
        .warnings(true)
        .flag("-Wno-missing-field-initializers")
        .file("foo.c")
        .compile("foo");

    test.cmd(0)
        .must_have_in_order("-Wall", "-Wno-missing-field-initializers");
}

#[test]
fn gnu_x86_64() {
    for vendor in &["unknown-linux-gnu", "apple-darwin"] {
        let target = format!("x86_64-{}", vendor);
        let test = Test::gnu();
        test.gcc()
            .target(&target)
            .host(&target)
            .file("foo.c")
            .compile("foo");

        test.cmd(0).must_have("-fPIC").must_have("-m64");
    }
}

#[test]
fn gnu_x86_64_no_pic() {
    reset_env();

    for vendor in &["unknown-linux-gnu", "apple-darwin"] {
        let target = format!("x86_64-{}", vendor);
        let test = Test::gnu();
        test.gcc()
            .pic(false)
            .target(&target)
            .host(&target)
            .file("foo.c")
            .compile("foo");

        test.cmd(0).must_not_have("-fPIC");
    }
}

#[test]
fn gnu_i686() {
    for vendor in &["unknown-linux-gnu", "apple-darwin"] {
        let target = format!("i686-{}", vendor);
        let test = Test::gnu();
        test.gcc()
            .target(&target)
            .host(&target)
            .file("foo.c")
            .compile("foo");

        test.cmd(0).must_have("-m32");
    }
}

#[test]
fn gnu_i686_pic() {
    for vendor in &["unknown-linux-gnu", "apple-darwin"] {
        let target = format!("i686-{}", vendor);
        let test = Test::gnu();
        test.gcc()
            .pic(true)
            .target(&target)
            .host(&target)
            .file("foo.c")
            .compile("foo");

        test.cmd(0).must_have("-fPIC");
    }
}

#[test]
fn gnu_x86_64_no_plt() {
    let target = "x86_64-unknown-linux-gnu";
    let test = Test::gnu();
    test.gcc()
        .pic(true)
        .use_plt(false)
        .target(target)
        .host(target)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-fno-plt");
}

#[test]
fn gnu_aarch64_none_no_pic() {
    for target in &["aarch64-unknown-none-softfloat", "aarch64-unknown-none"] {
        let test = Test::gnu();
        test.gcc()
            .target(target)
            .host(target)
            .file("foo.c")
            .compile("foo");

        test.cmd(0).must_not_have("-fPIC");
    }
}

#[test]
fn gnu_uefi_no_pic() {
    reset_env();

    for arch in &["aarch64", "i686", "x86_64"] {
        let target = format!("{}-unknown-uefi", arch);
        let test = Test::gnu();
        test.gcc()
            .target(&target)
            .host(&target)
            .file("foo.c")
            .compile("foo");

        test.cmd(0).must_not_have("-fPIC");
    }
}

#[test]
fn gnu_set_stdlib() {
    reset_env();

    let test = Test::gnu();
    test.gcc()
        .cpp_set_stdlib(Some("foo"))
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_not_have("-stdlib=foo");
}

#[test]
fn gnu_include() {
    let test = Test::gnu();
    test.gcc().include("foo/bar").file("foo.c").compile("foo");

    test.cmd(0).must_have("-I").must_have("foo/bar");
}

#[test]
fn gnu_define() {
    let test = Test::gnu();
    test.gcc()
        .define("FOO", "bar")
        .define("BAR", None)
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_have("-DFOO=bar").must_have("-DBAR");
}

#[test]
fn gnu_compile_assembly() {
    let test = Test::gnu();
    test.gcc().file("foo.S").compile("foo");
    test.cmd(0).must_have("foo.S");
}

#[test]
fn gnu_shared() {
    reset_env();

    let test = Test::gnu();
    test.gcc()
        .file("foo.c")
        .shared_flag(true)
        .static_flag(false)
        .compile("foo");

    test.cmd(0).must_have("-shared").must_not_have("-static");
}

#[test]
fn gnu_flag_if_supported() {
    reset_env();

    if cfg!(windows) {
        return;
    }
    let test = Test::gnu();
    test.gcc()
        .file("foo.c")
        .flag("-v")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wflag-does-not-exist")
        .flag_if_supported("-std=c++11")
        .compile("foo");

    test.cmd(0)
        .must_have("-v")
        .must_have("-Wall")
        .must_not_have("-Wflag-does-not-exist")
        .must_not_have("-std=c++11");
}

#[cfg(not(windows))]
#[test]
fn gnu_flag_if_supported_cpp() {
    let test = Test::gnu();
    test.gcc()
        .cpp(true)
        .file("foo.cpp")
        .flag_if_supported("-std=c++11")
        .compile("foo");

    test.cmd(0).must_have("-std=c++11");
}

#[test]
fn gnu_static() {
    reset_env();

    let test = Test::gnu();
    test.gcc()
        .file("foo.c")
        .shared_flag(false)
        .static_flag(true)
        .compile("foo");

    test.cmd(0).must_have("-static").must_not_have("-shared");
}

#[test]
fn gnu_no_dash_dash() {
    let test = Test::gnu();
    test.gcc().file("foo.c").compile("foo");

    test.cmd(0).must_not_have("--");
}

#[test]
fn gnu_std_c() {
    let test = Test::gnu();
    test.gcc().file("foo.c").std("c11").compile("foo");

    test.cmd(0).must_have("-std=c11");
}

#[test]
fn msvc_smoke() {
    reset_env();

    let test = Test::msvc();
    test.gcc().file("foo.c").compile("foo");

    test.cmd(0)
        .must_have("-O2")
        .must_have("foo.c")
        .must_not_have("-Z7")
        .must_have("-c")
        .must_have("-MD");
    test.cmd(1)
        .must_have(test.td.path().join("db3b6bfb95261072-foo.o"));
}

#[test]
fn msvc_opt_level_0() {
    reset_env();

    let test = Test::msvc();
    test.gcc().opt_level(0).file("foo.c").compile("foo");

    test.cmd(0).must_not_have("-O2");
}

#[test]
fn msvc_debug() {
    let test = Test::msvc();
    test.gcc().debug(true).file("foo.c").compile("foo");
    test.cmd(0).must_have("-Z7");
}

#[test]
fn msvc_include() {
    let test = Test::msvc();
    test.gcc().include("foo/bar").file("foo.c").compile("foo");

    test.cmd(0).must_have("-I").must_have("foo/bar");
}

#[test]
fn msvc_define() {
    let test = Test::msvc();
    test.gcc()
        .define("FOO", "bar")
        .define("BAR", None)
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_have("-DFOO=bar").must_have("-DBAR");
}

#[test]
fn msvc_static_crt() {
    let test = Test::msvc();
    test.gcc().static_crt(true).file("foo.c").compile("foo");

    test.cmd(0).must_have("-MT");
}

#[test]
fn msvc_no_static_crt() {
    let test = Test::msvc();
    test.gcc().static_crt(false).file("foo.c").compile("foo");

    test.cmd(0).must_have("-MD");
}

#[test]
fn msvc_no_dash_dash() {
    let test = Test::msvc();
    test.gcc().file("foo.c").compile("foo");

    test.cmd(0).must_not_have("--");
}

#[test]
fn msvc_std_c() {
    let test = Test::msvc();
    test.gcc().file("foo.c").std("c11").compile("foo");

    test.cmd(0).must_have("-std:c11");
}

// Disable this test with the parallel feature because the execution
// order is not deterministic.
#[cfg(not(feature = "parallel"))]
#[test]
fn asm_flags() {
    let test = Test::gnu();
    test.gcc()
        .file("foo.c")
        .file("x86_64.asm")
        .file("x86_64.S")
        .asm_flag("--abc")
        .compile("foo");
    test.cmd(0).must_not_have("--abc");
    test.cmd(1).must_have("--abc");
    test.cmd(2).must_have("--abc");
}

#[test]
fn gnu_apple_sysroot() {
    let targets = ["aarch64-apple-darwin", "x86_64-apple-darwin"];

    for target in targets {
        let test = Test::gnu();
        test.shim("fake-gcc")
            .gcc()
            .compiler("fake-gcc")
            .target(target)
            .host(target)
            .file("foo.c")
            .compile("foo");

        let cmd = test.cmd(0);
        cmd.must_not_have("-isysroot");
    }
}

#[test]
#[cfg(target_os = "macos")] // Invokes xcrun
fn gnu_apple_arch() {
    let cases = [
        ("x86_64-apple-darwin", "x86_64"),
        ("x86_64h-apple-darwin", "x86_64h"),
        ("aarch64-apple-darwin", "arm64"),
        ("arm64e-apple-darwin", "arm64e"),
        ("i686-apple-darwin", "i386"),
        ("aarch64-apple-ios", "arm64"),
        ("armv7s-apple-ios", "armv7s"),
        ("arm64_32-apple-watchos", "arm64_32"),
        ("armv7k-apple-watchos", "armv7k"),
    ];

    for (target, arch) in cases {
        let test = Test::gnu();
        test.shim("fake-gcc")
            .gcc()
            .compiler("fake-gcc")
            .target(&target)
            .host(&"aarch64-apple-darwin")
            .file("foo.c")
            .compile("foo");

        let cmd = test.cmd(0);
        cmd.must_have_in_order("-arch", arch);
    }
}

#[test]
#[cfg(target_os = "macos")] // Invokes xcrun
fn gnu_apple_deployment_target() {
    let cases = [
        ("x86_64-apple-darwin", "-mmacosx-version-min=10.12"),
        ("aarch64-apple-darwin", "-mmacosx-version-min=10.12"),
        ("aarch64-apple-ios", "-miphoneos-version-min=10.0"),
        ("aarch64-apple-ios-sim", "-mios-simulator-version-min=10.0"),
        ("x86_64-apple-ios", "-mios-simulator-version-min=10.0"),
        ("aarch64-apple-ios-macabi", "-mtargetos=ios10.0-macabi"),
        ("aarch64-apple-tvos", "-mappletvos-version-min=10.0"),
        (
            "aarch64-apple-tvos-sim",
            "-mappletvsimulator-version-min=10.0",
        ),
        ("aarch64-apple-watchos", "-mwatchos-version-min=5.0"),
        (
            "aarch64-apple-watchos-sim",
            "-mwatchsimulator-version-min=5.0",
        ),
        ("aarch64-apple-visionos", "-mtargetos=xros1.0"),
        ("aarch64-apple-visionos-sim", "-mtargetos=xros1.0-simulator"),
    ];

    for (target, os_version_flag) in cases {
        let test = Test::gnu();
        test.shim("fake-gcc")
            .gcc()
            .compiler("fake-gcc")
            .target(&target)
            .host(&"aarch64-apple-darwin")
            // Avoid dependency on environment in test.
            .__set_env("MACOSX_DEPLOYMENT_TARGET", "10.12")
            .__set_env("IPHONEOS_DEPLOYMENT_TARGET", "10.0")
            .__set_env("TVOS_DEPLOYMENT_TARGET", "10.0")
            .__set_env("WATCHOS_DEPLOYMENT_TARGET", "5.0")
            .__set_env("XROS_DEPLOYMENT_TARGET", "1.0")
            .file("foo.c")
            .compile("foo");

        let cmd = test.cmd(0);
        cmd.must_have(os_version_flag);
    }
}

#[cfg(target_os = "macos")]
#[test]
fn macos_cpp_minimums() {
    let versions = &[
        // Too low
        ("10.7", (10, 9)),
        // Minimum
        ("10.9", (10, 9)),
        // Higher
        ("11.0", (11, 0)),
    ];

    let target = "x86_64-apple-darwin";
    for (deployment_target, set_version) in versions {
        let test = Test::gnu();
        test.gcc()
            .target(target)
            .host(target)
            .cpp(true)
            .__set_env("MACOSX_DEPLOYMENT_TARGET", deployment_target)
            .file("foo.c")
            .compile("foo");

        let exec = test.cmd(0);
        let deployment_arg = exec
            .args
            .iter()
            .find_map(|arg| arg.strip_prefix("-mmacosx-version-min="))
            .expect("no deployment target argument was set");

        let mut deployment_parts = deployment_arg.split('.').map(|v| v.parse::<u32>().unwrap());

        let major = deployment_parts.next().unwrap();
        let minor = deployment_parts.next().unwrap();

        // Check that we are on at least our minimums since this test reads from system
        // SDK state, and that can vary per-system. It should never go lower then the deployment
        // target we pass.
        assert!(major >= set_version.0);

        // If still on 10.x make sure `x` didn't go lower.
        if major == set_version.0 {
            assert!(minor >= set_version.1);
        }
    }

    let test = Test::gnu();
    test.gcc()
        .target(target)
        .host(target)
        .__set_env("MACOSX_DEPLOYMENT_TARGET", "10.7")
        .file("foo.c")
        .compile("foo");

    // No C++ leaves it untouched
    test.cmd(0).must_have("-mmacosx-version-min=10.7");
}

#[cfg(target_os = "macos")]
#[test]
fn clang_apple_tvos() {
    let target = "aarch64-apple-tvos";
    let test = Test::clang();
    test.gcc()
        .__set_env("TVOS_DEPLOYMENT_TARGET", "9.0")
        .target(target)
        .host(target)
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_have("--target=arm64-apple-tvos");
    test.cmd(0).must_have("-mappletvos-version-min=9.0");
}

#[cfg(target_os = "macos")]
#[test]
fn clang_apple_mac_catalyst() {
    let output = std::process::Command::new("xcrun")
        .args(["--show-sdk-path", "--sdk", "macosx"])
        .output()
        .unwrap();
    if !output.status.success() {
        return;
    }
    let sdkroot = std::str::from_utf8(&output.stdout).unwrap().trim();

    let test = Test::clang();
    test.gcc()
        .target("aarch64-apple-ios-macabi")
        .__set_env("IPHONEOS_DEPLOYMENT_TARGET", "15.0")
        .file("foo.c")
        .compile("foo");
    let execution = test.cmd(0);

    execution.must_have("--target=arm64-apple-ios15.0-macabi");
    // --target and -mtargetos= don't mix
    execution.must_not_have("-mtargetos=");
    execution.must_have_in_order("-isysroot", sdkroot);
    execution.must_have_in_order(
        "-isystem",
        &format!("{sdkroot}/System/iOSSupport/usr/include"),
    );
    execution.must_have_in_order(
        "-iframework",
        &format!("{sdkroot}/System/iOSSupport/System/Library/Frameworks"),
    );
    execution.must_have(format!("-L{sdkroot}/System/iOSSupport/usr/lib"));
    execution.must_have(format!(
        "-F{sdkroot}/System/iOSSupport/System/Library/Frameworks"
    ));
}

#[cfg(target_os = "macos")]
#[test]
fn clang_apple_tvsimulator() {
    let target = "x86_64-apple-tvos";

    let test = Test::clang();
    test.gcc()
        .__set_env("TVOS_DEPLOYMENT_TARGET", "9.0")
        .target(target)
        .host(target)
        .file("foo.c")
        .compile("foo");

    test.cmd(0)
        .must_have("--target=x86_64-apple-tvos-simulator");
    test.cmd(0).must_have("-mappletvsimulator-version-min=9.0");
}

#[cfg(target_os = "macos")]
#[test]
fn clang_apple_visionos() {
    // Only run this test if visionOS is available on the host machine
    let output = std::process::Command::new("xcrun")
        .args(["--show-sdk-version", "--sdk", "xros"])
        .output()
        .unwrap();
    if !output.status.success() {
        return;
    }

    let test = Test::clang();
    test.gcc()
        .__set_env("XROS_DEPLOYMENT_TARGET", "1.0")
        .target("aarch64-apple-visionos")
        .file("foo.c")
        .compile("foo");

    dbg!(test.cmd(0).args);

    test.cmd(0).must_have("--target=arm64-apple-xros1.0");
    // --target and -mtargetos= don't mix.
    test.cmd(0).must_not_have("-mtargetos=");

    // Flags that don't exist.
    test.cmd(0).must_not_have("-mxros-version-min=1.0");
    test.cmd(0).must_not_have("-mxrsimulator-version-min=1.0");
}

#[cfg(target_os = "macos")]
#[test]
fn apple_sdkroot_wrong() {
    let output = std::process::Command::new("xcrun")
        .args(["--show-sdk-path", "--sdk", "iphoneos"])
        .output()
        .unwrap();
    if !output.status.success() {
        return;
    }

    let wrong_sdkroot = "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk";
    let test = Test::clang();
    test.gcc()
        .__set_env("SDKROOT", wrong_sdkroot)
        .target("aarch64-apple-ios")
        .file("foo.c")
        .compile("foo");

    dbg!(test.cmd(0).args);

    test.cmd(0)
        .must_have(std::str::from_utf8(&output.stdout).unwrap().trim());
    test.cmd(0).must_not_have(wrong_sdkroot);
}

#[test]
fn compile_intermediates() {
    let test = Test::gnu();
    let intermediates = test
        .gcc()
        .file("foo.c")
        .file("x86_64.asm")
        .file("x86_64.S")
        .asm_flag("--abc")
        .compile_intermediates();

    assert_eq!(intermediates.len(), 3);

    assert!(intermediates[0].display().to_string().contains("foo"));
    assert!(intermediates[1].display().to_string().contains("x86_64"));
    assert!(intermediates[2].display().to_string().contains("x86_64"));
}

#[test]
fn clang_android() {
    let target = "arm-linux-androideabi";

    // On Windows, we don't use the Android NDK shims for Clang, so verify that
    // we use "clang" and set the target correctly.
    #[cfg(windows)]
    {
        let test = Test::new();
        test.shim("clang").shim("llvm-ar");
        test.gcc()
            .target(target)
            .host("x86_64-pc-windows-msvc")
            .file("foo.c")
            .compile("foo");
        test.cmd(0).must_have("--target=arm-linux-androideabi");
    }

    // On non-Windows, we do use the shims, so make sure that we use the shim
    // and don't set the target.
    #[cfg(not(windows))]
    {
        let test = Test::new();
        test.shim("arm-linux-androideabi-clang")
            .shim("arm-linux-androideabi-ar")
            .shim("llvm-ar");
        test.gcc().target(target).file("foo.c").compile("foo");
        test.cmd(0).must_not_have("--target=arm-linux-androideabi");
    }
}
