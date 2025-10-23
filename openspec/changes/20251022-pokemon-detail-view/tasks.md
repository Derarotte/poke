# Implementation Tasks: Pokemon Detail View

## Phase 1: Core Team List Display

### Task 1.1: Create team_list_menu.rs module
- **File**: `src/cli/team_list_menu.rs`
- **Work**: Implement TeamListMenu struct with static methods
- **Methods to create**:
  - `display_team_list(team: &[PokemonInstance], current_index: usize)` - Display all 6 team members with status
  - `get_hp_bar(hp: u32, max_hp: u32) -> String` - Create visual HP bar
  - `get_pokemon_status_icon(is_fainted: bool) -> &'static str` - Status indicator
  - `get_input() -> String` - Read user input
- **Testing**: Verify output formatting matches design

### Task 1.2: Integrate team list into main game menu
- **File**: `src/cli/mod.rs`
- **Work**: Add pub use for TeamListMenu
- **Location**: Update the location exploration menu to call team list when option 3 is selected
- **Testing**: Menu navigation works correctly

## Phase 2: Pokemon Detail View

### Task 2.1: Create pokemon_detail_menu.rs module
- **File**: `src/cli/pokemon_detail_menu.rs`
- **Work**: Implement PokemonDetailMenu struct
- **Methods**:
  - `display_pokemon_detail(pokemon: &PokemonInstance, species: &PokemonSpecies, stats: &PokemonStats, team_index: usize, team_size: usize)`
  - `display_basic_info(name: String, species_id: u32, level: u32, exp: u32)`
  - `display_stats_comparison(base_stats: &Stat, calculated_stats: &PokemonStats, ivs: &IndividualValues, nature_mults: &NatureMultipliers)`
  - `display_nature_and_iv(nature: &Nature, talent: &Talent, ivs: &IndividualValues)`
  - `display_moves(moves: &[Move])`
  - `display_capture_info(caught_with: &str, location_id: u32, timestamp: u64)`
  - `display_navigation(current: usize, total: usize)` - Show < > navigation
  - `get_input() -> String`
- **Dependencies**: Uses existing PokemonStats, NatureMultipliers
- **Testing**: All sections format correctly, stat calculations accurate

### Task 2.2: Add helper functions for formatting
- **File**: `src/cli/pokemon_detail_menu.rs`
- **Work**: Create formatting helpers
  - `format_nature_name(nature: &Nature) -> String` - Chinese nature names
  - `format_talent_name(talent: &Talent) -> String` - Chinese talent names
  - `format_stat_bar(value: u32, max_value: u32) -> String` - Visual stat bar
  - `format_timestamp(timestamp: u64) -> String` - Format capture date
  - `format_type(pokemon_type: PokemonType) -> String` - Type display with emoji
  - `calculate_exp_percentage(current: u32, required: u32) -> u32`
- **Testing**: Format functions produce correct output

### Task 2.3: Implement detail view display logic
- **File**: `src/cli/pokemon_detail_menu.rs`
- **Work**: Main display function that combines all sections
  - Renders sections in order (Basic → Stats → Nature/IV → Moves → Capture)
  - Handles scrolling for long content if needed
  - Shows navigation footer with < > arrows
- **Testing**: All information displays, no overflow errors

## Phase 3: Menu Integration & Navigation

### Task 3.1: Create detail view menu handler
- **File**: `src/cli/pokemon_detail_menu.rs`
- **Work**: Implement menu loop for detail view
  - `show_detail_menu(pokemon_index: usize, team: &[PokemonInstance]) -> Result<MenuAction, String>`
  - Handle arrow keys or A/D for navigation between team members
  - Handle 0 or ESC to return to team list
  - MenuAction enum: Next, Previous, ReturnToTeam
- **Testing**: Navigation between team members works, returns function correctly

### Task 3.2: Update location exploration handler
- **File**: `src/handlers/exploration_handler.rs` or relevant menu handler
- **Work**: Add case for option 3 "查看队伍"
  - Call team list menu when selected
  - Handle team list navigation (which Pokemon selected → call detail view)
  - Return to exploration menu on exit
- **Testing**: Menu flow works end-to-end

### Task 3.3: Test menu integration
- **Work**: Manual testing of complete flow
  - Start game → Enter location → Select "查看队伍" → View team list → Select Pokemon → View details → Navigate → Return
- **Testing**: All navigation paths work correctly

## Phase 4: Data Integration

