// Modificado por Grupo MOST para Most Anything. Obra original: Craft Anything del equipo ArtCraft (https://github.com/storytold/craft-anything), Apache-2.0.
//! Windows only: embed the app icon and version info (VERSIONINFO) into `craft-anything.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `MOST_ANYTHING_REQUIRE_WINRES=1` (set by
//! the release workflow) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/craft-anything.ico");
    println!("cargo:rerun-if-env-changed=MOST_ANYTHING_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/craft-anything.ico")
        .set("ProductName", "Most Anything")
        .set("FileDescription", "Most Anything: find out how any software works")
        .set("LegalCopyright", "Copyright (c) the ArtCraft Team and the Craft Anything contributors; modificaciones Copyright (c) 2026 Grupo MOST. MIT OR Apache-2.0.")
        .set("OriginalFilename", "craft-anything.exe")
        .set("InternalName", "craft-anything");
    if let Err(e) = res.compile() {
        if std::env::var_os("MOST_ANYTHING_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=craft-anything.exe built without icon/version resources: {e}");
    }
}
