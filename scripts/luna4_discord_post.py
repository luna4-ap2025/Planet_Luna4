import os
import requests
from datetime import datetime

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

def post_observatory(phase, cycle, resources):
    webhook_url = os.environ.get("DISCORD_OBSERVATORY_WEBHOOK")
    message_id = os.environ.get("OBSERVATORY_MESSAGE_ID")

    if not webhook_url:
        print("No DISCORD_OBSERVATORY_WEBHOOK found")
        return

    content = (
        f"🌑 **Luna4 Observatory**\n"
        f"━━━━━━━━━━━━━━\n"
        f"**Cycle:** {cycle}\n"
        f"**Phase:** {phase}\n"
        f"**Accessible Resources:** {', '.join(resources)}\n\n"
        f"_The surface remains under continuous observation._"
    )

    payload = {"content": content}

    try:
        if message_id:
            edit_url = f"{webhook_url}/messages/{message_id}"
            requests.patch(edit_url, json=payload)
            print("Updated Observatory message")
        else:
            requests.post(webhook_url, json=payload)
            print("Posted new Observatory message")
    except Exception as e:
        print(f"Error updating Observatory: {e}")

def post_changelog(tag, name, body):
    msg = f"🌒 Cycle Adjustment Detected {tag} — \"{name}\""
    if body:
        msg += f" • {body.replace(chr(10),' • ')}"
    post_to_discord("DISCORD_CHANGELOG_WEBHOOK", msg)

def post_experiment(tag, name, url):
    msg = f"🌒 [Experiment] {tag} — \"{name}\" • {url}"
    post_to_discord("FUTURE_WEBHOOK", msg)
