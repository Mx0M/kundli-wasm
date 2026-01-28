from pathlib import Path

RAW = Path("./data/raw")
OUT = Path("../src/ephemeris/moon")

OUT.mkdir(parents=True, exist_ok=True)

# -----------------------------
# TERM FILTERING LOGIC
# -----------------------------


def keep_term(file_index, a0, a1):
    max_effect = abs(a0) + abs(a1)

    # MAIN longitude (ELP1–ELP3)
    if file_index <= 3:
        return max_effect >= 2.0

    # SECULAR / Earth-figure (ELP4–ELP9)
    if 4 <= file_index <= 9:
        return max_effect >= 15.0

    # PLANETARY perturbations (ELP10–ELP36)
    return max_effect >= 20.0


# -----------------------------
# PARSE ONLY — NO FILTERING
# -----------------------------

def parse_file(path):
    terms = []

    for line in path.read_text().splitlines():
        line = line.strip()

        # skip headers / blank lines
        if not line:
            continue
        if line[0].isalpha():  # e.g. "MAIN PROBLEM..."
            continue

        parts = line.split()
        if len(parts) < 6:
            continue

        try:
            d = int(parts[0])
            m = int(parts[1])
            mp = int(parts[2])
            f = int(parts[3])
            a0 = float(parts[4])
            a1 = float(parts[5])
        except ValueError:
            continue

        terms.append((d, m, mp, f, a0, a1))

    return terms


# -----------------------------
# WRITE RUST OUTPUT
# -----------------------------

def write_rust(name, terms):
    path = OUT / f"elp_terms_{name}.rs"
    with path.open("w") as f:
        f.write("// AUTO-GENERATED — DO NOT EDIT\n\n")
        f.write("use super::elp_types::ElpTerm;\n\n")
        f.write(f"pub static {name.upper()}_TERMS: &[ElpTerm] = &[\n")

        for d, m, mp, f_, a0, a1 in terms:
            f.write(
                f"    ElpTerm {{ d: {d}, m: {m}, mp: {mp}, f: {f_}, a0: {a0}, a1: {a1} }},\n"
            )

        f.write("];\n")


# -----------------------------
# MAIN DRIVER
# -----------------------------

main_terms = []
sec_terms = []
plan_terms = []

for i in range(1, 37):
    fname = f"ELP{i}.txt"
    path = RAW / fname

    if not path.exists():
        print(f"⚠️ missing {fname}")
        continue

    raw_terms = parse_file(path)

    for d, m, mp, f_, a0, a1 in raw_terms:
        # MAIN longitude
        if i <= 3:
            if abs(a0) + abs(a1) >= 2.0:
                main_terms.append((d, m, mp, f_, a0, a1))

        # SKIP ELP4–ELP9 ENTIRELY
        elif 4 <= i <= 9:
            continue

        # PLANETARY perturbations
        else:
            if abs(a0) + abs(a1) >= 20.0:
                plan_terms.append((d, m, mp, f_, a0, a1))


write_rust("main", main_terms)
write_rust("sec", sec_terms)
write_rust("plan", plan_terms)

print("ELP code generation complete:")
print(f"  MAIN terms      : {len(main_terms)}")
print(f"  SECULAR terms   : {len(sec_terms)}")
print(f"  PLANETARY terms : {len(plan_terms)}")
