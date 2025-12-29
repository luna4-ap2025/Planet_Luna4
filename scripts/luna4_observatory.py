import time
import json
import os
import requests

WEBHOOK_OBSERVATORY = os.environ["DISCORD_OBSERVATORY_WEBHOOK"]
STATE_FILE = "scripts/observatory_state.json"

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

# ---- Load state ----
if os.path.exists(STATE_FILE):
    with open(STATE_FILE, "r") as f:
        state = json.load(f)
else:
    state = {}

# ---- Compute phase ----
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
        "text": "Luna4 🌙 The moon that feeds your world"
    }
}

payload = {"embeds": [embed]}

# ---- Post or edit ----
if "message_id" in state:
    # Edit existing message
    requests.patch(
        f"{WEBHOOK_OBSERVATORY}/messages/{state['message_id']}",
        json=payload
    )
else:
    # Create message and save ID
    response = requests.post(
        WEBHOOK_OBSERVATORY + "?wait=true",
        json=payload
    ).json()

    state["message_id"] = response["id"]

    with open(STATE_FILE, "w") as f:
        json.dump(state, f)
