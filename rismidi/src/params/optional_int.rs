use std::sync::Arc;

use nih_plug::prelude::{IntParam, IntRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FooRange {
    min: i32,
    max: i32,
}

impl FooRange {
    fn in_band_none(&self) -> i32 {
        self.min - 1
    }

    fn in_band_range(&self) -> IntRange {
        IntRange::Linear {
            min: self.min - 1,
            max: self.max,
        }
    }
}

pub struct OptionalIntParam {
    inner: IntParam,

    some_range: FooRange,
}

impl OptionalIntParam {
    pub fn new(name: impl Into<String>, default: Option<i32>, min: i32, max: i32) -> Self {
        // TODO: Remove foo
        let some_range = FooRange { min, max };

        Self {
            inner: IntParam::new(
                name,
                Self::optional_to_plain(default, some_range),
                some_range.in_band_range(),
            ),
            some_range,
        }
    }

    pub fn with_none_selected_description(mut self, description: impl Into<String>) -> Self {
        let description = description.into();
        let in_band_none = self.some_range.in_band_none();
        let in_band_range = self.some_range.in_band_range();

        self.inner = self
            .inner
            .with_value_to_string(Arc::new({
                let description = description.clone();

                move |val| {
                    if val == in_band_none {
                        description.to_owned()
                    } else {
                        format!("{}", val)
                    }
                }
            }))
            .with_string_to_value(Arc::new(move |string| {
                let string = string.trim();
                if string == description.to_owned() {
                    Some(in_band_none)
                } else {
                    let int: i32 = string.parse().ok()?;
                    // if in_band_range.
                    todo!()
                }
            }));

        self
    }

    fn optional_to_plain(optional: Option<i32>, some_range: FooRange) -> i32 {
        match optional {
            Some(value) => value.clamp(some_range.min, some_range.max),
            None => some_range.in_band_none(),
        }
    }
}
