extern crate cc;

fn main() {
    cc::Build::new()
        .file("bsdiff-43/bsdiff.c")
        .file("bsdiff-43/bspatch.c")
        .static_flag(true)
        .compile("bsdiff");
}
