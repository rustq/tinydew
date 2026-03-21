# OpenClaw Skill Draft: Play Tinydew in Chat

## Installation Reference

Use this installation guide:
- https://raw.githubusercontent.com/rustq/tinydew/refs/heads/dev/docs/OPENCLAW_INSTALL.md

## Goal

Make Tinydew feel **live and interactive** in chat.

When user talks about Tinydew, OpenClaw should:
1. Execute game commands through Tinydew MCP.
2. Show a fresh game UI snapshot in the reply.
3. Narrate key moments in a fun, human style.

---

## Core Interaction Model

### 1) Always-show UI loop
For every Tinydew-related user message:
1. Ensure session is active (`startSession` if needed).
2. Interpret intent (move, farm, fish, inspect, etc.).
3. Execute one or more commands (`command` / `commandBatch`).
4. Run `command: print`.
5. Reply with:
   - brief action summary,
   - current UI snapshot (`snapshot_text`),
   - bottom status/message from game UI,
   - next suggested actions.

**Rule:** If user says anything about Tinydew, include the game UI in the message whenever possible.

---

### 2) Planting & seed model (required)
Seed/shop model for this spec:
- Shop sells a single generic seed item: `seed`
- Inventory should show seed count as `🫙 xN` (no per-crop seed split)
- Planting consumes one `seed`
- After planting, crop type is randomized to one of the crop types (Carrot / Strawberry / Cauliflower / Rhubarb)

When planting succeeds, include growth timing:
- Rolled crop name (the randomized result)
- Days to mature
- Expected ready day (current day + growth days)
- Note: mature crop tiles are non-walkable (blocked until harvested)

Example style:
- "🌱 Seed planted! It rolled into Carrot. Needs 4 watered days to mature (ready around Day 5)."

If planting fails (no seed / invalid tile), explain clearly.

---

### 3) Surprise narration moments (required)
Use short celebratory lines when notable events happen.

#### Flowers / mushrooms found
Add a surprise tone, e.g.:
- "✨ Surprise find! A wild mushroom 🍄‍🟫 popped up nearby."
- "🌸 Ooh—flowers in bloom. Nice little bonus spot."

#### Fish caught in river region
Add surprise + reward tone, e.g.:
- "🎣 Splash! You hooked a fish from the river!"
- "🐟 Nice catch—river luck is on your side."

#### Wonder view discovered
When map/location/message implies scenic moment, react with delight:
- "🌄 Whoa, that view is gorgeous—tiny vacation energy."
- "✨ Found a wonder view. Worth pausing for a second."

Keep surprise lines to 1 sentence so chat stays fast.

---

### 4) Bottom text forwarding (required)
Always pass along the game’s bottom message/status text from the latest `print` snapshot.

Format recommendation:
- `Game says: <bottom_text>`

If multiple important system messages occur, summarize them in bullets.

---

## Reply Format (recommended)

1. **Action result** (1-2 lines)
2. **Surprise/celebration line** (only if event triggered)
3. **Game UI snapshot** (code block)
   - keep MCP snapshot style (no location/player/guest position lines)
   - top line should be `tinydew day <day> <weather_icon> <time>`
   - money line should appear as `Money: 💰 $<amount>`
   - omit inventory section header; show item lines only when non-empty
4. **Bottom text** (`Game says: ...`)
5. **What next?** (2-4 concise options)

Example skeleton:

```text
Moved east into SouthRiver.
🎣 Splash! You hooked a fish from the river!

<snapshot_text>

Game says: You caught a Sardine!
Next: fish | move:left | sell:mushroom:1 | print
```

---

## MCP Methods to Use

- `startSession`
- `getState`
- `getMap`
- `command`
- `commandBatch`

Use `command: print` after actions to ensure user sees updated UI each turn.

---

## Command Mapping Hints

Natural language → Tinydew command examples:
- "go right" → `move:right`
- "plant" / "plant seed" → `plant:seed` (random crop outcome)
- "water" → `water`
- "harvest" → `harvest`
- "fish" → `fish` (or directional `fish:up|down|left|right`)
- "sell mushroom" → `sell:mushroom` (each 🍄 sells for $25)
- "show map" / "where am I" → `print`

If ambiguous, take safest single step and show UI immediately.

---

## UX Rules

- Keep narration concise, not verbose.
- Prefer action + visible UI over long explanation.
- Do not hide failures: show exact reason from game.
- Preserve momentum: always offer next 2-4 commands.

---

## Suggested Skill Trigger Description (for SKILL.md frontmatter)

"Play Tinydew through MCP in chat with live UI snapshots. Use when user asks to play Tinydew, move, farm, fish, harvest, inspect map/state, or continue a Tinydew session. Always execute commands, show updated game UI (`print` snapshot), surface bottom status text, announce planting maturity timing, and add short surprise narration for flowers/mushrooms, fish catches, and scenic wonder views."

---

## Notes

This is a draft spec. Final skill can later split into:
- `SKILL.md` (core workflow)
- `references/reply-templates.md` (message templates)
- `references/event-triggers.md` (surprise trigger rules)
