use crate::tree::Tree;

#[derive(Default, Debug)]
pub struct Config;
#[derive(Debug)]
pub struct Context {
    config: Config,
    tree: Option<Tree>,
}

impl Context {
    pub fn create(cfg: Option<Config>) -> Self {
        Self {
            config: cfg.unwrap_or_default(),
            tree: None,
        }
    }
    pub fn tree(&self) -> Option<&Tree> {
        self.tree.as_ref()
    }
    pub fn config(&self) -> &Config {
        &self.config
    }
}
pub trait Parse<T> {
    fn parse(&mut self, input: T);
}

impl Parse<&str> for Context {
    fn parse(&mut self, input: &str) {
        self.tree = Some(Tree::from(input));
    }
}

impl Parse<String> for Context {
    fn parse(&mut self, input: String) {
        self.tree = Some(Tree::from(&input));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let raw = r#"
          <div id="123">hello
          world
          </div>
        "#;
        let mut ctx = Context::create(None);
        ctx.parse(raw);
        if let Some(t) = ctx.tree() {
            println!("{:?}", t);
        }
    }
}
