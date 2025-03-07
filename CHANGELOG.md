# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.2.0 (2025-03-07)

### Chore

 - <csr-id-1c5ac77681ee1f446cd5af527e151d456cf69838/> add error types
 - <csr-id-b7a64a7b7a2973a4923b7a3abad6656c60656c76/> add error types
 - <csr-id-701e5dfe2b1aeb70a58c45d1dad705b7a2a377d7/> fix tags
 - <csr-id-a8528e3deb827643cd4fda69245fb86218531961/> migrate to rust 2024 and some ci
 - <csr-id-8cff501b2278c3407ebaeb582f2e0abda5f8e27d/> fix typo
 - <csr-id-fb5ee8d35b9c955d70973cd3ce330767ee1d10de/> fix os parity checks
 - <csr-id-e95d8c35b1c924de1feaf011872781f711f2a6ad/> renamed commit hook name format
 - <csr-id-4bb574f5a0a2cd318ff7b18286c6856cd56c4aa0/> add sample cmd usages and deps
 - <csr-id-3685f3c011840588ff892c7c57259dd62c6f2477/> add rules

### Documentation

 - <csr-id-b6a4b5561581c9d96e4ff3794c083d7cccc4356e/> windows hook_executor is implemented and fairly stable now
 - <csr-id-2cb356903243c6a8aee7e7d67f78930fc43e41e5/> add shields badge
 - <csr-id-e79bafc4122361dbfda32b87c40f415397a94413/> add arch docs
 - <csr-id-fc72b4b6acf1914f847cfbd63653b314240eb338/> add workflows

### New Features

 - <csr-id-f39eacb0f6235a8be0ea4eafea7cad363923f13a/> add update_cwd_path method to GitClone and InitGitRepo for setting current working directory
 - <csr-id-dd76e5154ec3681aba00f23d945802ee5197305c/> add new error types and update module visibility
 - <csr-id-b93abfc35d659087988f1a1bbad73aa265dc4cda/> add new error types and update module visibility
 - <csr-id-daa823ffdb57d1f71c5c889eb9e7a53a3a25dbbb/> implement full git add logic using libgit2
   This commit introduces a complete implementation of git add functionality in the GitAdd event. Instead of using a placeholder command, the new code leverages libgit2 (via the git2 crate) to open the repository, retrieve the index, add all files recursively, and write the index to disk. Detailed error handling is provided via BGitError.
 - <csr-id-dd2a5dcf2a2938686ca938184e4df79812161fe3/> improved ui for cli
 - <csr-id-82c701afb5230e3e759875103806c9f680657aec/> common action store implemented
 - <csr-id-3b8196f3d7008f404af5baa943bd867c8e25098b/> Heap for error type and refactor and hook executor
 - <csr-id-ff7a9b546891852208c01596a0d10ba387a6eddc/> code structure
 - <csr-id-ec2b4318007176ab9fcc8673c37e15e07ad90c14/> add MIT license
 - <csr-id-22aa5c046d2d68e834a88136526bee58658637df/> initial commit

### Bug Fixes

 - <csr-id-add40ef35161ba0efa46f5b363dd6c560f0279fb/> fix windows ci
 - <csr-id-1b7e460a2394bea0c9e69495513a2f3e326067fe/> fix windows ci
 - <csr-id-ec365a0a575fc19ace3ce458041f3c916260692e/> fix prompt dialog mangled into progress bar

### Other

 - <csr-id-c555361aa97d43c1b86840401c7ebe883c544dfe/> windows runners as well
 - <csr-id-a47e1126912a9e84a7dd48e8ad38386ed6c5057e/> windows runners as well
 - <csr-id-f8bdce230d3edc08a2944c335e648e2835eabd48/> code cov check
 - <csr-id-ea17e150e9839dede10ed3f568d67f22bc8b7416/> fix
 - <csr-id-376da8460a8e0bd4c45d9cbc582365a959e399a3/> metadata changes
 - <csr-id-fb00861d9ca0b9498f2efb87543735a6ce3849c3/> add action scripts for test and build

