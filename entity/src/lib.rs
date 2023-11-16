use std::collections::HashMap;

#[derive(Debug)]
pub struct Questions {
    pub inner: HashMap<QuestionId, Question>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuestionId(pub u64);

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Question {
    pub id: QuestionId,
    pub logic: String,
}

impl PartialEq for Question {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Questions {
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&QuestionId, &Question)> {
        self.inner.iter()
    }

    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &QuestionId> {
        self.inner.keys()
    }

    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &Question> {
        self.inner.values()
    }
}

impl Question {
    pub const fn new(id: QuestionId, logic: String) -> Self {
        Self { id, logic }
    }
}
