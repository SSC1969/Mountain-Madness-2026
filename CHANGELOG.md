# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.1](https://github.com/SSC1969/TermFishing/compare/v1.0.0...v1.0.1) - 2026-03-16

### Added

- 7 new fish, including three new legendary fish!

### Fixed

- holding f no longer skips playing the game
- newly added fish now get included in the dex in existing saves

### Other

- Update CI targets and OS versions in cd.yml

## [1.0.0](https://github.com/SSC1969/TermFishing/compare/v0.1.2...v1.0.0) - 2026-03-15

### Added

- added 25 new fish
- added three new fish
- replace base_value with species rarity-based values
- reworked fish generation/value to use normal distribution of averages
- game now starts on the help screen after submitting player name
- selected menu tab is now highlighted in toolbar, improved borders
- add colour highlighting to menus and toolbar
- added display for fish rarities in dex and backpack
- add help menu
- updated fish to use Vec<(String, Style)> for icons
- added data persistency by serializing and saving player struct to game data directory

### Fixed

- made sure all fish display properly in backpack/dex
- removed debug info from ui
- chat no longer prints error messages
- sending a message now displays your own name
- deselected menu item when changing menus
- exit chat when sending message or when fish bites
- species rarity now factors into odds of appearing
- Reworked fish value formula and updated species base values
- species now have more accurate stats in kg using floats
- updated species rarities/multipliers to be more distinct
- purchasing items now properly subtracts money from the player
- ui to vec index map now takes into account item tab properly
- moved backpack ui to internal vec index map into backpack struct so it gets properly serialized
- bite time is now calculated correctly
- shop removes already-owned items when loading a save
- moved save_game() calls to an AppEvent and added autosave on timer
- added proper error handling to a failed game save
- loading a save no longer causes fish to bite immediately
- Fish no longer immediately bite when the game starts
- caught fish no longer appear in the upper-left corner after the catch animation finishes
- game waits to start running until after player inputs name
- Updated toasts to use Vec<(String, Style)> to allow for styling

### Other

- bump version to 1.0.0
- removed unused code and old dummied tests
- refactored Menu enum to use strum for automatic counting instead of using a tracking constant
- updated save/load to use more efficient MessagePack encoding
- updated `Fish` to use reference to Species instead of cloning
- added lazy-static crate to dependencies
- added directories crate to dependencies
- Added rush rod
- Updated hook strength to work the same way as lure mult
- Removed the ability to sell rods
- Added shop menu and allowed for making purchases, assuming the player has enough money
- Added new rod and updated fishing calculations to base lure mult around a default of 100
- Decreased catch animation timer
- Added money variable and the ability to sell items
- Updated catch_fish test to reflect the fact that the player's backpack now starts with an item in it
- Updated undiscovered text in dex to prevent overflowing out of window
- Seperated tools and fish into seperate tabs in the dex and backpack
- Added the ability to equip rods, and added rod stats as a variable in fishing calculations
- Updated backpack to use vec for storing items rather than hashmap - will eventually need to do this for the collection, too
- Updated the Item trait's  method to allow for more complex icon styling, and added that more complex styling to the Rod icon.
- Added Rod data structure and a basic 'Average Rod' example with no functionality
- Update CHANGELOG with version 0.1.1 details
- Clean up CHANGELOG.md by removing duplicates
- release v0.1.2
- Update CHANGELOG to remove 0.1.2 entry
- Downgrade version from 0.1.2 to 0.1.1
- Fixed issues in changelog
- release v0.1.2
- release v0.1.2
- Downgrade version from 0.1.3 to 0.1.2
- release v0.1.3
- Rename binary from 'my-bin' to 'TermFishing'

## [0.1.2](https://github.com/SSC1969/TermFishing/compare/v0.1.1...v0.1.2) - 2026-03-04

### Fixed

- actually properly fixed token generating step in release workflow
- properly added workflow token generating step to release

### Other

- Update CHANGELOG to remove 0.1.2 entry
- Downgrade version from 0.1.2 to 0.1.1
- Fixed issues in changelog
- release v0.1.2
- release v0.1.2
- Downgrade version from 0.1.3 to 0.1.2
- release v0.1.3
- Rename binary from 'my-bin' to 'TermFishing'
- release v0.1.2

## [0.1.1](https://github.com/SSC1969/TermFishing/compare/mm-submission...v0.1.1) - 2026-03-04

### Other

- Fix id-token permission in release workflow
- Reset version to 0.1.0
- added app to allow generating secret keys so cd workflow can be triggered by release-plz
- added cd workflow to run after release-plz published a release
- added (properly working) release-plz workflow
- Updated package metadata
- Added machete to the toolchain, and used it to find and remove unused dependencies (cargo machete --with-metadata)
- Moved chat methods into seperate handler to run on it's own thread more easily, encapsulated all player state management into player.rs, replaced rushed code with marginally better code
- Started bugfixing
- Merge branch 'main' into collection
- Updated items to use an enum-based implementation, and added proper functionality to the collection
- Started completion of collection/dex

## [Mountain Madness Submission] - 2026-03-01

### Other

- Added navigation to the backpack menu
- Updated inventory UI
- Fixed warnings
- Merged
- Updated menu UI
- First draft inventory UI
- implement get_all() for backpack and dex, implement dex
- add fish generation
- add species, including file for all definitions
- derive default
- Updated UI
- add missing module
- Merge branch 'main' into inventory
- quick generate, catch tests
- backpack search, add, remove implemented. player can catch_fish
- Add basic structs + inventory
- Added extra files to structure
- Added skeleton file structure
- Created file skeleton
- Fixed formatting you're welcome Adam <3, again
- Fixed formatting you're welcome Adam <3
- Networking test
