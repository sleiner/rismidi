/// A duration in "wall clock" time (milliseconds, seconds, ...).
pub type TimeDuration = std::time::Duration;

/// The duration of a note, in relation to a pulse of quarter notes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NoteDuration {
    nominator: u32,
    denominator: u32,
    dots: u32,
    tuplet: Option<Tuplet>,
}

impl NoteDuration {
    /// Creates a new [`NoteDuration`]. `nominator` and `denominator` determine the length.
    ///
    /// # Examples
    ///
    /// ```
    /// let quarter_note = rismidi::NoteDuration::new(1, 4);
    /// let three_eights = rismidi::NoteDuration::new(3, 8);
    /// ```
    ///
    pub fn new(nominator: u32, denominator: u32) -> NoteDuration {
        NoteDuration {
            nominator,
            denominator,
            dots: 0,
            tuplet: None,
        }
    }

    /// Changes how many [dots][1] the note has.
    ///
    /// The first dot increases the length of the note by half. Each subsequent dot increases the
    /// length of the note by half the length of the previous dot.
    ///
    /// [1]: https://en.wikipedia.org/wiki/Dotted_note
    pub fn dotted(mut self, num_dots: u32) -> Self {
        self.dots = num_dots;
        self
    }

    /// Turns the current [NoteDuration] into a [triplet][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Tuplet#Triplet
    pub fn triplet(self) -> Self {
        self.tuplet(Tuplet::TRIPLET)
    }

    /// Turns the current [NoteDuration] into a [tuplet][1].
    ///
    /// You can think of a tuplet as a more generalized version of a [triplet][2].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Tuplet
    /// [2]: https://en.wikipedia.org/wiki/Tuplet#Triplet
    pub fn tuplet(mut self, tuplet: Tuplet) -> Self {
        self.tuplet = Some(tuplet);
        self
    }

    /// Removes all tuplets from the current [NoteDuration].
    pub fn straight(mut self) -> Self {
        self.tuplet = None;
        self
    }
}

/// Models a musical [tuplet][1].
///
/// [1]: https://en.wikipedia.org/wiki/Tuplet
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tuplet {
    num_regular_notes: u32,
    num_tuplet_notes: u32,
}

impl Tuplet {
    const TRIPLET: Tuplet = Tuplet {
        num_tuplet_notes: 3,
        num_regular_notes: 2,
    };
}
