#[derive(clap::Args)]
pub struct Markdown {}

impl Markdown {
    pub fn invoke<C>(&self)
    where
        C: clap::CommandFactory,
    {
        let markdown = clap_markdown::help_markdown::<C>();
        print!("{}", markdown);
    }
}
