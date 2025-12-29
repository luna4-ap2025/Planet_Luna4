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
    msg = f"🌑 Luna4 Observatory | Cycle {cycle} | Phase {phase} | Resources: {', '.join(resources)}"
    post_to_discord("DISCORD_OBSERVATORY_WEBHOOK", msg)

def post_changelog(tag, name, body):
    msg = f"🌒 Cycle Adjustment Detected {tag} — \"{name}\""
    if body:
        msg += f" • {body.replace(chr(10),' • ')}"
    post_to_discord("DISCORD_CHANGELOG_WEBHOOK", msg)

def post_experiment(tag, name, url):
    msg = f"🌒 [Experiment] {tag} — \"{name}\" • {url}"
    post_to_discord("FUTURE_WEBHOOK", msg)
