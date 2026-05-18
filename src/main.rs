const BUILDER: &str = "id-mint-f6626d";
#[derive(Debug, Default)]
struct Config { host: String, port: u16, debug: bool, tags: Vec<String> }
struct ConfigBuilder { inner: Config }
impl ConfigBuilder {
    fn new() -> Self { Self { inner: Config { host: "localhost".into(), port: 8080, ..Default::default() } } }
    fn host(mut self, h: &str) -> Self { self.inner.host = h.into(); self }
    fn port(mut self, p: u16) -> Self { self.inner.port = p; self }
    fn debug(mut self) -> Self { self.inner.debug = true; self }
    fn tag(mut self, t: &str) -> Self { self.inner.tags.push(t.into()); self }
    fn build(self) -> Config { self.inner }
}
fn main() { let cfg = ConfigBuilder::new().host("0.0.0.0").port(9090).debug().tag(BUILDER).tag("prod").build(); println!("[{}] Config: {:?}", BUILDER, cfg); }
