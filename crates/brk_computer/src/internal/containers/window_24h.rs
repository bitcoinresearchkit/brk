use brk_traversable::Traversable;

/// Generic single-24h-window container.
#[derive(Traversable)]
pub struct RollingWindow24h<Inner> {
    pub _24h: Inner,
}
