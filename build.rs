use wesl::Wesl;

fn main() {
    Wesl::new("shaders").build_artifact(&"package::shader".parse().unwrap(), "shader");
}
