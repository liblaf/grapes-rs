#[derive(clap::Args)]
pub struct Completion {
    #[arg(value_enum)]
    shell: clap_complete::Shell,
}

impl Completion {
    pub fn invoke<C>(&self)
    where
        C: clap::CommandFactory,
    {
        let mut cmd = C::command();
        let bin_name = cmd
            .get_bin_name()
            .unwrap_or_else(|| cmd.get_name())
            .to_string();
        clap_complete::generate(self.shell, &mut cmd, bin_name, &mut std::io::stdout());
    }
}
