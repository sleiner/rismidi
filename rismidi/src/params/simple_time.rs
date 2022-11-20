use crate::{NoteDuration, TimeDuration};
use nih_plug::prelude::{Enum, EnumParam, FloatParam, FloatRange, IntRange, Param, Params};
use std::sync::Arc;

/// Specifies a "musical" duration, like "a quarter note" or "200 ms".
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SimpleTime {
    /// "Wall clock" time.
    Free(TimeDuration),

    /// Time as synchronized to the musical grid.
    Sync(NoteDuration),
}

#[derive(Debug, PartialEq, Eq, Enum)]
enum SimpleTimeMode {
    #[id = "free"]
    #[name = "Free"]
    Free,

    #[id = "sync"]
    #[name = "Sync"]
    Sync,
}

impl From<SimpleTime> for SimpleTimeMode {
    fn from(from: SimpleTime) -> Self {
        match from {
            SimpleTime::Free(_) => SimpleTimeMode::Free,
            SimpleTime::Sync(_) => SimpleTimeMode::Sync,
        }
    }
}

struct InnerParam {
    mode_param: Arc<EnumParam<SimpleTimeMode>>,

    millisecond_range: FloatRange,
    choice_range: IntRange,
    sync_choices: Vec<(String, NoteDuration)>,
}

impl InnerParam {
    pub fn new(
        mode_param: EnumParam<SimpleTimeMode>,
        millisecond_range: FloatRange,
        sync_choices: Vec<(String, NoteDuration)>,
    ) -> Self {
        InnerParam {
            mode_param: Arc::new(mode_param),
            millisecond_range,
            choice_range: IntRange::Linear {
                min: 0,
                max: (sync_choices.len() - 1) as i32,
            },
            sync_choices: sync_choices,
        }
    }

    pub fn normalized_to_string(&self, normalized: f32) -> String {
        match self.mode_param.value() {
            SimpleTimeMode::Free => {
                format!("{} ms", self.normalized_to_free(normalized).as_millis())
            }
            SimpleTimeMode::Sync => {
                let (string, _) = self.normalized_to_sync(normalized);
                string.to_owned()
            }
        }
    }

    pub fn string_to_normalized(&self, string: &str) -> Option<f32> {
        let string = string.trim();

        let value = match self.mode_param.value() {
            SimpleTimeMode::Free => {
                let string = string.trim_end_matches("ms").trim();
                let millis = string.parse().ok()?;
                self.millisecond_range.normalize(millis)
            }
            SimpleTimeMode::Sync => {
                let index = self.sync_choices.iter().position(|(s, _)| string == s)?;
                self.choice_range.normalize(index.try_into().ok()?)
            }
        };

        Some(value)
    }

    pub fn normalized_to_delay_time(&self, normalized: f32) -> SimpleTime {
        match self.mode_param.value() {
            SimpleTimeMode::Free => SimpleTime::Free(self.normalized_to_free(normalized)),
            SimpleTimeMode::Sync => {
                SimpleTime::Sync(self.normalized_to_sync(normalized).1.to_owned())
            }
        }
    }

    fn normalized_to_free(&self, normalized: f32) -> TimeDuration {
        let time_ms = self.millisecond_range.unnormalize(normalized);
        TimeDuration::from_millis(time_ms.round() as u64)
    }

    fn normalized_to_sync(&self, normalized: f32) -> &(String, NoteDuration) {
        let normalized = normalized.clamp(0.0, 1.0);

        let max_index = self.sync_choices.len() - 1;
        let index = (normalized * (max_index as f32)).round() as usize;

        &self.sync_choices[index]
    }
}

/// A collection of two parameters specifying either a musical or a "wall clock time" duration.
///
/// The collection consists of two parameters:
/// 1. The **mode** parameter allows the user to choose between two time modes: _Free_ and _Sync_.
/// 2. The **time** parameter lets the user specify a time, either in milliseconds (in the _Free_
///    mode) or in [note durations](`NoteDuration`) (in the _Sync_ mode).
///
/// # Examples
///
/// ```
/// use nih_plug::prelude::*;
/// use rismidi::params::SimpleTimeParams;
///
/// #[derive(Params)]
/// struct MyPluginParams {
///     #[nested(group = "Delay", id_prefix = "delay")]
///     pub delay_time: SimpleTimeParams,
/// }
/// ```
#[derive(Params)]
pub struct SimpleTimeParams {
    #[id = "mode"]
    mode_param: Arc<EnumParam<SimpleTimeMode>>,

    #[id = "time"]
    time_param: FloatParam,

    inner: Arc<InnerParam>,
}

impl SimpleTimeParams {
    /// Creates a new [`SimpleTimeParams`] instance.
    ///
    /// Using `millisecond_range`, you can specify the scale of milliseconds in the "free" time
    /// mode. `sync_choices` contains the choices presented to the user in the "sync" time mode.
    pub fn new(
        name: impl Into<String>,
        millisecond_range: FloatRange,
        sync_choices: &[(String, NoteDuration)],
    ) -> SimpleTimeParams {
        let name = name.into();
        let inner = Arc::new(InnerParam::new(
            EnumParam::new(format!("{name}: Mode"), SimpleTimeMode::Free),
            millisecond_range,
            sync_choices.to_owned(),
        ));

        SimpleTimeParams {
            mode_param: inner.mode_param.clone(),
            time_param: FloatParam::new(
                format!("{name}: Time"),
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_value_to_string(Arc::new({
                let inner = inner.clone();
                move |v| inner.normalized_to_string(v)
            }))
            .with_string_to_value(Arc::new({
                let inner = inner.clone();
                move |s| inner.string_to_normalized(s)
            })),
            inner,
        }
    }

    /// Returns the time that the user has chosen.
    pub fn value(&self) -> SimpleTime {
        let normalized_time = self.time_param.modulated_normalized_value();
        self.inner.normalized_to_delay_time(normalized_time)
    }
}
