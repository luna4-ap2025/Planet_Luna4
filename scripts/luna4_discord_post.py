import os
import requests
from datetime import datetime
import time

#Post to discord
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

#Post to log in DarkSide of Luna4
def log_darkside(message):
    timestamp = datetime.utcnow().strftime("%Y-%m-%d %H:%M:%S UTC")
    post_to_discord("LOGS_WEBHOOK", f"🛰 {timestamp} | {message}")

#Observatory Def

PHASE_DATA = {
    "New Moon": {
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
    "First Quarter": {
        "emoji": "🌓",
        "illumination": "50%",
        "surface": "Unevenly Lit",
        "resources": ["Oxygen", "Hydrogen"],
        "prophecy": (
            "Light returns unevenly.\n\n"
            "Common matter rises first,\n"
            "preparing the way for greater yield."
        )
    },
    "Full Moon": {
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
    "Last Quarter": {
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
}

    def get_current_phase():
    phases = list(PHASE_DATA.keys())
    PHASE_DURATION = 105  # seconds
    now = int(time.time())
    index = (now // PHASE_DURATION) % len(phases)
    return phases[index]

def post_observatory():
    webhook = os.environ["DISCORD_OBSERVATORY_WEBHOOK"]
    message_id = os.environ.get("OBSERVATORY_MESSAGE_ID")

    phase_name = get_current_phase()
    phase = PHASE_DATA[phase_name]

    CYCLE_DURATION = 420
    cycle = int(time.time()) // CYCLE_DURATION

    timestamp = datetime.utcnow().strftime("%Y-%m-%d %H:%M UTC")

    content = f"""
⋆⭒˚.⋆🌙⋆⭒˚.⋆  L U N A 4   O B S E R V A T O R Y  ⋆⭒˚.⋆🌙⋆⭒˚.⋆

> The moon that feeds your world.
> Quiet. Essential. Always there.

━━━━━━━━━━━━━━━━━━━━━━━

{phase['emoji']} **Current Lunar State**
Cycle: **{cycle}**
Phase: **{phase_name}**

🌘 Illumination: {phase['illumination']}
🌌 Surface Status: {phase['surface']}

━━━━━━━━━━━━━━━━━━━━━━━

🪨 **Resources Available**
""" + "\n".join(f"• {r}" for r in phase["resources"]) + f"""

━━━━━━━━━━━━━━━━━━━━━━━

🔮 **Observatory Reading**

{phase['prophecy']}

━━━━━━━━━━━━━━━━━━━━━━━

📡 *Observatory Status:* Stable  
🕰 *Last updated:* {timestamp}

⋆⁺₊⋆ ☾⋆⁺₊⋆  Luna4 watches. It always does.  ⋆⁺₊⋆ ☾⋆⁺₊⋆
""".strip()

    payload = {"content": content}

    if message_id:
        requests.patch(f"{webhook}/messages/{message_id}", json=payload)
    else:
        requests.post(webhook, json=payload)

last_update = int(os.environ.get("LAST_OBSERVATORY_UPDATE", "0"))
now = int(time.time())

if last_update and now - last_update > 600:
    content += "\n\n⚠️ *Signal degradation detected. Observatory feed unstable.*"

#Changelog log (lol)
def post_changelog(tag, name, body):
    msg = f"🌒 Cycle Adjustment Detected {tag} — \"{name}\""
    if body:
        msg += f" • {body.replace(chr(10),' • ')}"
    post_to_discord("DISCORD_CHANGELOG_WEBHOOK", msg)

#for future ideas
def post_experiment(tag, name, url):
    msg = f"🌒 [Experiment] {tag} — \"{name}\" • {url}"
    post_to_discord("FUTURE_WEBHOOK", msg)
