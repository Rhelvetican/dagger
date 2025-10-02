use dagger_lib::{GitCallback, git2::Progress};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{marker::PhantomData, num::NonZeroUsize};

#[derive(Debug, Clone)]
pub struct TransferProgress<'a> {
    spinner: MultiProgress,
    ptr: Option<ProgressBar>,
    total: u64,
    __data: PhantomData<&'a ()>,
}

impl<'a> TransferProgress<'a> {
    fn construct_std_spinner(msg: &str) -> ProgressBar {
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
    pub fn new(msg: &str) -> Self {
        let mut std = Self {
            spinner: MultiProgress::new(),
            ptr: None,
            total: 0,
            __data: PhantomData,
        };

        std.inject(Self::construct_std_spinner(msg));
        std
    }

    pub fn inject(&mut self, item: ProgressBar) {
        self.ptr = Some(self.spinner.add(item));
    }
}

impl GitCallback for TransferProgress<'_> {
    fn callback(&mut self, progress: Progress<'_>) -> bool {
        if self.total == 0
            && let Some(total) = NonZeroUsize::new(progress.total_objects()).map(|n| n.get() as u64)
            && let Some(ptr) = self.ptr.as_ref()
        {
            let style = ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap_or(ProgressStyle::default_bar())
                .tick_chars("⡿⣟⣯⣷⣾⣽⣻⢿")
                .progress_chars("#>-");

            ptr.set_style(style);
            ptr.set_length(total);

            self.total = total;
        }

        if self.total != 0
            && let Some(ptr) = self.ptr.as_ref()
        {
            ptr.set_position(progress.received_objects() as _);
        }

        self.total != 0
    }

    fn println(&self, msg: &str) {
        let _ = self.spinner.println(msg);
    }
}
