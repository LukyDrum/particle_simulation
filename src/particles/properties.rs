/// Describes how does the particle burn.
pub enum Burnability {
    /// The particle is currently on fire, can light up other particles.
    IsBurning,
    /// The particle can burn if next to fire, contains a durability parameter (How long before the particle burns down).
    CanBurn(u8),
    /// The particle does not react to fire in any way.
    None,
}
