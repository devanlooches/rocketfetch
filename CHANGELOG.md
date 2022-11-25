# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Miscellaneous Tasks

- Update CHANGELOG ([07816a7](07816a703d148ac5a56dcef8769cc464d80652fb))
- Add git-cliff configuration ([73a22ab](73a22ab9d2c9a857e3e5ea22ea16082c39ded65c))

### Ci

- Remove publishing workflow ([0587426](0587426ad5553e21e1594e411c7eba6d6fc981a4))

## [0.7.2] - 2022-11-24

[5665006](5665006b39ea38991784c7278e272e6f0a3a3928)...[bc66284](bc662840ff776bf8a9143b6f36527d24200ef40a)

### Performance

- Switch to scoped threads and removing cloning ([f3309fd](f3309fd1b16d88bc3968597f5aa6a46b1f54e695))

### Build

- Remove unused dependencies ([d7a023b](d7a023b764c69e209c155a93e840c6da16482760))

## [0.7.1] - 2022-11-06

[85c2d00](85c2d00ddf8d355c639fd47589e1f783feee2ea0)...[5665006](5665006b39ea38991784c7278e272e6f0a3a3928)

### Miscellaneous Tasks

- Update changelog ([155452d](155452da8d55f933299d29cbe0a262b94be9a35e))
- Update dependency versions ([1f6c8be](1f6c8bebb354b039ddf9ca600a0b43f2a776a439))
- Release new version ([5665006](5665006b39ea38991784c7278e272e6f0a3a3928))

## [0.7.0] - 2022-11-06

[a3efd60](a3efd60047d7eb19490b961faead6e09c94cf432)...[85c2d00](85c2d00ddf8d355c639fd47589e1f783feee2ea0)

### Bug Fixes

- Fix os-detection on linux ([e6c36f8](e6c36f8f74b83474dc74649d0e3198a5da607374))
- Fix windows logo function ([2eb5f0c](2eb5f0c4f9e2599f9c65b50d45b9b543fd2519e2))
- Fix macos logo fetching function ([6aa488d](6aa488dba9e9a045e9a48d10c600f496e68db48d))
- Setup for windows to work without unimplemented features out of the box ([799d46b](799d46bf468f97792a52c28aa97de891b57c86a1))
- Fix os name for windows ([0fa2bed](0fa2bed0f7f1b1195b34dc557399ab110a055dd6))

### Features

- Add ubuntu OS Support ([0de451e](0de451e5abca416046c20706b89800aa3490281b))
- Added windows support ([7f6c559](7f6c5593f028e34dff7fe04b4d2c8a0248848b66))

### Miscellaneous Tasks

- Add changelog ([fe55d16](fe55d16585dfc3b385c1fc5df6a2db767bbd9c3b))
- Update versions ([5a0d8cf](5a0d8cf0c85a8218c34f48dde1f6024212f1787b))
- Update versions ([85c2d00](85c2d00ddf8d355c639fd47589e1f783feee2ea0))

### Refactor

- Refactor logo fetching functions ([4cf4b28](4cf4b2848636cbc695e549c425caee97fdcdc74d))

### Testing

- Add more tests for workflow ([b8769d2](b8769d229e7730519b9696dd7dee972542276098))
- Ignore certain tests for workflow ([971e622](971e622f033f6066e8628712a01e55c208d71d71))
- Update tests ([0a53c0d](0a53c0d0903553ec6c7b21dd4ce2b499c815b21d))
- Fix tests for workflow ([109d7fe](109d7feeb88ffad281d0862bfb750eab4e29d270))
- Add seperate windows tests taking out unimplemented features ([8d6e3a8](8d6e3a88523ab601bed981234642f27c2f1d7d8c))
- Add seperate linux tests ([da18ea1](da18ea1e198723c7558e5bacfbc45c9ae79fe4a5))
- Update tests to reflect unimplemented functions ([b2d5ece](b2d5eceed56f8c4f20af2d3017821a28f4b1b319))
- Update linux tests ([d32c9d6](d32c9d6457db811f93df4b150e98769219e86d4a))
- Update linux tests ([6ee2026](6ee2026dfaee58026d7981d98b56102e3818926a))
- Update linux tests ([de24cf5](de24cf54b1308467ae9f952193fbfb6621cfdb9c))
- Update linux tests ([c1d47bd](c1d47bdf405adda543181feea144d0f6839a94c8))
- Update defaults for windows ([9345314](9345314fe7e64f6c659439122aacc96250879a59))

### Ci

