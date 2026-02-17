pub mod mixer {
    #[derive(Clone)]
    pub struct Mixer;
}

pub use self::mixer::Mixer;

pub struct Sink;
impl Sink {
    pub fn connect_new(_mixer: &Mixer) -> Self {
        Self
    }
    pub fn append<S>(&self, _source: S) {}
    pub fn detach(self) {}
}

pub trait Source {}

pub mod source {
    pub struct SineWave;
    impl SineWave {
        pub fn new(_freq: f32) -> Self {
            Self
        }
    }
    impl super::Source for SineWave {}

    pub struct Amplify<S>(S);
    impl<S> super::Source for Amplify<S> {}

    pub struct TakeDuration<S>(S);
    impl<S> super::Source for TakeDuration<S> {}

    pub trait SourceExt: super::Source + Sized {
        fn amplify(self, _value: f32) -> Amplify<Self> {
            Amplify(self)
        }
        fn take_duration(self, _duration: std::time::Duration) -> TakeDuration<Self> {
            TakeDuration(self)
        }
    }
    impl<S: super::Source> SourceExt for S {}
}

pub use self::source::SineWave;
pub use self::source::SourceExt;

pub struct OutputStream;
pub struct OutputStreamBuilder;
impl OutputStreamBuilder {
    pub fn open_default_stream() -> Result<OutputStream, ()> {
        Err(())
    }
}
impl OutputStream {
    pub fn mixer(&self) -> &Mixer {
        &Mixer
    }
}