### Task 4.1: Add required imports and dependencies
- **Files**: `src/cli/pokemon_detail_menu.rs`, `src/cli/team_list_menu.rs`
- **Work**:
  - Import PokemonInstance, PokemonStats, PokemonSpecies from pokemon_generator
  - Import NatureMultipliers for stat display
  - Import Pokemon type info
  - Import Location info for capture location names
- **Testing**: No compilation errors

### Task 4.2: Implement stat calculation integration
- **Work**: Use `calculate_pokemon_stats()` from pokemon_generator
  - Get species info from species_id
  - Calculate final stats with IVs and Nature
  - Display both base and calculated stats
- **Testing**: Stats match game mechanics

### Task 4.3: Handle missing data gracefully
- **Work**: Add fallback displays for:
  - Unknown species (show species_id if name not found)
  - Missing moves (show empty moves section)
  - Unknown locations (show location_id instead of name)
- **Testing**: No panic errors, graceful degradation

## Phase 5: Data Display Enhancements

### Task 5.1: Implement stat bar visualization
- **Work**: Create visual stat bars (like ████░░░░░░)
  - 10-character bar for HP display
  - 12-character bar for stat displays
  - Percentage-based fill
- **Testing**: Bars render correctly for various values

### Task 5.2: Add color/emoji support (optional enhancement)
- **Work**: If using crossterm/colored, add:
  - Type icons (🔥 for Fire, ⚡ for Electric, etc.)
  - Nature/Talent icons
  - HP status colors (green/yellow/red)
- **Testing**: Display is readable and attractive

### Task 5.3: Optimize display for terminal width
- **Work**: Ensure all output fits in 80-character terminal width
  - Truncate long names if needed
  - Wrap move descriptions
  - Handle variable terminal sizes
- **Testing**: Manual testing at different terminal widths

## Phase 6: Testing & Polish

### Task 6.1: Unit tests for display functions
- **File**: `src/cli/pokemon_detail_menu.rs` (tests module)
- **Tests**:
  - `test_stat_bar_formatting` - Bar lengths correct
  - `test_nature_names_chinese` - Nature names display correctly
  - `test_timestamp_formatting` - Dates format as expected
  - `test_iv_total_calculation` - IV sum is correct
- **Coverage**: ≥ 80% of new functions

### Task 6.2: Integration tests for menu flow
- **Work**: Create integration test for complete flow
  - Create test player with Pokemon team
  - Simulate menu selections
  - Verify correct detail view rendered
  - Verify navigation works
- **Testing**: All paths pass

### Task 6.3: Manual testing checklist
- [ ] View team with all 6 Pokemon (some fainted)
- [ ] View team with < 6 Pokemon
- [ ] View Pokemon detail with all stats correct
- [ ] Navigate between team members with arrows
- [ ] Return to team list and back multiple times
- [ ] Check formatting at 80-column width
- [ ] Test with Pokemon having different natures
- [ ] Test with different IV values
- [ ] Test with Pokemon at different levels
- [ ] Verify move PP displays correctly
- [ ] Check capture info accuracy

## Phase 7: Documentation

### Task 7.1: Add rustdoc comments
- **Work**: Document all public methods and structs
- **Coverage**: All public functions have doc comments

### Task 7.2: Update user guide
- **File**: Create `POKEMON_VIEWING.md` if needed
- **Content**: Explain how to view team and what each stat means

## Dependency Graph

```
Phase 1 (Task 1.1, 1.2)
    ↓
Phase 2 (Task 2.1, 2.2, 2.3)
    ↓
Phase 3 (Task 3.1, 3.2, 3.3)
    ↓
Phase 4 (Task 4.1, 4.2, 4.3)
    ↓
Phase 5 (Task 5.1, 5.2, 5.3) - Parallel with Phase 6
    ↓
Phase 6 (Task 6.1, 6.2, 6.3)
    ↓
Phase 7 (Task 7.1, 7.2)
```

## Estimated Timeline

- Phase 1: 30 minutes
- Phase 2: 1-2 hours (depends on display complexity)
- Phase 3: 30 minutes
- Phase 4: 30 minutes
- Phase 5: 1 hour
- Phase 6: 1 hour
- Phase 7: 30 minutes

**Total: 4-5 hours of focused development**

## Success Metrics

✅ All 6 team members display in list view
✅ Detail view shows all stat information
✅ IVs and Nature modifiers calculate correctly
✅ Navigation between Pokemon works smoothly
✅ All tests pass
✅ Display fits in 80-column terminal
✅ No panics or error messages from menu
✅ User can understand Pokemon capabilities from display
