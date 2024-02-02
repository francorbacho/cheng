#!/usr/bin/python

import subprocess
import sys
import os

STARTING_FEN = "4r3/p7/4nk2/3p1Bp1/3P2P1/5PK1/8/1R6 w - - 4 30"


class Engine:
    def __init__(self, path: str) -> None:
        if not os.path.isfile(path):
            raise Exception(f"engine `{path}` does not exist")

        self.verbose = True
        self.process = subprocess.Popen(
            path,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            universal_newlines=True,
            bufsize=1,
            shell=False,
        )

        self.uci_send("uci")
        self.uci_send("isready")
        self.uci_waitfor("uciok")
        self.uci_waitfor("readyok")

    def uci_send(self, cmd: str) -> None:
        if self.verbose:
            print(f"<< {cmd}", file=sys.stderr)
        self.process.stdin.write(f"{cmd}\n")
        self.process.stdin.flush()

    def uci_recv(self) -> str:
        resp = self.process.stdout.readline().strip()
        if self.verbose:
            print(f">> {resp}", file=sys.stderr)
        return resp

    def uci_waitfor(self, response: str) -> None:
        if self.uci_recv() != response:
            raise ValueError()

    def go(self, fen: str, moves: list[str]) -> str:
        moves_joined = " ".join(moves)
        self.uci_send(f"position fen {fen} moves {moves_joined}")
        self.uci_send(f"go movetime 0")
        while True:
            line = self.uci_recv()
            [prefix, bestmove] = line.split(" ")
            assert prefix == "bestmove"
            return bestmove

    def __exit__(self) -> None:
        self.process.terminate()


class WhiteWinner(Exception):
    pass


class BlackWinner(Exception):
    pass


class Draw(Exception):
    pass


def print_help() -> None:
    print(f"usage: {sys.argv[0]} <engine1> <engine2>")


def compare_them(engine_w: Engine, engine_b: Engine) -> str:
    # NOTE: Always starts white.
    fen = STARTING_FEN
    moves = []
    while True:
        mw = engine_w.go(fen, moves)
        if mw == "(none)":
            raise BlackWinner()
        moves.append(mw)
        print(f"{len(moves)}. {mw} ", end="")

        mb = engine_b.go(fen, moves)
        if mb == "(none)":
            raise WhiteWinner()
        moves.append(mb)
        print(f"{mb}", flush=True)


def main() -> None:
    if len(sys.argv) != 3:
        print_help()
        sys.exit(1)

    engine1 = Engine(sys.argv[1])
    engine2 = Engine(sys.argv[2])

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
