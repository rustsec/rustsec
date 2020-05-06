## [0.12.0] (2020-05-06)

- Update `rustsec` crate to v0.20 ([#221])
- Regenerate lockfile after `cargo audit fix` ([#219])
- Update dependencies; MSRV 1.40+ ([#216])

## [0.11.2] (2020-02-07)

- Improve yanked crate auditing messages and config ([#200])
- Fix `-c`/`--color` command line argument ([#199])

## [0.11.1] (2020-01-24)

- Add `vendored-openssl` feature ([#193])

## [0.11.0] (2020-01-22)

- Update `rustsec` crate to v0.17 release; MSRV 1.39+ ([#186], [#188])
- Warn for yanked crates ([#180])
- Respect sources of dependencies when auditing ([#175])
- Upgrade to `abscissa` v0.5 ([#174])
- `cargo audit fix` subcommand ([#157], [#166], [#181])

## [0.10.0] (2019-10-13)

- Upgrade `rustsec` to v0.16; new self-audit system ([#155])
- Upgrade to Abscissa v0.4; MSRV 1.36 ([#154])

## [0.9.3] (2019-10-08)

- Update to `rustsec` crate v0.15.2 ([#149])
- presenter: Cleanups for informational advisories ([#148])
- presenter: Print better message when no solution is available ([#144])

## [0.9.2] (2019-10-01)

- Update to `rustsec` crate v0.15 ([#138])

## [0.9.1] (2019-09-25)

- Update to `rustsec` crate v0.14.1 ([#134])

## [0.9.0] (2019-09-25)

- Add `--deny-warnings` option ([#128])
- Upgrade to `rustsec` crate v0.14 ([#126])
- Configuration file: `~/.cargo/audit.toml` ([#123], [#125])
- Fix `--help` ([#113])
- Warn for outdated `rustsec` crate versions ([#112])
- Display warnings for select informational advisories ([#110])
- Display dependency trees with each advisory ([#109])

## [0.8.1] (2019-08-25)

- Fix `--version` ([#101])

## [0.8.0] (2019-08-16)

- Use the Abscissa application framework ([#85], [#87], [#92], [#94])
- Implement `--no-fetch` ([#97])
- Add support for reading lockfiles from STDIN ([#98])

## [0.7.0] (2019-07-15)

- Switch from `term` to `termcolor` crate ([#83])
- Update `gumdrop` to v0.6, `rustsec` crate to v0.12; min Rust 1.32+ ([#82])
- Produce valid JSON when no vulnerabilities are detected ([#77])
- Implement `--ignore` option ([#75])

## [0.6.1] (2018-12-16)

- Fix option parsing ([#64])

## [0.6.0] (2018-12-15)

- Update to Rust 2018 edition ([#61])
- Update to `rustsec` crate v0.10 ([#59])
- Prevent `--help` from exiting with error ([#57])
- Add `--json` flag for JSON output ([#41])

## [0.5.2] (2018-07-29)

- Have `cargo audit version` exit with status `0` ([#38])

## [0.5.1] (2018-07-29)

- Refactoring and UI improvements ([#37])

## [0.5.0] (2018-07-29)

- Upgrade `rustsec` crate to 0.9 ([#36])

## [0.4.0] (2018-07-24)

- Honor the `affected_platforms` attribute ([#35])
- Update `rustsec` crate dependency to `0.8` series ([#34])
- Update `term` crate dependency to `0.5` series ([#31])

## [0.3.2] (2018-07-23)

- README.md: Use `<img>` tag for screenshot so it renders on crates.io ([#28])

## [0.3.1] (2018-07-23)

- Use ` OR ` delimiter to display patched versions ([#25])
- Fix `cargo audit --version` ([#24])

## [0.3.0] (2018-07-23)

- Near rewrite of cargo-audit using `rustsec` 0.7.0.
  See also the [rustsec 0.7.0 release notes] ([#22])

## 0.2.1 (2017-09-24)

- Use crate isatty to resolve Windows build errors ([#14])

## 0.2.0 (2017-03-05)

- Upgrade to rustsec 0.6.0 crate ([#12])
- Configurable colors ([#10])
- Avoid panicking if there are no dependencies ([#8])
- Handle error and instruct the user to generate a lockfile before audit ([#6])

## 0.1.1 (2017-02-27)

- Make cargo-audit a proper cargo subcommand ([#2])

## 0.1.0 (2017-02-27)

- Initial release

[0.12.0]: https://github.com/RustSec/cargo-audit/pull/222
[#221]: https://github.com/RustSec/cargo-audit/pull/221
[#219]: https://github.com/RustSec/cargo-audit/pull/219
[#216]: https://github.com/RustSec/cargo-audit/pull/216
[0.11.2]: https://github.com/RustSec/cargo-audit/pull/201
[#200]: https://github.com/RustSec/cargo-audit/pull/200
[#199]: https://github.com/RustSec/cargo-audit/pull/199
[0.11.1]: https://github.com/RustSec/cargo-audit/pull/194
[#193]: https://github.com/RustSec/cargo-audit/pull/193
[0.11.0]: https://github.com/RustSec/cargo-audit/pull/189
[#188]: https://github.com/RustSec/cargo-audit/pull/188
[#186]: https://github.com/RustSec/cargo-audit/pull/186
[#181]: https://github.com/RustSec/cargo-audit/pull/181
[#180]: https://github.com/RustSec/cargo-audit/pull/180
[#175]: https://github.com/RustSec/cargo-audit/pull/175
[#174]: https://github.com/RustSec/cargo-audit/pull/174
[#166]: https://github.com/RustSec/cargo-audit/pull/166
[#157]: https://github.com/RustSec/cargo-audit/pull/157
[0.10.0]: https://github.com/RustSec/cargo-audit/pull/156
[#155]: https://github.com/RustSec/cargo-audit/pull/155
[#154]: https://github.com/RustSec/cargo-audit/pull/154
[0.9.3]: https://github.com/RustSec/cargo-audit/pull/150
[#149]: https://github.com/RustSec/cargo-audit/pull/149
[#148]: https://github.com/RustSec/cargo-audit/pull/148
[#144]: https://github.com/RustSec/cargo-audit/pull/144
[0.9.2]: https://github.com/RustSec/cargo-audit/pull/139
[#138]: https://github.com/RustSec/cargo-audit/pull/138
[0.9.1]: https://github.com/RustSec/cargo-audit/pull/135
[#134]: https://github.com/RustSec/cargo-audit/pull/134
[0.9.0]: https://github.com/RustSec/cargo-audit/pull/130
[#128]: https://github.com/RustSec/cargo-audit/pull/128
[#126]: https://github.com/RustSec/cargo-audit/pull/126
[#125]: https://github.com/RustSec/cargo-audit/pull/125
[#123]: https://github.com/RustSec/cargo-audit/pull/123
[#113]: https://github.com/RustSec/cargo-audit/pull/113
[#112]: https://github.com/RustSec/cargo-audit/pull/112
[#110]: https://github.com/RustSec/cargo-audit/pull/110
[#109]: https://github.com/RustSec/cargo-audit/pull/109
[0.8.1]: https://github.com/RustSec/cargo-audit/pull/102
[#101]: https://github.com/RustSec/cargo-audit/pull/101
[0.8.0]: https://github.com/RustSec/cargo-audit/pull/99
[#98]: https://github.com/RustSec/cargo-audit/pull/98
[#97]: https://github.com/RustSec/cargo-audit/pull/97
[#94]: https://github.com/RustSec/cargo-audit/pull/94
[#92]: https://github.com/RustSec/cargo-audit/pull/92
[#87]: https://github.com/RustSec/cargo-audit/pull/87
[#85]: https://github.com/RustSec/cargo-audit/pull/85
[0.7.0]: https://github.com/RustSec/cargo-audit/pull/84
[#83]: https://github.com/RustSec/cargo-audit/pull/83
[#82]: https://github.com/RustSec/cargo-audit/pull/82
[#77]: https://github.com/RustSec/cargo-audit/pull/77
[#75]: https://github.com/RustSec/cargo-audit/pull/75
[0.6.1]: https://github.com/RustSec/cargo-audit/pull/65
[#64]: https://github.com/RustSec/cargo-audit/pull/64
[0.6.0]: https://github.com/RustSec/cargo-audit/pull/62
[#61]: https://github.com/RustSec/cargo-audit/pull/61
[#59]: https://github.com/RustSec/cargo-audit/pull/59
[#57]: https://github.com/RustSec/cargo-audit/pull/57
[#41]: https://github.com/RustSec/cargo-audit/pull/41
[0.5.2]: https://github.com/RustSec/cargo-audit/compare/v0.5.1...v0.5.2
[#38]: https://github.com/RustSec/cargo-audit/pull/38
[0.5.1]: https://github.com/RustSec/cargo-audit/compare/v0.5.0...v0.5.1
[#37]: https://github.com/RustSec/cargo-audit/pull/37
[0.5.0]: https://github.com/RustSec/cargo-audit/compare/v0.4.0...v0.5.0
[#36]: https://github.com/RustSec/cargo-audit/pull/36
[0.4.0]: https://github.com/RustSec/cargo-audit/compare/v0.3.2...v0.4.0
[#35]: https://github.com/RustSec/cargo-audit/pull/35
[#34]: https://github.com/RustSec/cargo-audit/pull/34
[#31]: https://github.com/RustSec/cargo-audit/pull/31
[0.3.2]: https://github.com/RustSec/cargo-audit/compare/v0.3.1...v0.3.2
[#28]: https://github.com/RustSec/cargo-audit/pull/28
[0.3.1]: https://github.com/RustSec/cargo-audit/compare/v0.3.0...v0.3.1
[#25]: https://github.com/RustSec/cargo-audit/pull/25
[#24]: https://github.com/RustSec/cargo-audit/pull/24
[0.3.0]: https://github.com/RustSec/cargo-audit/compare/v0.2.1...v0.3.0
[rustsec 0.7.0 release notes]: https://github.com/RustSec/rustsec-client/blob/master/CHANGES.md#070-2018-07-22
[#22]: https://github.com/RustSec/cargo-audit/pull/22
[#14]: https://github.com/RustSec/cargo-audit/pull/14
[#12]: https://github.com/RustSec/cargo-audit/pull/12
[#10]: https://github.com/RustSec/cargo-audit/pull/10
[#8]: https://github.com/RustSec/cargo-audit/pull/8
[#6]: https://github.com/RustSec/cargo-audit/pull/6
[#2]: https://github.com/RustSec/cargo-audit/pull/2
