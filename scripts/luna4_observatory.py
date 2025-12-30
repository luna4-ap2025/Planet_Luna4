import os
import requests
import time
from datetime import datetime

# ---- Discord helpers ----
def post_to_discord(webhook_env, content):
    webhook_url = os.environ.get(webhook_env)
    if not webhook_url:
        print(f"No webhook found for {webhook_env}")
        return
    payload = {"content": content}
    try:
        requests.post(webhook_url, json=payload)
        print(f"Posted to {webhook_env}")
    except Exception as e:
        print(f"Error posting to {webhook_env}: {e}")

def log_darkside(message):
    timestamp = datetime.utcnow().strftime("%Y-%m-%d %H:%M:%S UTC")
    post_to_discord("LOGS_WEBHOOK", f"🛰 {timestamp} | {message}")

# ---- Phase data ----
PHASES = [
    {
        "name": "New Moon",
        "emoji": "🌑",
        "illumination": "0%",
        "surface": "Veiled",
        "resources": ["Carbon"],
        "prophecy": (
            "Luna4 has turned inward.\n\n"
            "The surface sleeps beneath shadow,\n"
            "and only the patient may gather what lies below.\n"
            "Rare matter surfaces briefly,\n"
            "rewarding those who arrive in silence."
        )
    },
    {
        "name": "First Quarter",
        "emoji": "🌓",
        "illumination": "50%",
        "surface": "Awakening",
        "resources": ["Oxygen", "Hydrogen"],
        "prophecy": (
            "Light returns unevenly.\n\n"
            "Common matter rises first,\n"
            "preparing the way for greater yield."
        )
    },
    {
        "name": "Full Moon",
        "emoji": "🌕",
        "illumination": "100%",
        "surface": "Exposed",
        "resources": ["All Resources"],
        "prophecy": (
            "Nothing is hidden.\n\n"
            "Luna4 offers everything it has,\n"
            "and the surface hums with abundance."
        )
    },
    {
        "name": "Last Quarter",
        "emoji": "🌗",
        "illumination": "50%",
        "surface": "Unstable",
        "resources": ["Oxygen", "Silicon"],
        "prophecy": (
            "The light withdraws.\n\n"
            "What remains is unstable,\n"
            "a reminder that cycles do not linger."
        )
    }
]

# ---- Timing math ----
CYCLE_DURATION = 420  # seconds
PHASE_DURATION = 105  # seconds
LUNA4_EPOCH = 1767052800  # 30 Dec 2025 00:00 UTC
now = int(time.time())
elapsed = now - LUNA4_EPOCH

cycle = elapsed // CYCLE_DURATION
phase_index = int((elapsed % CYCLE_DURATION) // PHASE_DURATION)
phase = PHASES[phase_index]

# ---- Resources & prophecy ----
resources_block = "\n".join(f"• {r}" for r in phase["resources"])
prophecy = phase["prophecy"]
timestamp = datetime.utcnow().strftime("%Y-%m-%d %H:%M UTC")

# ---- Build content ----
content = f"""
⋆⭒˚.⋆🌙⋆⭒˚.⋆  L U N A 4   O B S E R V A T O R Y  ⋆⭒˚.⋆🌙⋆⭒˚.⋆

> The moon that feeds your world.
> Quiet. Essential. Always there.

━━━━━━━━━━━━━━━━━━━━━━━

{phase['emoji']} **Current Lunar State**
Cycle: **{cycle}**
Phase: **{phase['name']}**

🌘 Illumination: {phase['illumination']}
🌌 Surface Status: {phase['surface']}

━━━━━━━━━━━━━━━━━━━━━━━

🪨 **Resources Available**
{resources_block}

━━━━━━━━━━━━━━━━━━━━━━━

🔮 **Observatory Reading**

{prophecy}

━━━━━━━━━━━━━━━━━━━━━━━

📡 *Observatory Status:* Stable
🕰 *Last updated:* {timestamp}

⋆⁺₊⋆ ☾⋆⁺₊⋆  Luna4 watches. It always does.  ⋆⁺₊⋆ ☾⋆⁺₊⋆
""".strip()

# ---- Signal degradation ----
LAST_UPDATE = int(os.environ.get("LAST_OBSERVATORY_UPDATE", "0"))
if LAST_UPDATE and now - LAST_UPDATE > CYCLE_DURATION:
    content += "\n\n⚠️ *Signal degradation detected. Observatory feed unstable.*"

# ---- Send to Discord ----
WEBHOOK_OBSERVATORY = os.environ["DISCORD_OBSERVATORY_WEBHOOK"]
MESSAGE_ID = os.environ["OBSERVATORY_MESSAGE_ID"]

payload = {"embeds": [embed]}  # use the embed, not plain text

requests.patch(
    f"{WEBHOOK_OBSERVATORY}/messages/{MESSAGE_ID}",
    json={"content": content}
)
#log_darkside(f"Observatory cycle {cycle} updated successfully.")
