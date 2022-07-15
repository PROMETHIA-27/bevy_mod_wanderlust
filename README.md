# Wanderlust
Wanderlust is a character controller addon. Inspired by [this excellent video](https://www.youtube.com/watch?v=qdskE8PJy6Q) and
my previous attempts at creating a character controller, it is implemented on top of [Rapier physics](https://rapier.rs/)
and highly customizable.

```rust,no_run
# use bevy::prelude::*;
use bevy_mod_wanderlust::WanderlustPlugin;

App::new().add_plugins(DefaultPlugins).add_plugin(WanderlustPlugin).run()
```
 
Wanderlust does not handle mouselook, as it's more-or-less trivial to implement compared to movement, and would add significant complexity to build in
as many projects will have vastly different requirements for mouselook. The `simple.rs` example includes an example mouselook implementation.

## Contributions
Wanderlust is intended to cover nearly every possible use case of a character controller, so if your use case is not supported (or there's a feature you would like to see)
please drop an issue on the repository! PRs are also welcome, but I may not accept all PRs. Open an issue first if you're not certain that I would accept.

## Examples
See the `simple.rs` example which shows a simple character controller setup.

Dual-licensed under MIT OR Apache 2.0.