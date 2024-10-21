/// Describes how does the particle burn.
#[derive(Clone, Copy)]
pub enum Burnability {
    /// The particle is currently on fire, can light up other particles.
    /// Contains a durability parameter (How long before the particle burns down).
    IsBurning(u8),
    /// The particle can burn if next to fire.
    CanBurn,
    /// The particle does not react to fire in any way.
    None,
}

impl Burnability {
    pub fn decreased_by(&self, value: u8) -> Burnability {
        match self {
            Self::IsBurning(time) => {
                let res = (*time as i32) - (value as i32);
                let res = res.max(0) as u8;
                Self::IsBurning(res)
            }
            _ => *self,
        }
    }
}
