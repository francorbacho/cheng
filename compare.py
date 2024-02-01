#!/usr/bin/python

import subprocess
import sys
import os

STARTING_FEN = "4r3/p7/4nk2/3p1Bp1/3P2P1/5PK1/8/1R6 w - - 4 30"


class WhiteWinner(Exception):
    pass


class BlackWinner(Exception):
    pass


class Draw(Exception):
    pass


def print_help() -> None:
    print(f"usage: {sys.argv[0]} <engine1> <engine2>")


def feed(engine: str, fen: str, move: str) -> str:
    cmd = f"fen {fen}\nfeed {move}\nd\n"
    res = subprocess.run([engine], input=cmd.encode(), capture_output=True)
    res = res.stdout.decode().strip().split("\n")
    new_fen = res[-2].split("fen: ")[1]
    result = res[-1].split("result: ")[1]

    if result == "Undecided":
        return new_fen
    if "winner: White" in result:
        raise WhiteWinner()
    if "winner: Black" in result:
        raise BlackWinner()
    if "Draw" in result:
        raise Draw()

    raise Exception()


def get_move(engine: str, fen: str) -> str:
    cmd = f"fen {fen}\nev\n"
    res = subprocess.run([engine], input=cmd.encode(), capture_output=True)
    res = res.stdout.decode().strip().split("\n")
    move = res[0]
    return move


def compare_them(engine_w: str, engine_b: str) -> str:
    # NOTE: Always starts white.
    fen = STARTING_FEN
    no = 1
    while True:
        mw = get_move(engine_w, fen)
        print(f"{no}. {mw} ", end="")
        fen = feed(engine_w, fen, mw[1:])
        mb = get_move(engine_b, fen)
        fen = feed(engine_b, fen, mb[1:])

        print(f"{mb}", flush=True)
        no += 1


def main() -> None:
    if len(sys.argv) != 3:
        print_help()
        sys.exit(1)

    engine1 = sys.argv[1]
    engine2 = sys.argv[2]

    if not os.path.isfile(engine1):
        print(f"engine 1 `{engine1}` does not exist")
    if not os.path.isfile(engine2):
        print(f"engine 2 `{engine2}` does not exist")

    try:
        compare_them(engine1, engine2)
    except WhiteWinner:
        print("engine 1 won")
    except BlackWinner:
        print("engine 2 won")
    except Draw:
        print("engines drew")


if __name__ == "__main__":
    main()
