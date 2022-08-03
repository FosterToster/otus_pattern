pub trait Component<T> {
    fn tree_view(&self) -> T;
}
