#!/usr/bin/env python3
"""各 .rs の先頭に AtCoder の問題URLを付ける。既に付いていれば何もしない。

対応するパス:
  ABC477/A.rs          -> abc477_a
  ABC478/b/B-better.rs -> abc478_b
  AHC071/main.rs       -> ahc071_a

引数でファイルを渡すとそれだけを処理し、無ければリポジトリ内の全 .rs を処理する。
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
CONTEST = re.compile(r"^(ABC|ARC|AGC|AHC)(\d{3})$", re.I)
LETTER = re.compile(r"^([a-z])(?![a-z])", re.I)  # "B-better" は可、"main" は不可

changed = []
targets = [pathlib.Path(a).resolve() for a in sys.argv[1:]] or sorted(ROOT.rglob("*.rs"))
for f in targets:
    parts = f.relative_to(ROOT).parts
    m = CONTEST.match(parts[0])
    if not m:
        continue
    contest = parts[0].lower()
    if m.group(1).upper() == "AHC":
        letter = "a"
    else:
        cand = parts[1] if len(parts) > 2 else f.stem
        lm = LETTER.match(cand)
        if not lm:
            print(f"skip (問題記号を判定できない): {f.relative_to(ROOT)}")
            continue
        letter = lm.group(1).lower()

    text = f.read_text(encoding="utf-8")
    if "atcoder.jp/contests/" in text.split("\n", 1)[0]:
        continue
    url = f"https://atcoder.jp/contests/{contest}/tasks/{contest}_{letter}"
    f.write_text(f"// {url}\n{text}", encoding="utf-8")
    changed.append(f"{f.relative_to(ROOT)} -> {url}")

print("\n".join(changed) or "変更なし")
