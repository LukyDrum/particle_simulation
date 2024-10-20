/// Describes how does the particle burn.
pub enum Burnability {
    /// The particle is currently on fire, can light up other particles.
    /// Contains a durability parameter (How long before the particle burns down).
    IsBurning(u8),
    /// The particle can burn if next to fire.
    CanBurn,
    /// The particle does not react to fire in any way.
    None,
}
