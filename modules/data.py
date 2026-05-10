import psycopg2
import os
from dotenv import load_dotenv, dotenv_values


load_dotenv()


def connection():
    conn = psycopg2.connect(
        dbname=os.getenv("DBNAME"),
        user=os.getenv("DBUSER"),
        password=os.getenv("DBPASSWORD"),
        host=os.getenv("DBHOST")
        )

    return conn



# uuid | tag | kd | hp kd | hp k 10 m | snd kd | ovl kd | ovl k 10 m |
def insert_player(conn: connection, tag: str, kd: float, hp_kd: float, hp_k_10m: float, snd_kd: float, ovl_kd: float, ovl_k_10m: float):
    cur = conn.cursor()
    cur.execute(
        "INSERT into players(tag, kd, hp_kd, hp_k_10m, snd_kd, ovl_kd, ovl_k_10m) VALUES (%s, %.2f, %.2f, %.2f, %.2f, %.2f, %.2f);",
        (tag, kd, hp_kd, hp_k_10m, snd_kd, ovl_kd, ovl_k_10m)
    )
