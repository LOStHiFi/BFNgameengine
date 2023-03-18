struct System<K: Fn(Context) -> bool> {
    id: u32,
    running_mode: RunningMode,
    running_group: RunningGroup,
    operate: K,
}

enum RunningMode {
    Logical { gap: u8 },
    Rendering,
    Event { event: Event, priority: u8 },
}

enum RunningGroup {
    All,
    Multiple(Vec<u32>),
    Exclude(Vec<u32>),
}