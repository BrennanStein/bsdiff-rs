extern crate cc;

fn main() {
    cc::Build::new()
        .file("bsdiff/bsdiff.c")
        .file("bsdiff/bspatch.c")
        .compile("bsdiff");
}
