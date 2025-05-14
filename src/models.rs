include!(concat!(env!("OUT_DIR"), "/mod.rs"));

/// High-level status information of the dish.
#[derive(Debug, Clone)]
pub struct DishStatus {
    pub raw: space_x::api::device::Response,
}
