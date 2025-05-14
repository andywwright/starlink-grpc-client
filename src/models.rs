// No include! here—lib.rs already did it.
// Just wrap the shared types.

#[derive(Debug, Clone)]
pub struct DishStatus {
    pub raw: crate::space_x::api::device::Response,
}
