# Vedic Kundli Engine (Rust + WASM)

A high-precision **Vedic astrology (Jyotish) computation engine** written in **Rust** and compiled to **WebAssembly**, designed to run fully client-side in the browser.

The project focuses on **astronomical correctness, clean architecture**, and **extensibility**, providing a solid foundation for kundli generation, divisional charts, and dasha systems.

**Site using:** [Astrolyk Kundli](https://www.astrolyk.com/kundli/)  

---

## ✨ Features

### 🌍 Astronomy & Ephemeris
- VSOP87 planetary positions (Sun–Saturn)
- High-precision Moon (ELP2000 series)
- True and mean Moon support
- Sidereal positions (Lahiri ayanāṁśa)
- Client-side, deterministic calculations

### 🪐 Astrology
- Lagna & house calculation
- Planetary positions (tropical + sidereal)
- Divisional charts **D1 to D30**
- Nakshatra & pada calculation
- Vimshottari dasha system:
  - Mahadasha
  - Antardasha
  - Pratyantardasha

### 🌐 Web UI
- Fully client-side WebAssembly app
- Responsive UI (mobile + desktop)
- Expandable dasha tree view
- Divisional chart selector
- No backend, no tracking

---

## 🧠 Design Philosophy

- **Astronomy first**: calculations are based on established ephemeris models
- **Separation of concerns**:
  - Astronomy ≠ Astrology logic
  - True vs Mean Moon handled explicitly
- **No black-box magic**: all algorithms are transparent and documented
- **Extensible architecture** for future additions

---

## ⚠️ Accuracy Notes

This project aims for **high astronomical accuracy** and consistency.

Some traditional astrology software (e.g. commercial tools) may differ slightly due to:
- Mean vs true Moon usage
- Historical ayanāṁśa conventions
- Boundary handling choices

Such differences are expected and explicitly documented in the code.

---

## 🧩 Tech Stack

- **Rust** — core computation engine
- **WebAssembly (wasm-bindgen)** — browser execution
- **JavaScript** — UI integration
- **HTML / CSS** — responsive frontend

---

## 🚀 Getting Started

### Build WASM
```bash
wasm-pack build --target web
