pub const NEW_HOUSE: &str = "«nuevo»";

#[derive(Clone, Copy)]
pub enum Action {
    Select,
    Filter,
    Create,
    Update,
    Delete,
    Change,
}
