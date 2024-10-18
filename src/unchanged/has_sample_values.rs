pub trait HasSampleValues: Sized {
    fn sample() -> Self;
    fn sample_other() -> Self;
}
