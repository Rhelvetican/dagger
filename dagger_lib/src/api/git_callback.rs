use git2::Progress;

pub trait GitCallback {
    fn callback(&mut self, progress: Progress) -> bool;
}
