use crate::owned_model::OwnedQuad;
use itertools::Itertools;
use rio_api::model::{GraphName, NamedNode, Quad, Subject, Term};
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq)]
pub struct Dataset<'a> {
    quads: Vec<Quad<'a>>,
}

impl<'a> AsRef<[Quad<'a>]> for Dataset<'a> {
    fn as_ref(&self) -> &[Quad<'a>] {
        self.quads.as_slice()
    }
}

impl<'a> From<&'a [OwnedQuad]> for Dataset<'a> {
    fn from(value: &'a [OwnedQuad]) -> Self {
        let quads = value.iter().map(Quad::from).collect();
        Dataset::new(quads)
    }
}

impl<'a> Dataset<'a> {
    pub fn new(quads: Vec<Quad<'a>>) -> Self {
        Self { quads }
    }

    pub fn iter(&self) -> Iter<'_, Quad<'a>> {
        self.quads.iter()
    }

    pub fn match_pattern(
        &'a self,
        s: Option<Subject<'a>>,
        p: Option<NamedNode<'a>>,
        o: Option<Term<'a>>,
        g: Option<Option<GraphName<'a>>>,
    ) -> QuadPatternFilter<'a, Iter<'a, Quad<'a>>> {
        self.iter().match_pattern((s, p, o, g).into())
    }

    pub fn skip_pattern(
        &'a self,
        s: Option<Subject<'a>>,
        p: Option<NamedNode<'a>>,
        o: Option<Term<'a>>,
        g: Option<Option<GraphName<'a>>>,
    ) -> QuadPatternFilter<'a, Iter<'a, Quad<'a>>> {
        self.iter().skip_pattern((s, p, o, g).into())
    }
}

#[derive(Copy, Clone)]
pub struct QuadPattern<'a> {
    subject: Option<Subject<'a>>,
    predicate: Option<NamedNode<'a>>,
    object: Option<Term<'a>>,
    graph_name: Option<Option<GraphName<'a>>>,
}

impl<'a>
    From<(
        Option<Subject<'a>>,
        Option<NamedNode<'a>>,
        Option<Term<'a>>,
        Option<Option<GraphName<'a>>>,
    )> for QuadPattern<'a>
{
    fn from(
        value: (
            Option<Subject<'a>>,
            Option<NamedNode<'a>>,
            Option<Term<'a>>,
            Option<Option<GraphName<'a>>>,
        ),
    ) -> Self {
        Self {
            subject: value.0,
            predicate: value.1,
            object: value.2,
            graph_name: value.3,
        }
    }
}

impl QuadPattern<'_> {
    pub fn match_pattern<'a>(self, quad: &'a Quad<'a>) -> bool {
        self.subject.map_or_else(|| true, |s| s == quad.subject)
            && self.predicate.map_or_else(|| true, |p| p == quad.predicate)
            && self.object.map_or_else(|| true, |o| o == quad.object)
            && self
                .graph_name
                .map_or_else(|| true, |g| g == quad.graph_name)
    }
}

pub trait QuadIterator<'a>: Iterator<Item = &'a Quad<'a>> {
    fn match_patterns(self, patterns: Vec<QuadPattern<'a>>) -> QuadPatternFilter<'a, Self>
    where
        Self: Sized,
    {
        QuadPatternFilter::new(self, patterns, QuadPatternFilterKind::Match)
    }

    fn skip_patterns(self, patterns: Vec<QuadPattern<'a>>) -> QuadPatternFilter<'a, Self>
    where
        Self: Sized,
    {
        QuadPatternFilter::new(self, patterns, QuadPatternFilterKind::Skip)
    }

    fn match_pattern(self, pattern: QuadPattern<'a>) -> QuadPatternFilter<'a, Self>
    where
        Self: Sized,
    {
        self.match_patterns(vec![pattern])
    }

    fn skip_pattern(self, pattern: QuadPattern<'a>) -> QuadPatternFilter<'a, Self>
    where
        Self: Sized,
    {
        self.skip_patterns(vec![pattern])
    }

    fn subjects(self) -> Box<dyn Iterator<Item = Subject<'a>> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::from(self.map(|quad: &'a Quad<'a>| quad.subject).unique())
    }

    fn predicates(self) -> Box<dyn Iterator<Item = NamedNode<'a>> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::from(self.map(|quad: &'a Quad<'a>| quad.predicate).unique())
    }

    fn objects(self) -> Box<dyn Iterator<Item = Term<'a>> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::from(self.map(|quad: &'a Quad<'a>| quad.object).unique())
    }

    fn graph_names(self) -> Box<dyn Iterator<Item = Option<GraphName<'a>>> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::from(self.map(|quad: &'a Quad<'a>| quad.graph_name).unique())
    }
}

impl<'a, T: ?Sized> QuadIterator<'a> for T where T: Iterator<Item = &'a Quad<'a>> {}

pub enum QuadPatternFilterKind {
    Match,
    Skip,
}

pub struct QuadPatternFilter<'a, I>
where
    I: Iterator<Item = &'a Quad<'a>>,
{
    patterns: Vec<QuadPattern<'a>>,
    filter_kind: QuadPatternFilterKind,
    inner: I,
}

impl<'a, I> QuadPatternFilter<'a, I>
where
    I: Iterator<Item = &'a Quad<'a>>,
{
    pub fn new(
        inner: I,
        patterns: Vec<QuadPattern<'a>>,
        filter_kind: QuadPatternFilterKind,
    ) -> Self {
        Self {
            patterns,
            inner,
            filter_kind,
        }
    }
}

impl<'a, I> Iterator for QuadPatternFilter<'a, I>
where
    I: Iterator<Item = &'a Quad<'a>>,
{
    type Item = &'a Quad<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find(|quad| match self.filter_kind {
            QuadPatternFilterKind::Match => self.patterns.iter().all(|p| p.match_pattern(quad)),
            QuadPatternFilterKind::Skip => !self.patterns.iter().any(|p| p.match_pattern(quad)),
        })
    }
}
