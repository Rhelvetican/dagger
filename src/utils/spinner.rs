use std::{marker::PhantomData, num::NonZeroUsize};

use git2::Progress;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone)]
pub struct TransferProgress<'a> {
    spinner: ProgressBar,
    total: u64,
    __data: PhantomData<&'a ()>,
}

impl<'a> TransferProgress<'a> {
    pub fn construct_spinner(msg: &str) -> ProgressBar {
        let spinner = ProgressBar::new_spinner();

        spinner.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .unwrap()
                .tick_chars("⡿⣟⣯⣷⣾⣽⣻⢿"),
        );

        spinner.set_message(msg.to_string());

        spinner
    }

    #[inline]
    pub fn new(spinner: ProgressBar) -> Self {
        Self {
            spinner,
            total: 0,
            __data: PhantomData,
        }
    }

    pub fn update(&mut self, progress: Progress<'_>) -> bool {
        if self.total == 0
            && let Some(total) = NonZeroUsize::new(progress.total_objects()).map(|n| n.get() as u64)
        {
            let style = ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap_or(ProgressStyle::default_bar())
                .tick_chars("#>-");
            self.spinner.set_style(style);
            self.spinner.set_length(total);
            self.total = total;
        }

        if self.total != 0 {
            self.spinner.set_position(progress.received_objects() as _);
            self.spinner.set_message(format!(
                "Receiving objects: {}/{} (Deltas: {}/{})",
                progress.received_objects(),
                self.total,
                progress.indexed_deltas(),
                progress.total_deltas()
            ));
        }

        self.total != 0
    }
}
