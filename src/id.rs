use std::collections::BTreeSet;
use std::ops::Bound;
use std::sync::{LazyLock, Mutex};

static SLIDE_IDS: LazyLock<Mutex<BTreeSet<u8>>> = LazyLock::new(Mutex::default);

/// A Slide id
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, derive_more::Display, derive_more::FromStr,
)]
pub struct SlideId(u8);

impl Default for SlideId {
    fn default() -> Self {
        let mut ids = SLIDE_IDS.lock().unwrap();
        ids.insert(0);
        Self(0)
    }
}

impl From<u8> for SlideId {
    fn from(value: u8) -> Self {
        let mut ids = SLIDE_IDS.lock().unwrap();
        ids.insert(value);

        Self(value)
    }
}

impl SlideId {
    pub fn first() -> Option<Self> {
        let ids = SLIDE_IDS.lock().unwrap();
        ids.first().copied().map(Self)
    }

    pub fn last() -> Option<Self> {
        let ids = SLIDE_IDS.lock().unwrap();
        ids.last().copied().map(Self)
    }

    fn inspect(&self) -> (Option<u8>, Option<u8>) {
        let ids = SLIDE_IDS.lock().unwrap();
        let previous = ids.range(..self.0).last().copied();
        let next = ids
            .range((Bound::Excluded(self.0), Bound::Unbounded))
            .next()
            .copied();
        (previous, next)
    }

    pub fn relation(&self, other: &Self) -> Option<SlideIdRelation> {
        if self == other {
            return Some(SlideIdRelation::Current);
        }
        let (previous, next) = self.inspect();
        if Some(other.0) == previous {
            Some(SlideIdRelation::Previous)
        } else if Some(other.0) == next {
            Some(SlideIdRelation::Next)
        } else {
            None
        }
    }

    pub fn previous(&self) -> Option<Self> {
        self.inspect().0.map(Self)
    }

    pub fn next(&self) -> Option<Self> {
        self.inspect().1.map(Self)
    }
}

#[derive(Debug, Clone, Copy, derive_more::Display)]
pub enum SlideIdRelation {
    #[display("previous")]
    Previous,
    #[display("current")]
    Current,
    #[display("next")]
    Next,
}
