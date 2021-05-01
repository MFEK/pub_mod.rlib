# `proc_mod`

When called as:

```rust
proc_mod!(".");
```

When `.` contains:

```
mod.rs
select.rs
anchors.rs
```

Expands to:

```rust
pub mod select;
pub mod anchors;
```