- Update workflow ([12bfa8a](12bfa8a36dc8bb130e7284c866f83feb38394d03))
- Add windows test ([6550dbd](6550dbda692983c5e43d7ad80d49f2a2c97104de))
- Update workflow ([865df1a](865df1ab19bc82491bc04fe0cea0c071c52cc12d))
- Update workflow ([85cd368](85cd368c75bd39454c2d8bcb4c2b10ae4589e53f))

## [0.6.13] - 2022-11-05

[9b85f05](9b85f05234e38d5bb6bf9d3f15cfef2a3a74f151)...[a3efd60](a3efd60047d7eb19490b961faead6e09c94cf432)

### Refactor

- Listen to clippy ([94b6781](94b6781b9926fff66de636926d8da93811aedd20))

### Ci

- Fix publishing workflow ([ff9e099](ff9e0996cb617ba832afbd347ce35704f48eb3b2))
- Fix publish workflow ([a3efd60](a3efd60047d7eb19490b961faead6e09c94cf432))

## [0.6.12] - 2022-11-05

[632789a](632789a09c34f73e428c8d15fd7028038e9b79d3)...[9b85f05](9b85f05234e38d5bb6bf9d3f15cfef2a3a74f151)

### Bug Fixes

- Fix github workflow ([4905f9f](4905f9f05dbf4c2e0eb13b65ae9ba946d8d6093c))
- Fix github workflow ([e8926c6](e8926c6996e03c0ee5568e494d28c0ec1061799a))
- Add option to disable line wrap ([f32041f](f32041f7fa24842aadcd544eaf3dd4f86e57ca92))
- Add ability to disable line wrap ([dc22133](dc2213396b523912da2128407fbfcf58fd610225))
- Fix top padding functionality ([e58a32e](e58a32e2d654205152a12c0097aacdd34d685671))
- Fix style of side-block mode ([152ab4d](152ab4d5c5b29033a8fd1d199a62839bcea88d5b))

### Miscellaneous Tasks

- Rename job in github workflow ([cb0062c](cb0062c92582f3e1bb40db5904dae4f317c2b796))

### Refactor

- Remove unneeded error handling ([da0c66f](da0c66f04601d8ef6e58f13ac81e1830cadcb461))

### Testing

- Add tests for ci ([c14c5f9](c14c5f946a60931329fd4a554c72ebf12794f7e9))
- Add tests for github-workflows ([d1ade04](d1ade04fc789acc94a531b531a8299e36f198046))

### Ci

- Update workflow ([218670a](218670a75e3680ae827bb1abf1a07729f0f5fef1))
- Add seperate rocketfetch configuration for the workflow ([b9901a4](b9901a4a4c2e6ae8fbc6e5691d2dcb18abf27f6c))
- Add seperate config file for workflow ([11e95d8](11e95d89c48d1de17b85b9cc2cff10fd099a446d))
- Add job for archlinux ([45f1046](45f1046b4332e49c24a8dc72535fe216a4115c40))
- Add archlinux os ([3dd643c](3dd643cfb69f48183e33c5c2e021340780365a60))
- Add archlinux job ([4062ed8](4062ed8f948fcab666e218375eae48fd42e51ab0))
- Capture stdout of tests ([73157b7](73157b796bba9dcc1574ed836bd632a523d5ef3e))
- Add publish to crates.io action ([9cf0643](9cf064366ef7a47fcf3bc2dd75ef0fb169888418))
- Update workflow ([1c4dd8b](1c4dd8b9626f08996752b52c12889ca435aa5131))
- Update workflow ([e4d50e9](e4d50e9431a74536a955ad25423c92c73c954ad4))
- Update workflow ([afdae4d](afdae4d4d1c6e8b3ddadea6ac20dbfa06ae08166))
- Update workflow ([0772fd3](0772fd37882dcde348759ac091aacb74b7719024))
- Update workflow ([d830fd5](d830fd5fa4071c408f68265f06dd7c4579958c7f))
- Update workflow ([b3d7966](b3d796607864486dba457605f7679d146bec8ff2))
- Update workflow ([8c7252b](8c7252bea2de41f6e0e029ea4ee891ddca1072f8))
- Update workflow ([bdb71b8](bdb71b8984cdcf8d3b9e3c49b022ae804d3592e4))
- Add publish workflow ([9b85f05](9b85f05234e38d5bb6bf9d3f15cfef2a3a74f151))

## [0.6.9] - 2022-10-01

[45cd61a](45cd61a154a8b814c11fdc580452e883b9dcdcc2)...[632789a](632789a09c34f73e428c8d15fd7028038e9b79d3)

### Miscellaneous Tasks

- Update version ([632789a](632789a09c34f73e428c8d15fd7028038e9b79d3))

### Build

- Fix build errors and update all dependencies ([a9340f2](a9340f27b997d7a69fc2a5666c5ceba41ffe1083))

<!-- generated by git-cliff -->
