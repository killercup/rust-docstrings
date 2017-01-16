use pulldown_cmark::{Event};

pub struct OffsetParser<'a>(pub ::pulldown_cmark::Parser<'a>);

impl<'a> Iterator for OffsetParser<'a> {
    type Item = (Event<'a>, usize);

    fn next(&mut self) -> Option<(Event<'a>, usize)> {
        let &mut OffsetParser(ref mut inner_iter) = self;
        let offset = inner_iter.get_offset();

        match inner_iter.next() {
            Some(event) => {
                Some((event, offset))
            }
            None => None
        }
    }
}

pub fn fst<T1, T2>((t1, _t2): (T1, T2)) -> T1 {
    t1
}
