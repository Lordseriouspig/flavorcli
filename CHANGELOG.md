# Changelog

All notable changes to this project will be documented in this file.

## 1.0.0 - 2026-01-24

[04927ef](04927efd67448b16472e9ef7e5e5c2ca5a6b6393)...[6f9cf24](6f9cf2486359728b2b5b37907709ded46d9f4455)

### Bug Fixes

- Changelog issue ([b6dab9c](b6dab9cb470a3e0a3e78475d8090e537df80ebed))
- Remove attachments from devlog update ([85df650](85df6503fbcc033f94731d9223bbc1bdacf72a45))

### Documentation

- Update docs ([c76869b](c76869b84b22ae53072f2b1e9a4bd5b3cee63015))
- Add new devlog commands to docs ([966633f](966633fd99160a5d03ce1c7acac1e39b922438ff))
- Change up terminology ([5e3ec70](5e3ec7019edb8dfc5ba846a5d4d9629f0456eff3))
- Update features in README ([06ce595](06ce59526ed641812265ca36e784d8b0666b3bc5))

### Features

- Add query to users ([a23652c](a23652cc9a0bd2b8ea8c9c65e3d3440c45e27299))
- Add create update and delete devlogs! ([0b98799](0b987993eae24dc384cb37c85907fec6395918ba))
- Show devlog media when project id is supplied or it is available ([00c1cc1](00c1cc1f7b5216ba5cccce06119e185010e81d06))
- Use futures in resolve project and devlog ([a81561d](a81561d3e321c9cdf245f0586ec5fd2020080b38))

### Miscellaneous Tasks

- Nix flake ([a9ef500](a9ef500db04e72db84ee6d5a2bd4c804bb0c71d0))
- Remove unused import ([bc3d32f](bc3d32fc1bc970524c57445246c82ac75766d5fd))

### Refactor

- Remove some .clone() ([681cf71](681cf717ad3c6e270d1a89cefe9306f7f51cde8f))

### Styling

- Lintttttttttttt ([b150cd3](b150cd36babbca081640d4af91730480c63e824e))

## 0.3.0 - 2026-01-16

[7b7829b](7b7829b7329d9031b64b68e69c0aa71431e69a13)...[04927ef](04927efd67448b16472e9ef7e5e5c2ca5a6b6393)

### Bug Fixes

- Change sorting logic for regional ([a94299d](a94299d09263a34162a01179ca98875816aa4bd5))

### Documentation

- Basic docs structure ([25c3449](25c344941a50793139f89d6601536bca0243399d))
- Add getting started page ([df8a67c](df8a67cd16b01e9e47893fccf7f0b37403642d4e))
- Add installation page ([a3d9f97](a3d9f978d6b5921cc248399d77186f74e5565b89))
- Add command basics page ([5cec7c8](5cec7c865af331a7a7ba0d76b615eda887f5f22d))
- Add add to path instructions ([518a804](518a8046491a8cf184a86c718e7309d9c1ec98e5))
- Add auth commands instructions ([f915297](f915297dca2146e6f1571e8fc0e7a0947ee0efdd))
- Add devlog commands instructions ([bc1337c](bc1337cf20e52dca72f1e69935c7890a070a5bf7))
- Add project docs ([2bd141e](2bd141e15a8dacd299c2bf5f73c31d5850b01c27))
- Add store docs ([a762012](a762012f2cb3ab86a5af137ae64e1e1bd4c672af))
- Add user command docs ([d1e34f5](d1e34f541dc3ce782306c79a7d5b16a7a4314288))
- I am not in the mood to do dev docs so lets forget it ever existed ([20f42b2](20f42b21040e9c1bf35145d4756cea9d87d1b243))
- Add docs links ([d533fbe](d533fbe082ced13c01d9fd28755d16ab70683126))
- Add quick links ([a13ed5f](a13ed5ff77d76ee62ab69d0fa68cedfc34c90510))
- Fix up note ([fd2bb16](fd2bb16e0f15986f5788c5756a29ac9dcf5c1f5b))
- Note for flavortown users ([6c252a6](6c252a68b70d04ff95ea4387ba115220709a9507))

### Features

- Add project create and update commands ([16860c1](16860c1af4f6a391c25ce78d2b527e4f64bb69d3))
- Add a flag to choose table fields in store list ([66849b7](66849b743eb6e68ffa757f7b73772d0725372ab0))
- Add sorting for store list ([caaf1f5](caaf1f5fe2f4375859dcbaaed6f035364782d977))
- Add table field choosing to user list ([27830e4](27830e4dc7bb394578f8cfb53e971221eed08238))
- Add custom fields to project devlogs list ([3998cdd](3998cddb40771a19b31150feb7d93bb45038cc3a))
- Add custom fields to project list ([f309870](f309870a88bf6fc0b2c5432c55fdef313c062d3d))
- Add resolve flag to get user ([4ef1269](4ef1269a9bc45f37fb167e1841e7082052640821))
- Allow update to not change title or description ([6ad5bc6](6ad5bc644caf661f254f778b6dfe37d2f99d58f0))

### Miscellaneous Tasks

- Fix release name ([b448016](b448016a577ad39c7c71416b3834223ba2c29e48))
- Add -j to the zip command on linux ([e6276e4](e6276e4008df9ccfbcbec4d99672e12e6d4a2c26))
- Small debug log ([717613d](717613d10a2ce33a8241b6c6a99508e64b5d59cc))
- Add aur stuff ([1f043bd](1f043bd9fc3362f24ba1e73b558b8d5484cc128e))
- Make linter happy ([1ad5e34](1ad5e3467a1d7ffe4bf1c20e97ecd1e770c353a2))
- Lintttttttttttt ([ea8d0e4](ea8d0e4d785a65a2cbdd0384ad915656f7629041))

### Refactor

- Use /users/me insteaed of asking for user id by default ([0e2cf24](0e2cf2493bf5a57e434eb9e0a2791210eb4b09f2))
- Use u32 consistantly ([a44e3a3](a44e3a300458296827bf25cbc989c90cd076d14b))

## 0.2.0-beta.1 - 2026-01-08

[44417e2](44417e24214d83033ef615cd4944e202b34fd4c2)...[7b7829b](7b7829b7329d9031b64b68e69c0aa71431e69a13)

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

- Create release action ([8c438d8](8c438d8164879c4652927392638ed49fd491d160))
- Fix the action ([5fc71cb](5fc71cb50242adfc467987978e1282ba51ba215e))
- Some release action enhancements ([71dd9af](71dd9afa06317a8b4bd2ec97c07cfd6e6d4bd99f))
- Rm build script ([450c71e](450c71ef724ceec4eb723a1e81bd8fb3e851d042))

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
