import os
import requests
from datetime import datetime
import time

# ----------------------------
# Discord helpers
# ----------------------------

def post_to_discord(webhook_env, content):
    webhook_url = os.environ.get(webhook_env)
    if not webhook_url:
        print(f"No webhook found for {webhook_env}")
        return

    try:
        requests.post(webhook_url, json={"content": content})
        print(f"Posted to {webhook_env}")
    except Exception as e:
        print(f"Error posting to {webhook_env}: {e}")

def log_darkside(message):
    timestamp = datetime.utcnow().strftime("%Y-%m-%d %H:%M:%S UTC")
    post_to_discord("LOGS_WEBHOOK", f"🛰 {timestamp} | {message}")

# ----------------------------
# Observatory data
# ----------------------------

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

PHASE_DURATION = 105          # seconds per phase
CYCLE_DURATION = PHASE_DURATION * 4  # full lunar cycle

def get_cycle_and_phase():
    now = int(time.time())
    cycle = now // CYCLE_DURATION
    phase_index = (now % CYCLE_DURATION) // PHASE_DURATION
    phase_name = list(PHASE_DATA.keys())[phase_index]
    return cycle, phase_name


    # PATCH ONLY — never create new messages
    response = requests.patch(
        f"{webhook}/messages/{message_id}",
        json={"content": content}
    )

    if response.status_code >= 300:
        raise RuntimeError(f"Discord PATCH failed: {response.status_code} | {response.text}")

# ----------------------------
# Changelog & future hooks
# ----------------------------

def post_changelog(tag, name, body):
    msg = f"🌒 Cycle Adjustment Detected {tag} — \"{name}\""
    if body:
        msg += f" • {body.replace(chr(10), ' • ')}"
    post_to_discord("DISCORD_CHANGELOG_WEBHOOK", msg)

def post_experiment(tag, name, url):
    msg = f"🌒 [Experiment] {tag} — \"{name}\" • {url}"
    post_to_discord("FUTURE_WEBHOOK", msg)
