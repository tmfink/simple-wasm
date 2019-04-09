use cc;

fn main() {
    cc::Build::new()
        .file("double/double.c")
        .compile("double");
}
