import time
import json
import os
import requests

WEBHOOK_OBSERVATORY = os.environ["DISCORD_OBSERVATORY_WEBHOOK"]

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

now = int(time.time())
cycle = now // CYCLE_DURATION
phase_index = (now % CYCLE_DURATION) // PHASE_DURATION
phase = PHASES[int(phase_index)]

time_remaining = PHASE_DURATION - (now % PHASE_DURATION)

embed = {
    "title": f"{phase['emoji']} Luna4 Observatory",
    "description": (
        f"**Phase:** {phase['name']}\n"
        f"**Cycle:** #{cycle}\n"
        f"**Time Remaining:** {time_remaining}s\n\n"
        f"**Available Resources:**\n"
        + "\n".join(f"• {r}" for r in phase["resources"])
    ),
    "color": phase["color"],
    "footer": {
        "text": "Luna4 • The moon that feeds your world"
    }
}

payload = {
    "embeds": [embed]
}

# --- Edit last message if it exists ---
history = requests.get(f"{WEBHOOK_OBSERVATORY}/messages?limit=1").json()

if history:
    message_id = history[0]["id"]
    requests.patch(
        f"{WEBHOOK_OBSERVATORY}/messages/{message_id}",
        json=payload
    )
else:
    requests.post(WEBHOOK_OBSERVATORY, json=payload)
