# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added

### Changed

### Fixed

## 0.3 - 2022-07-11
### Added
- An override for skipping ground check.
- Exclude entities set for the controller settings to ignore certain entities in ground cast.

### Changed
- Updated to bevy 0.11
- Do multiple shape casts.
- Spring damping force is now the damping ratio instead of a direct coefficient to the damped harmonic oscillator equation
    - <1 is under-damped
    - 1 is critically damped
    - \>1 is over-damped
- First person example:
    - Mouse sensitivity no longer dependent on delta time
    - Sensitivity also scaled more based on common default sensitivities in FPS games (e.g. Valorant)
    - Cursor locked/invisible on launch

### Fixed
- Clamp instead of normalize, so partial inputs work
- Damping velocity takes into account angular velocity

## 0.2.2 - 2022-08-28
### Fixed
- Acceleration bugs.

## 0.2.1 - 2022-08-28
### Changed
- Publicize systems.

### Fixed
- Resolve jitter.

## 0.2.0 - 2022-07-31
### Changed
- Update to bevy 0.8.

## 0.1.5 - 2022-07-19
### Added
- GlobalTransform to bundles.
- Starship example.
- Preset constructors.

### Changed
- Tweak upright force to feel better.
- Rename CCBundle to FPSBundle.

## 0.1.4 - 2022-07-16
### Changed
- Reduce dependencies.

## 0.1.3 - 2022-07-16
### Added
- Mid-air jumps.

### Changed
- Improved jump consistency.
- Made physics tweaks optional.

## 0.1.2 - 2022-07-15
### Added
- Coyote time.
- Jump buffering.

## 0.1.1 - 2022-07-15
### Changed
- Split settings from controller and added input.

## 0.1.0 - 2022-07-15
### Added
- Movement system proof of concept.
