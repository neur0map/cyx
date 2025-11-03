/// Runtime context for CLI flags and options
#[derive(Debug, Clone, Default)]
pub struct CliContext {
    pub quiet: bool,
    pub verbose: bool,
    pub no_tty: bool,
    pub learn: bool,
}

impl CliContext {
    pub fn new(quiet: bool, verbose: bool, no_tty: bool, learn: bool) -> Self {
        Self {
            quiet,
            verbose,
            no_tty,
            learn,
        }
    }

    /// Check if colors should be disabled
    pub fn should_disable_colors(&self) -> bool {
        self.no_tty || self.quiet
    }

    /// Check if we should show banners and decorations
    pub fn should_show_decorations(&self) -> bool {
        !self.quiet && !self.no_tty
    }

    /// Check if we should show progress messages
    pub fn should_show_progress(&self) -> bool {
        !self.quiet
    }

    /// Check if we should show verbose debug info
    pub fn should_show_verbose(&self) -> bool {
        self.verbose
    }
}
