# Wanderlust
<p align="left">
 <a href="https://crates.io/crates/bevy_mod_wanderlust">
  <img src="https://img.shields.io/badge/crates.io-wanderlust-orange">
 </a>
 <a href="https://github.com/PROMETHIA-27/bevy_mod_wanderlust">
  <img src="https://img.shields.io/badge/github-wanderlust-brightgreen">
 </a>
</p>

Wanderlust is a character controller addon. Inspired by [this excellent video](https://www.youtube.com/watch?v=qdskE8PJy6Q) and
my previous attempts at creating a character controller, it is implemented on top of [Rapier physics](https://rapier.rs/)
and highly customizable.
 
Wanderlust does not handle mouselook, as it's more-or-less trivial to implement compared to movement, and would add significant complexity to build in
as many projects will have vastly different requirements for mouselook. The `first_person.rs` example includes an example mouselook implementation.

To use Wanderlust, simply add the [`WanderlustPlugin`](plugins::WanderlustPlugin) to your `App`, and create an entity with the [`CharacterControllerBundle`](bundles::CharacterControllerBundle). The generic parameter for these types can be obtained from the `backends` module after enabling the appropriate features:

* Enable the `rapier2d` feature to expose [`Rapier2dBackend`](backends::Rapier2dBackend) to use as a parameter for `WanderlustPlugin` and
  [`Rapier2dControllerPhysicsBundle`](backends::Rapier2dControllerPhysicsBundle) to use as generic parameter for `CharacterControllerBundle`.
* Enable the `rapier3d` feature to expose [`Rapier3dBackend`](backends::Rapier3dBackend) to use as a parameter for `WanderlustPlugin` and
  [`Rapier3dControllerPhysicsBundle`](backends::Rapier3dControllerPhysicsBundle) to use as generic parameter for `CharacterControllerBundle`.

## Planned Features
- Wallrunning
- Be more agnostic to up-vectors
- More examples
  - Mario-Galaxy-style planetoids
  - Moving platforms
- Fix various jitter issues

## Potential Features
- Become agnostic to physics backend?
- Dashing?
- Ledge grappling?
- Input rework?
- More bundles for different common configurations?

## Contributions
Wanderlust is intended to cover nearly every possible use case of a character controller, so if your use case is not supported (or there's a feature you would like to see)
please drop an issue on the repository! PRs are also welcome, but I may not accept all PRs. Open an issue first if you're not certain that I would accept.

## Examples
The `first_person.rs` example which shows a simple 3D character controller setup. Requires `--features=rapier3d`.
The `platformer_2d.rs` example which shows a simple 2D character controller setup. Requires `--features=rapier2d`.
The `starship.rs` example which shows a simple spaceship controller setup. Requires `--features=rapier3d`.
The `starship.rs` example which shows a simple asteroids-like spaceship controller setup. Requires `--features=rapier2d`.

Dual-licensed under MIT OR Apache 2.0.
