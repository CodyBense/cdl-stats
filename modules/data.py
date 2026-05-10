import psycopg2
import os
from dotenv import load_dotenv, dotenv_values


load_dotenv()


def connect():
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


def select_all(conn):
    cur = conn.cursor()
    cur.execute("SELECT * FROM players;")
    return cur.fetchall()
