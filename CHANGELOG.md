# Changelog

All notable changes to this project will be documented in this file.

## 0.2.0-beta.1 - 2026-01-08

[71dd9af](71dd9afa06317a8b4bd2ec97c07cfd6e6d4bd99f)...[76af6da](76af6dae293308a346eeadac772cd096d2ae9943)

### Miscellaneous Tasks

- Rm build script ([450c71e](450c71ef724ceec4eb723a1e81bd8fb3e851d042))

## x.x.3 - 2026-01-08

[5fc71cb](5fc71cb50242adfc467987978e1282ba51ba215e)...[71dd9af](71dd9afa06317a8b4bd2ec97c07cfd6e6d4bd99f)

### Miscellaneous Tasks

- Some release action enhancements ([71dd9af](71dd9afa06317a8b4bd2ec97c07cfd6e6d4bd99f))

## x.x.2 - 2026-01-08

[8c438d8](8c438d8164879c4652927392638ed49fd491d160)...[5fc71cb](5fc71cb50242adfc467987978e1282ba51ba215e)

### Miscellaneous Tasks

- Fix the action ([5fc71cb](5fc71cb50242adfc467987978e1282ba51ba215e))

## x.x.1 - 2026-01-08

[dca35aa](dca35aa1e51f52e3483732e4729a6f3ad6338504)...[8c438d8](8c438d8164879c4652927392638ed49fd491d160)

### Miscellaneous Tasks

- Create release action ([8c438d8](8c438d8164879c4652927392638ed49fd491d160))

## x.x.x - 2026-01-07

[44417e2](44417e24214d83033ef615cd4944e202b34fd4c2)...[dca35aa](dca35aa1e51f52e3483732e4729a6f3ad6338504)

### Bug Fixes

- Fix a bug where sentry debug logs were outputted by default ([049ca3c](049ca3c029435b126bd40c563423a80cfa60f225))
- Fix a bug where the package name would be named incorrectly ([0b82044](0b820449511c3ac51f2eff556c53f57508d3f225))
- Set some flags to be conflicting ([84aca48](84aca48a72e661fc3b0ef35286d565a2380d3510))

### Features

- Add --json flag ([615dcac](615dcac3c0dc4aab41fa7b7a3d4292e0d54e347a))
- Add --short flag to devlog get ([85665d9](85665d9745be9dc2d34ef8b6520d7e61f776fe3f))
- Add --resolve flag to print project ([d31ad26](d31ad26b23c56c238533dffbc8ec871cdb69f37c))
- Make list devlogs return project name ([402dba3](402dba33f2c63bdad0b2cd61c181a052dd9afcf0))
- Add --short and --long flags to get store ([34abed2](34abed2cff05b6f56b65bd24eb1da0f0a35f2c5e))
- Add --region flag to store list ([75310d1](75310d18f0c4610a6a3b40faf850faecda3d5294))

### Miscellaneous Tasks

- Update release.toml ([d721290](d7212904755ca7d86a3d37d5ac8773f945175232))

### Refactor

- Moved printlns to macros in print_ helpers ([dca35aa](dca35aa1e51f52e3483732e4729a6f3ad6338504))

### Styling

- Clippy stuff ([fb65755](fb657557d4ab661f0997644d9d30e85ba04dc331))

## 0.1.0-beta.3 - 2026-01-07

[c25db4e](c25db4e09f23591d11b64871fa91aee1ce1565bd)...[44417e2](44417e24214d83033ef615cd4944e202b34fd4c2)

### Bug Fixes

