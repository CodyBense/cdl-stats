import psycopg2
import os
from dotenv import load_dotenv, dotenv_values


load_dotenv()


# SQL Funcs
def connect() -> None:
    conn = psycopg2.connect(
        dbname=os.getenv("DBNAME"),
        user=os.getenv("DBUSER"),
        host=os.getenv("DBHOST")
        )

    return conn



# tag | kd | hp kd | hp k 10 m | snd kd | ovl kd | ovl k 10 m |
def insert_player(conn, tag: str, kd: float, hp_kd: float, hp_k_10m: float, snd_kd: float, ovl_kd: float, ovl_k_10m: float) -> None:
    cur = conn.cursor()
    cur.execute(
        "INSERT into players(tag, kd, hp_kd, hp_k_10m, snd_kd, ovl_kd, ovl_k_10m) VALUES (%s, %s, %s, %s, %s, %s, %s);",
        (tag, round(kd,2) , round(hp_kd,2) , round(hp_k_10m,2) , round(snd_kd,2) , round(ovl_kd,2) , round(ovl_k_10m,2))
    )
    conn.commit()


def init_data(conn, player_dict: dict) -> None:
    for player in player_dict['result']['data']['json']:
        insert_player(conn, player['player_tag'], player['kd'], player['hp_kd'], player['hp_k_10m'], player['snd_kd'], player['ovl_kd'], player['ovl_k_10m'])


def select_all(conn) -> None:
    cur = conn.cursor()
    cur.execute("SELECT * FROM players;")
    return cur.fetchall()


def select_player(conn, tag: str) -> dict:
    cur = conn.cursor()
    cur.execute(f"SELECT * FROM players WHERE tag='{tag}';")
    stats = cur.fetchone()
    return {
        "tag": stats[0],
        "kd": stats[1],
        "hp_kd": stats[2],
        "hp_k_10m": stats[3],
        "snd_kd": stats[4],
        "ovl_kd": stats[5],
        "ovl_k_10m": stats[6],
    }


# UPDATE table_name SET column1 = value1, column2 = value2, ... WHERE condition;
def update_all_stats(conn, tag: str, kd: float, hp_kd: float, hp_k_10m: float, snd_kd: float, ovl_kd: float, ovl_k_10m: float) -> None:
    cur = conn.cursor()
    cur.execute(f"UPDATE players SET kd = %s, hp_kd = %s, hp_k_10m = %s, snd_kd = %s, ovl_kd = %s, ovl_k_10m = %s WHERE tag = %s;", (round(kd,2) , round(hp_kd,2) , round(hp_k_10m,2) , round(snd_kd,2) , round(ovl_kd,2) , round(ovl_k_10m,2), tag))
    conn.commit()


# Other data funcs
def print_stats(player_stats: dict) -> None:
    print(f"Tag: {player_stats['tag']}\nKD: {player_stats['kd']}\nHP KD: {player_stats['hp_kd']}\nHP K 10m: {player_stats['hp_k_10m']}\nSND KD: {player_stats['snd_kd']}\nOVL KD: {player_stats['ovl_kd']}\nOVL K 10m: {player_stats['ovl_k_10m']}")
