# Pokemon Detail View Feature - Complete Implementation Proposal

## 📋 Overview

This OpenSpec proposal defines the implementation of a complete Pokemon viewing interface that allows players to:

1. **View their team** - See all 6 Pokemon with quick status overview
2. **Inspect details** - Comprehensive view of each Pokemon including:
   - Basic info (name, species, level, experience)
   - Species racial values (种族值)
   - Calculated stats with IV and Nature adjustments
   - Individual Values breakdown
   - Nature and its stat modifiers
   - Talents/Abilities
   - Current moves and PP
   - Capture information

## 📁 Files in This Proposal

- **proposal.md** - Problem statement and solution overview
- **design.md** - Architecture, UI mockups, data flows
- **tasks.md** - Detailed implementation tasks and testing plan
- **README.md** - This file

## 🎯 Key Features

### Team List View
```
╔════════════════════════════════════════╗
║           你的队伍 (1-6)              ║
╠════════════════════════════════════════╣
║ 1. 皮卡丘 Lv.25 (⚡ 电)               ║
║    HP: ███████░░ 95/120              ║
║ 2. 妙蛙花 Lv.28 (🌿 草/毒)            ║
║    HP: ████████░ 105/130             ║
║ 3. 喷火龙 Lv.30 (🔥 火/飞)            ║
║    HP: ███░░░░░░ 80/140 (昏迷)       ║
╚════════════════════════════════════════╝
```

### Pokemon Detail View
Shows complete information:
- Basic Info (name, type, level, experience)
- Stats with base/calculated comparison
- Individual Values (IV) breakdown
- Nature and ability
- Moves with PP
- Capture location and method

## 💡 Why This Matters

This is a **core Pokemon experience feature**:
- Players understand their team composition
- Learn about Pokemon mechanics (IVs, Nature, stats)
- Make informed team building decisions
- Feel immersed in a complete Pokemon game

## 🔧 Implementation Approach

**Two new menu modules:**
1. `src/cli/team_list_menu.rs` - Team overview
2. `src/cli/pokemon_detail_menu.rs` - Detailed Pokemon info

**Integration points:**
- Location exploration menu → Option 3 "查看队伍"
- Main game menu loop

**Data sources:**
- `PokemonInstance` - Individual Pokemon data
- `PokemonSpecies` - Base stats and species info
- `PokemonStats` - Calculated final stats

## 📊 Scope & Effort

**Phase Breakdown:**
- Phase 1: Team list display (30 min)
- Phase 2: Detail view implementation (1-2 hrs)
- Phase 3: Menu integration (30 min)
- Phase 4: Data integration (30 min)
- Phase 5: Visual enhancements (1 hr)
- Phase 6: Testing & polish (1 hr)
- Phase 7: Documentation (30 min)

**Total: 4-5 hours**

## ✅ Success Criteria

- [ ] All 6 team members display correctly
- [ ] Pokemon detail view shows all stat information
- [ ] IVs and Nature modifiers calculate and display correctly
- [ ] Navigation between Pokemon works smoothly
- [ ] All tests pass
- [ ] Display fits within 80-character terminal width
- [ ] No panics or unhandled errors
- [ ] User understands Pokemon capabilities from interface

## 🚀 Next Steps

1. Review proposal and design
2. Approve approach
3. Execute Phase 1-7 tasks in order
4. Test thoroughly
5. Deploy feature

## 📌 Related Documents

- See `design.md` for architecture and UI mockups
- See `tasks.md` for detailed implementation checklist
- See `proposal.md` for problem statement and benefits