- Fix a bug where the program would panick if the devlog text contained Korean characters (#1) ([eeeff72](eeeff727a7043713dab25a3eabbd1f0b804d40c4))

### Miscellaneous Tasks

- Release flavorcli version 0.1.0-beta.3 ([44417e2](44417e24214d83033ef615cd4944e202b34fd4c2))

## 0.1.0-beta.2 - 2026-01-06

[72405e3](72405e3c4d0a256c8ab2d36d86c7561e4362160e)...[c25db4e](c25db4e09f23591d11b64871fa91aee1ce1565bd)

### Documentation

- Update install instructions ([932a33f](932a33f4dcaccdecb80b3492b9d3c2cc8ea8616b))
- Execution policy and chmod instructions ([b430926](b430926d041829ceb0848d9e1557ca183f5b657f))
- Add instructions for cargo ([709f954](709f954b3344e7bf7c251a03af2d8617a4d64b7f))

### Miscellaneous Tasks

- Add some release scripts and CHANGELOG ([5749fd3](5749fd3a28655260ece441c0964d2ed44d7644fc))
- Add sentry ([029a12e](029a12eff3499c62dbd64e531bc124d26c24ebf9))
- Add more logging ([e33c085](e33c085d83965e1329de81c4272bae5cf66e4dde))
- Release flavorcli version 0.1.0-beta.2 ([c25db4e](c25db4e09f23591d11b64871fa91aee1ce1565bd))

### Styling

- Clear out some clippy warnings ([c2394bd](c2394bdabe5c68f36c54819bd239a314dbd5b5ae))

## 0.1.0-beta.1 - 2026-01-04

### Bug Fixes

- Handle type changes to upstream API ([aa1107f](aa1107fe29e8f1dc0b1a89a07dc8de5c2cb919ed))

### Documentation

- Tests shields ([50846cf](50846cfb0645a1a0bc8a8e6befa702229339ba2d))

### Features

- Add command-line argument parsing with Clap ([2f29a9d](2f29a9d1237eef969d674c8028575d93ffac0706))
- Update commands ([8cfc7b1](8cfc7b1870da691ab338c26537f89b4f1b11d478))
- Auth set command ([823409b](823409b9eb598fae8af91450d1ab6635a9176951))
- Auth delete command ([214c271](214c271c5fc9bff00828e480f1abf1b8174bf4b6))
- Project command ([b006406](b006406430b7a7e068092d5a7436cf354f6c694d))
- Project list command ([6f14c1a](6f14c1aca19bdfad276c467db5861a4f57715aa2))
- Project devlog get command ([8416484](84164840d97b7a7196f31b5b9881a2364785d748))
- List devlog command ([9b81998](9b81998094cf23a1e3d2c758569cbf300206336f))
- Store get command ([8ddb671](8ddb67175f6c3f3009c43f2e9119dd7b59ce5c8b))
- Store list command ([81506f6](81506f61609498f1ce79b39d2e406854ac5d26b3))
- Get user command ([ad0c396](ad0c396d17d22d050921e647338dcda7e66664a4))
- Make 'get user' convert seconds to time ([7e9991b](7e9991b09b44df309ed641a543b21f0b99c87813))
- List user command ([8501f7b](8501f7bbad3babd21afbe0ea1bd5a66961061c2c))

### Miscellaneous Tasks

- Update workflows to install apt deps ([14ba626](14ba626db5b07a5b698526936cf750acde556fcf))
- Apt deps for this workflow too ([be9f205](be9f205f3d80369543276421e0e2758aa6314ec3))
- Prepare for a release ([2710d0a](2710d0a146c96439e2b42e7c89c973074eedc8c7))
- Update cargo.toml ([24faaf0](24faaf0b2d0003924aff232318b325a09231c3cf))
- Another cargo toml update because im stupid ([72405e3](72405e3c4d0a256c8ab2d36d86c7561e4362160e))

### Refactor

- Move commands to induvidual files ([9396b16](9396b16ac5fc36eeaae5fda58533607ab56764f3))

### Styling

- Linting ([8734743](8734743ce03da83a36ee9e92b50f6fc8cdd8553c))
- Clean up ([f0bd8c2](f0bd8c21a0567bc7f514f7bb06e7e5ffb5a77ae0))
- Remove unused import ([4c404cf](4c404cfc4088dcad0025be3c9de96c41ce035c5c))
- Remove unused function ([ed1c91d](ed1c91d976a50c4582dc8d7668dee3cc9367641b))

### Revert

- Downgrade reqwest ([1748c00](1748c000b8950aa379bd8f19f3720b92c1a3750d))

<!-- generated by git-cliff -->
