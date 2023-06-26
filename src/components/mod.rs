mod list;
mod searchbar;

#[derive(PartialEq)]
pub enum Msg {
    AppClose,
}

#[macro_export]
macro_rules! _impl_mock_common {
    () => {
        fn query(&self, attr: Attribute) -> Option<AttrValue> {
            self.props.get(attr)
        }
        fn attr(&mut self, attr: Attribute, value: AttrValue) {
            self.props.set(attr, value)
        }
    };
}
