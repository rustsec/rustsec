## 0.7.0 (2022-05-23)
- Update Abscissa to 0.6; replace `gumdrop` with `clap` v3 ([#525])
- Update `rustsec` crate to v0.26 ([#574])

[#525]: https://github.com/RustSec/rustsec/pull/525
[#574]: https://github.com/RustSec/rustsec/pull/574

## 0.6.0 (2021-11-13)
- Update `rustsec` crate to v0.25 ([#480])
- Update `atom_syndication` to 0.11 ([#481])

[#480]: https://github.com/RustSec/rustsec/pull/480
[#481]: https://github.com/RustSec/rustsec/pull/481

## 0.5.3 (2021-10-22)
- Bump `rust-embed` from 5.9.0 to 6.2.0 ([#437])
- Add information about CVSS score and metrics ([#452])
- Add severity tag for informational advisories ([#458])
- Index pages by keyword and category ([#459])

[#437]: https://github.com/RustSec/rustsec/pull/437
[#452]: https://github.com/RustSec/rustsec/pull/452
[#458]: https://github.com/RustSec/rustsec/pull/458
[#459]: https://github.com/RustSec/rustsec/pull/459

## 0.5.2 (2021-09-12)
- Update `atom_syndication` to 0.10 ([#390])
- Don't label OSV feature as unstable, since OSV 1.0 has shipped ([#434])

[#390]: https://github.com/RustSec/rustsec/pull/390
[#434]: https://github.com/RustSec/rustsec/pull/434

## 0.5.1 (2021-07-03)
- Bump `rustsec` to v0.24.1 ([#394])

[#394]: https://github.com/RustSec/rustsec/pull/394

## 0.5.0 (2021-06-28)
- OSV export ([#366])
- Bump `rustsec` to v0.24.0 ([#388])

[#366]: https://github.com/RustSec/rustsec/pull/366
[#388]: https://github.com/RustSec/rustsec/pull/388

## 0.4.3 (2021-05-22)
- Use crates index instead of crates.io api ([#372])

[#372]: https://github.com/RustSec/rustsec/pull/372

## 0.4.2 (2021-05-03)
- web: Add back an Atom feed for advisories

## 0.4.1 (2021-04-30)
- Display more information on the website

## 0.4.0 (2021-03-06)
- Use a fully Rust based solution for rendering web page
- Use rust-embed for static assets
- Add argument to change where website is outputted

## 0.3.5 (2021-03-06) [YANKED]

## 0.3.4 (2021-01-26)
- Bump `rustsec` crate to v0.23

## 0.3.3 (2021-01-04)
- assigner: fix "new year's" bug

## 0.3.2 (2020-11-23) 
- Bump `rustsec` crate to v0.23.0-pre

## 0.3.1 (2020-10-27)
- Bump `rustsec` crate to v0.22.2

## 0.3.0 (2020-10-26)
- Bump `rustsec` crate dependency to v0.22
- `assign-id`: fix command after V3 advisory format migration

## 0.2.1 (2020-07-24)
- Output mode for use with the production github action

## 0.2.0 (2020-06-29)
- linter: refactor into `Linter` struct; check all files
- Bump `rustsec` crate to v0.21.0
- `assign-id` subcommand

## 0.1.1 (2019-10-07)
- Bump `rustsec` crate to v0.15.1

## 0.1.0 (2019-09-21)
- Initial release
