import requests
import json


ALL_PLAYER_STATS_URL = "https://breakingpoint.gg/api/trpc/playerStats.getAggregatedOrderedPlayerStats?input=%7B%22json%22%3A%7B%22eventType%22%3A%5B%5D%2C%22mapId%22%3A%5B38%2C21%2C59%2C46%2C53%2C47%2C27%2C76%2C29%2C20%2C80%2C67%2C66%2C18%2C45%2C68%2C28%2C10%2C11%2C15%2C69%2C23%2C70%2C60%2C8%2C54%2C79%2C16%2C25%2C61%2C78%2C55%2C48%2C44%2C56%2C49%2C32%2C9%2C13%2C72%2C30%2C31%2C62%2C12%2C22%2C17%2C57%2C50%2C81%2C41%2C19%2C73%2C51%2C40%2C75%2C39%2C36%2C63%2C77%2C74%2C58%2C33%2C42%2C52%2C24%2C35%2C34%2C71%2C26%2C64%2C65%2C43%2C37%5D%2C%22modeId%22%3A%5B1%2C2%2C3%2C4%2C5%5D%2C%22eventId%22%3A%5B%5D%2C%22teamId%22%3A%5B%5D%2C%22onlyCDLStats%22%3Atrue%2C%22onlyChallengersStats%22%3Afalse%2C%22seasonId%22%3A2026%2C%22startAt%22%3Anull%2C%22endAt%22%3Anull%2C%22aggregateMatchStats%22%3Atrue%7D%2C%22meta%22%3A%7B%22values%22%3A%7B%22startAt%22%3A%5B%22undefined%22%5D%2C%22endAt%22%3A%5B%22undefined%22%5D%7D%7D%7D"


def get_all_player_stats() -> dict:
    player_stats_json = requests.get(ALL_PLAYER_STATS_URL).text
    player_stats_dict = json.loads(player_stats_json)
    return player_stats_dict


def get_player_tags(player_dict: dict) -> str:
    for player in player_dict['result']['data']['json']:
        return player['player_tag']


def get_player_stats(tag: str, player_dict: dict) -> dict:
    for player in player_dict['result']['data']['json']:
        if player['player_tag'].lower() == tag.lower():
            return player


def get_player_kd(tag: str, player_dict: dict) -> float:
    for player in player_dict['result']['data']['json']:
        if player['player_tag'].lower() == tag.lower():
             return player['kd']


def get_player_hp_kd(tag: str, player_dict: dict) -> float:
    for player in player_dict['result']['data']['json']:
        if player['player_tag'].lower() == tag.lower():
             return player['hp_kd']


def get_player_hp_k_10m(tag: str, player_dict: dict) -> float:
    for player in player_dict['result']['data']['json']:
        if player['player_tag'].lower() == tag.lower():
             return player['hp_k_10m']


def get_player_snd_kd(tag: str, player_dict: dict) -> float:
    for player in player_dict['result']['data']['json']:
        if player['player_tag'].lower() == tag.lower():
             return player['snd_kd']


def get_player_ovl_kd(tag: str, player_dict: dict) -> float:
    for player in player_dict['result']['data']['json']:
        if player['player_tag'].lower() == tag.lower():
             return player['ovl_kd']


def get_player_ovl_k_10m(tag: str, player_dict: dict) -> float:
    for player in player_dict['result']['data']['json']:
        if player['player_tag'].lower() == tag.lower():
             return player['ovl_k_10m']