### Refactor

 - <csr-id-8045ea32b7a5e4b259934541b72ad9d285d84bda/> remove old InitGitRepo action and replace with new prompt implementation
 - <csr-id-dd4c718df7b0f27c9498cfa481866c47dbda18bd/> remove name, description from trait definition for new

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 50 commits contributed to the release over the course of 319 calendar days.
 - 34 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Windows hook_executor is implemented and fairly stable now ([`b6a4b55`](https://github.com/Gyan172004/bgit/commit/b6a4b5561581c9d96e4ff3794c083d7cccc4356e))
    - Add update_cwd_path method to GitClone and InitGitRepo for setting current working directory ([`f39eacb`](https://github.com/Gyan172004/bgit/commit/f39eacb0f6235a8be0ea4eafea7cad363923f13a))
    - Remove old InitGitRepo action and replace with new prompt implementation ([`8045ea3`](https://github.com/Gyan172004/bgit/commit/8045ea32b7a5e4b259934541b72ad9d285d84bda))
    - Remove name, description from trait definition for new ([`dd4c718`](https://github.com/Gyan172004/bgit/commit/dd4c718df7b0f27c9498cfa481866c47dbda18bd))
    - Implemented tasks ask_to_init_clone_git , init_git_repo , ask_to_clone_git_repo ; events git clone and git init and some minor refactoring ([`b937519`](https://github.com/Gyan172004/bgit/commit/b937519a495686e00cb853c5acb38b5df756d9ad))
    - Merge pull request #2 from Him7n/main ([`cbb4c3a`](https://github.com/Gyan172004/bgit/commit/cbb4c3a8e3d0b4f5007520e9309dacca0f5d4dd5))
    - Fix : Repository Discover ([`dff1c8b`](https://github.com/Gyan172004/bgit/commit/dff1c8b6fd88704748c11a8509f0b2f94572e318))
    - Add new error types and update module visibility ([`dd76e51`](https://github.com/Gyan172004/bgit/commit/dd76e5154ec3681aba00f23d945802ee5197305c))
    - Fix windows ci ([`add40ef`](https://github.com/Gyan172004/bgit/commit/add40ef35161ba0efa46f5b363dd6c560f0279fb))
    - Windows runners as well ([`c555361`](https://github.com/Gyan172004/bgit/commit/c555361aa97d43c1b86840401c7ebe883c544dfe))
    - Add error types ([`1c5ac77`](https://github.com/Gyan172004/bgit/commit/1c5ac77681ee1f446cd5af527e151d456cf69838))
    - Add new error types and update module visibility ([`b93abfc`](https://github.com/Gyan172004/bgit/commit/b93abfc35d659087988f1a1bbad73aa265dc4cda))
    - Fix windows ci ([`1b7e460`](https://github.com/Gyan172004/bgit/commit/1b7e460a2394bea0c9e69495513a2f3e326067fe))
    - Windows runners as well ([`a47e112`](https://github.com/Gyan172004/bgit/commit/a47e1126912a9e84a7dd48e8ad38386ed6c5057e))
    - Add error types ([`b7a64a7`](https://github.com/Gyan172004/bgit/commit/b7a64a7b7a2973a4923b7a3abad6656c60656c76))
    - Fix : linting error ([`ff0c256`](https://github.com/Gyan172004/bgit/commit/ff0c256a17b6e4bb89e8338e540e33086e808705))
    - Implement full git add logic using libgit2 ([`daa823f`](https://github.com/Gyan172004/bgit/commit/daa823ffdb57d1f71c5c889eb9e7a53a3a25dbbb))
    - Add shields badge ([`2cb3569`](https://github.com/Gyan172004/bgit/commit/2cb356903243c6a8aee7e7d67f78930fc43e41e5))
    - Fix tags ([`701e5df`](https://github.com/Gyan172004/bgit/commit/701e5dfe2b1aeb70a58c45d1dad705b7a2a377d7))
    - Code cov check ([`f8bdce2`](https://github.com/Gyan172004/bgit/commit/f8bdce230d3edc08a2944c335e648e2835eabd48))
    - Minor changes ([`3d5ffc4`](https://github.com/Gyan172004/bgit/commit/3d5ffc4c732f186ff1c17ae3d9e16c3d0fb17b47))
    - Fix ([`ea17e15`](https://github.com/Gyan172004/bgit/commit/ea17e150e9839dede10ed3f568d67f22bc8b7416))
    - Migrate to rust 2024 and some ci ([`a8528e3`](https://github.com/Gyan172004/bgit/commit/a8528e3deb827643cd4fda69245fb86218531961))
    - Implemented hook execution for Windows ([`0693a3d`](https://github.com/Gyan172004/bgit/commit/0693a3d230fac16f4a6f9334b31c6f618cd74ffe))
    - Fix typo ([`8cff501`](https://github.com/Gyan172004/bgit/commit/8cff501b2278c3407ebaeb582f2e0abda5f8e27d))
    - Add arch docs ([`e79bafc`](https://github.com/Gyan172004/bgit/commit/e79bafc4122361dbfda32b87c40f415397a94413))
    - Fix prompt dialog mangled into progress bar ([`ec365a0`](https://github.com/Gyan172004/bgit/commit/ec365a0a575fc19ace3ce458041f3c916260692e))
    - Metadata changes ([`376da84`](https://github.com/Gyan172004/bgit/commit/376da8460a8e0bd4c45d9cbc582365a959e399a3))
    - Fix os parity checks ([`fb5ee8d`](https://github.com/Gyan172004/bgit/commit/fb5ee8d35b9c955d70973cd3ce330767ee1d10de))
    - Improved ui for cli ([`dd2a5dc`](https://github.com/Gyan172004/bgit/commit/dd2a5dcf2a2938686ca938184e4df79812161fe3))
    - Renamed commit hook name format ([`e95d8c3`](https://github.com/Gyan172004/bgit/commit/e95d8c35b1c924de1feaf011872781f711f2a6ad))
    - Common action store implemented ([`82c701a`](https://github.com/Gyan172004/bgit/commit/82c701afb5230e3e759875103806c9f680657aec))
    - Heap for error type and refactor and hook executor ([`3b8196f`](https://github.com/Gyan172004/bgit/commit/3b8196f3d7008f404af5baa943bd867c8e25098b))
    - Fixed error design ([`966f828`](https://github.com/Gyan172004/bgit/commit/966f828271c587a81469a6042c5638b3e915f655))
    - Add more graphs ([`37de9a2`](https://github.com/Gyan172004/bgit/commit/37de9a29ad57d34b4fb6412bab4444fae9a4a90e))
    - Git stash ([`fe03cc3`](https://github.com/Gyan172004/bgit/commit/fe03cc3cae05e3ff9126431d1dfdf2bc5222265c))
    - Some def changes ([`5721457`](https://github.com/Gyan172004/bgit/commit/5721457c63a7312fb31781c1f489a0d3e626a02c))
    - Some def changes ([`ecdae38`](https://github.com/Gyan172004/bgit/commit/ecdae38edd9316d7026cb8bfa75662a2f0465ee6))
    - Is git repo ([`420add0`](https://github.com/Gyan172004/bgit/commit/420add022d43ecb2b242876736e7f7fa58809824))
    - Add dummy task ([`577d1ee`](https://github.com/Gyan172004/bgit/commit/577d1ee9f0443ffab76983d55dd589d726c0cc10))
    - Some fix in data structures ([`d2badef`](https://github.com/Gyan172004/bgit/commit/d2badef3dc6bb7c150199dcf63e8dea7f44c0b31))
    - Intial prototype ([`fea9541`](https://github.com/Gyan172004/bgit/commit/fea9541659a4f7117dc7ce596c15c5afe76273e4))
    - Welp ([`ae50a32`](https://github.com/Gyan172004/bgit/commit/ae50a3267df8e7f48878a1101b2d623da68e05ac))
    - Code structure ([`ff7a9b5`](https://github.com/Gyan172004/bgit/commit/ff7a9b546891852208c01596a0d10ba387a6eddc))
    - Add sample cmd usages and deps ([`4bb574f`](https://github.com/Gyan172004/bgit/commit/4bb574f5a0a2cd318ff7b18286c6856cd56c4aa0))
    - Add rules ([`3685f3c`](https://github.com/Gyan172004/bgit/commit/3685f3c011840588ff892c7c57259dd62c6f2477))
    - Add workflows ([`fc72b4b`](https://github.com/Gyan172004/bgit/commit/fc72b4b6acf1914f847cfbd63653b314240eb338))
    - Add action scripts for test and build ([`fb00861`](https://github.com/Gyan172004/bgit/commit/fb00861d9ca0b9498f2efb87543735a6ce3849c3))
    - Add MIT license ([`ec2b431`](https://github.com/Gyan172004/bgit/commit/ec2b4318007176ab9fcc8673c37e15e07ad90c14))
    - Initial commit ([`22aa5c0`](https://github.com/Gyan172004/bgit/commit/22aa5c046d2d68e834a88136526bee58658637df))
</details>

