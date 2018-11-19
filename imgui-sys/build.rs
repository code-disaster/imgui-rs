extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("third-party/cimgui/cimgui/cimgui.cpp")
        .file("third-party/cimgui/cimgui/fontAtlas.cpp")
        .file("third-party/cimgui/cimgui/drawList.cpp")
        .file("third-party/cimgui/cimgui/listClipper.cpp")
        //.file("third-party/cimgui/imgui/imgui.cpp")
        //.file("third-party/cimgui/imgui/imgui_demo.cpp")
        .file("third-party/cimgui/imgui/imgui_draw.cpp")
        // use patched files for compiling with x86_64-pc-windows-gnu -->
        .file("third-party.patch/imgui.cpp")
        .file("third-party.patch/imgui_demo.cpp")
        .flag_if_supported("-Wno-class-memaccess")
        // <-- patch
        .compile("libcimgui.a");
}
