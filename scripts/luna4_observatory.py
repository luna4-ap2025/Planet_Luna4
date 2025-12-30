import time
import os
import requests

WEBHOOK_OBSERVATORY = os.environ["DISCORD_OBSERVATORY_WEBHOOK"]
MESSAGE_ID = os.environ["OBSERVATORY_MESSAGE_ID"]

PHASES = [
    {
        "emoji": "🌑",
        "name": "New Moon",
        "color": 0x2b2d31,
        "resources": ["Carbon"]
    },
    {
        "emoji": "🌓",
        "name": "First Quarter",
        "color": 0x3498db,
        "resources": ["Oxygen", "Hydrogen"]
    },
    {
        "emoji": "🌕",
        "name": "Full Moon",
        "color": 0xf1c40f,
        "resources": ["All Resources"]
    },
    {
        "emoji": "🌗",
        "name": "Last Quarter",
        "color": 0x1abc9c,
        "resources": ["Oxygen", "Silicon"]
    }
]

CYCLE_DURATION = 420
PHASE_DURATION = 105

# ---- Time math (FIXED) ----
now = int(time.time())
LUNA4_EPOCH = 1767052800  # 30 Dec 2025 00:00 UTC
elapsed = now - LUNA4_EPOCH

cycle = elapsed // CYCLE_DURATION
phase_index = int((elapsed % CYCLE_DURATION) // PHASE_DURATION)
phase = PHASES[phase_index]

# ---- Phase-derived values ----
illumination_map = {0: 0, 1: 50, 2: 100, 3: 50}
illumination = illumination_map[phase_index]

surface_states = {
    0: "Veiled",
    1: "Awakening",
    2: "Exposed",
    3: "Unstable"
}
surface_status = surface_states[phase_index]

resources_block = "\n".join(f"• {r}" for r in phase["resources"])

PROPHECIES = {
    0: "Luna4 has turned inward.\n\nThe surface sleeps beneath shadow,\nand only the patient may gather what lies below.",
    1: "Light returns unevenly.\n\nCommon matter rises first,\npreparing the way for greater yield.",
    2: "Nothing is hidden.\n\nLuna4 offers everything it has,\nand the surface hums with abundance.",
    3: "The light withdraws.\n\nWhat remains is unstable,\na reminder that cycles do not linger."
}
prophecy = PROPHECIES[phase_index]

timestamp = time.strftime("%Y-%m-%d %H:%M UTC", time.gmtime())

# ---- Embed ----
embed = {
    "title": "⋆⭒˚.⋆🌙⋆⭒˚.⋆  L U N A 4   O B S E R V A T O R Y  ⋆⭒˚.⋆🌙⋆⭒˚.⋆",
    "description": f"""
> The moon that feeds your world.
> Quiet. Essential. Always there.

━━━━━━━━━━━━━━━━━━━━━━━

{phase['emoji']} **Current Lunar State**
Cycle: **{cycle}**
Phase: **{phase['name']}**

🌘 Illumination: {illumination}%
🌌 Surface Status: {surface_status}

━━━━━━━━━━━━━━━━━━━━━━━

🪨 **Resources Available**
{resources_block}

━━━━━━━━━━━━━━━━━━━━━━━

🔮 **Observatory Reading**

{prophecy}

━━━━━━━━━━━━━━━━━━━━━━━

📡 *Observatory Status:* Stable
🕰 *Last update:* {timestamp}
""",
    "color": phase["color"],
    "footer": {
        "text": "⋆⁺₊⋆ ☾⋆⁺₊⋆  Luna4 watches. It always does.  ⋆⁺₊⋆ ☾⋆⁺₊⋆"
    }
}

payload = {"embeds": [embed]}

# ---- Update pinned message ----
requests.patch(
    f"{WEBHOOK_OBSERVATORY}/messages/{MESSAGE_ID}",
    json=payload
)
