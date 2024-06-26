# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.3.0...openstack_sdk-v0.4.0) - 2024-04-05

### Added
- Add LoadBalancer
- regenerate code with latest generator changes
- *(volume)* cover block-storage backup

### Other
- drop serde_yaml dependency
- Merge pull request [#113](https://github.com/gtema/openstack/pull/113) from gtema/deps
- update http/reqwest/hyper lib

## [0.3.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.2.0...openstack_sdk-v0.3.0) - 2024-03-15

### Added
- add autogenerated functional tests
- add image.metadef implementation
- improve image docstrings
- *(network)* enable volume type enctyption commands
- enable router interface/routes/gateways operations
- implement router {add,remove}_router_interface
- add identity group resources
- *(identity)* extend role commands
- *(identity)* Add role-assignment list command

### Other
- preparation changes for image.metadef
- replace deprecated chrono::Duration::days call
- reorg integration tests
- sort struct fields alphabetically
- *(deps)* bump open from 5.0.1 to 5.1.1
- remove few unnecessary clone invocations
- fix role responses

## [0.2.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.1.1...openstack_sdk-v0.2.0) - 2024-02-23

### Added
- split out scope into separate module
- *(auth)* Add support for auth with application credentials
- add func tests verifying connection
- further modularilze sdk
- split OpenStack client module

### Fixed
- *(sdk)* flatten _properties of extensible structs
- respect endpoint_override for missing catalog entry
- Respect server defaults for pagination

### Other
- switch to caret requirements syntax
- further improve comments
- better pre-format comments
- further modularize auth plugins
- Merge pull request [#55](https://github.com/gtema/openstack/pull/55) from gtema/dependabot/cargo/derive_builder-0.20.0

## [0.1.1](https://github.com/gtema/openstack/compare/openstack_sdk-v0.1.0...openstack_sdk-v0.1.1) - 2024-02-16

### Added
- *(docs)* Improve documents structure
- *(auth)* Enable WebSSO authentication

### Fixed
- ensure auth cache is enabled by default

### Other
- Revert "chore: release"
- release
- minor reorg in cargo manifests
