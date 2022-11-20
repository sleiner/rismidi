#![warn(missing_docs)] // TODO

use nih_plug::{params::range::FloatRange, prelude::FloatParam};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PercentageParamBuilder {
    name: String,
    default: f32,
    range: FloatRange,
}

impl PercentageParamBuilder {
    pub fn new(name: impl Into<String>) -> PercentageParamBuilder {
        PercentageParamBuilder {
            name: name.into(),
            default: 0.0,
            range: FloatRange::Linear {
                min: 0.0,
                max: 100.0,
            },
        }
    }

    pub fn with_default(mut self, default: f32) -> Self {
        self.default = default;
        self
    }

    pub fn with_range(mut self, range: FloatRange) -> Self {
        self.range = range;
        self
    }

    pub fn build_float(self) -> FloatParam {
        FloatParam::new(self.name, self.default, self.range)
            .with_value_to_string(Arc::new(|percentage| format!("{:.1}", percentage)))
            .with_unit(" %")
    }
}
