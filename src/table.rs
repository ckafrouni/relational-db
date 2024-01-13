use super::Column;

pub trait Table {
    fn name(&self) -> &str;
    fn columns(&self) -> Vec<Box<dyn Column>>;
    fn pk(&self) -> Option<Box<dyn Column>> {
        None
    }
}
