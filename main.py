import bp


def main():
    print("Hello from cdl-stats!")
    player_stats = bp.get_all_player_stats()
    bp.get_player_tags(player_stats)


if __name__ == "__main__":
    main()
