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

pub trait Source: Sized {
    fn amplify(self, _value: f32) -> source::Amplify<Self> {
        source::Amplify(self)
    }
    fn take_duration(self, _duration: std::time::Duration) -> source::TakeDuration<Self> {
        source::TakeDuration(self)
    }
}

pub mod source {
    pub struct SineWave;
    impl SineWave {
        pub fn new(_freq: f32) -> Self {
            Self
        }
    }
    impl super::Source for SineWave {}

    pub struct Amplify<S>(pub S);
    impl<S: super::Source> super::Source for Amplify<S> {}

    pub struct TakeDuration<S>(pub S);
    impl<S: super::Source> super::Source for TakeDuration<S> {}
}

pub use self::source::SineWave;

pub struct AudioError;

pub struct OutputStream;
pub struct OutputStreamBuilder;
impl OutputStreamBuilder {
    pub fn open_default_stream() -> Result<OutputStream, AudioError> {
        Err(AudioError)
    }
}
impl OutputStream {
    pub fn mixer(&self) -> &Mixer {
        &Mixer
    }
}
