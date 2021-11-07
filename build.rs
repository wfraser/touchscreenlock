fn main() {
    if cfg!(not(target_os = "windows")) {
        return;
    }

    // note: ProductVersion gets set by Cargo to the package version,
    // and FileVersion gets that suffixed with ".0".
    winres::WindowsResource::new()
        .set_icon("icon.ico")
        .set("CompanyName", "Wm. Fraser, Etc.")
        .set("FileDescription", "TouchLock prevents screen touches from having any effect while the program is in the foreground.")
        .set("InternalName", "touchlock.exe")
        .set("LegalCopyright", "Copyright (c) 2021 William R. Fraser")
        .set("OriginalFilename", "touchlock.exe")
        .set("ProductName", "TouchLock")
        .compile()
        .expect("failed to compile Windows resources");
}
