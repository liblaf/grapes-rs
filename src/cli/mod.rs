#[cfg(feature = "clap-complete")]
mod completion;
#[cfg(feature = "clap-complete")]
pub use self::completion::Completion;

#[cfg(feature = "clap-markdown")]
mod markdown;
#[cfg(feature = "clap-markdown")]
pub use self::markdown::Markdown;

use color_eyre::Result;

#[cfg(debug_assertions)]
type DefaultLogLevel = clap_verbosity_flag::TraceLevel;

#[cfg(not(debug_assertions))]
type DefaultLogLevel = clap_verbosity_flag::ErrorLevel;

#[derive(clap::Args)]
pub struct Shared<L = DefaultLogLevel>
where
    L: clap_verbosity_flag::LogLevel,
{
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity<L>,
}

#[derive(clap::Subcommand)]
#[non_exhaustive]
pub enum Commands {
    #[cfg(feature = "clap-complete")]
    Completion(Completion),
    #[cfg(feature = "clap-markdown")]
    GenMarkdown(Markdown),
}

pub enum ExecuteResult {
    EarlyExit,
    Success,
}

impl<L> Shared<L>
where
    L: clap_verbosity_flag::LogLevel,
{
    pub fn execute<C>(&self) -> Result<ExecuteResult>
    where
        C: clap::CommandFactory,
    {
        match &self.command {
            Some(command) => {
                match command {
                    #[cfg(feature = "clap-complete")]
                    Commands::Completion(completion) => completion.execute::<C>(),
                    #[cfg(feature = "clap-markdown")]
                    Commands::GenMarkdown(markdown) => markdown.execute::<C>(),
                    #[allow(unreachable_patterns)]
                    _ => {}
                }
                return Ok(ExecuteResult::EarlyExit);
            }
            None => {}
        }
        self.init_logging()?;
        Ok(ExecuteResult::Success)
    }

    pub fn init_logging(&self) -> Result<()> {
        let ansi = match supports_color::on_cached(supports_color::Stream::Stderr) {
            Some(color_level) => color_level.has_basic,
            None => false,
        };
        let builder = tracing_subscriber::fmt()
            .with_ansi(ansi)
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_max_level(self.verbosity.tracing_level_filter())
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .with_timer(tracing_subscriber::fmt::time::Uptime::default());
        if let Some(level) = self.verbosity.tracing_level()
            && level >= tracing::Level::DEBUG
        {
            builder.pretty().init();
        } else {
            builder.init();
        }
        color_eyre::install()?;
        Ok(())
    }
}
