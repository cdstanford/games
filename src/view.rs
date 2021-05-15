/*
    View trait:
    abstracts the behavior of an object that has both a public and a private view.
*/

pub trait View {
    /// Whether two items are equal in ground truth
    fn eq_priv(&self, other: &Self) -> bool;
    /// Whether two items have the same public view
    fn eq_pub(&self, other: &Self) -> bool;
    /// Display ground truth
    fn disp_priv(&self) -> String;
    /// Display public view
    fn disp_pub(&self) -> String;
}
