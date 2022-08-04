pub trait CompositeLeaf {
    type Return;
    fn operation(&self) -> Self::Return;
}

pub trait CompositeContainer<
    L: CompositeLeaf<Return = RSLT>,
    C: CompositeContainer<L, C, L::Return>,
    RSLT,
>: CompositeLeaf
{
    fn append_item(&mut self, member: CompositeItem<L, C, RSLT>);
}

pub enum CompositeItem<
    L: CompositeLeaf<Return = RSLT>,
    C: CompositeContainer<L, C, L::Return>,
    RSLT,
> {
    Leaf(L),
    Container(C),
}
