# Kundli-WASM: A High-Performance, Privacy-Preserving Vedic Astrology Computation Engine Built on Rust and WebAssembly


---

## Abstract

Kundli-WASM is an open-source, client-side Vedic astrology computation engine implemented in Rust and compiled to WebAssembly (WASM). The system performs high-precision astronomical and astrological calculations entirely within the user's browser or Node.js runtime, without transmitting any personal data to a remote server. The engine supports planetary position computation using the VSOP87 ephemeris model for the Sun through Saturn, high-precision lunar positions derived from the ELP2000 series, sidereal coordinate transformation via the Lahiri ayanāṁśa, divisional chart generation from D1 through D30, and the complete three-tier Vimshottari dasha system including Mahadasha, Antardasha, and Pratyantardasha.

The architectural decision to implement the core engine in Rust, a systems programming language with guaranteed memory safety and predictable performance characteristics, and to deliver it as a WASM binary provides several concrete advantages over traditional server-based or JavaScript-native approaches: near-native execution speed within the browser sandbox, strong memory safety guarantees without a garbage collector, full portability across all WASM-capable runtimes, and complete elimination of user data transmission. This paper describes the system's design rationale, architectural components, implementation pipeline, performance characteristics, privacy model, known limitations, and planned future work.

---

## 1. Introduction

Vedic astrology, known in Sanskrit as *Jyotiṣa* (ज्योतिष), is one of the oldest continuously practiced systems of astronomical observation and astrological interpretation in the world. Its computational requirements are substantial: constructing a natal horoscope, or *kundli*, requires calculating planetary longitudes for a precise moment in time, applying a sidereal correction to convert tropical coordinates to sidereal ones, computing house cusps and the ascendant (*lagna*), generating up to thirty divisional charts, and projecting multi-decade planetary period systems. These computations involve floating-point arithmetic over well-established astronomical models and must be performed with sufficient precision to produce results consistent with established astrological tradition.

Historically, these calculations were performed either by specialist astrologers using mathematical tables or, since the proliferation of personal computers, by dedicated desktop software. The widespread adoption of the web browser as the dominant computing platform created a new class of web-based kundli services; however, the dominant architectural pattern for such services is to accept user birth data via an HTML form, transmit it to a remote server, perform calculations server-side, and return a rendered result. This architecture introduces both a privacy concern—a user's birth date, time, and place of birth are personally identifying and potentially sensitive—and an infrastructure dependency.

Kundli-WASM addresses both concerns by relocating the entire computation stack to the client. The engine is authored in Rust, compiled once to a WASM binary, and loaded directly into the browser. All calculations execute locally; no network request carrying user data is made at any point during kundli generation. The project demonstrates that a computationally intensive domain such as astronomical ephemeris evaluation and astrological chart construction can be delivered as a self-contained, high-performance, privacy-respecting client-side application.

---

## 2. Problem Statement

Existing web-based kundli applications exhibit several structural deficiencies that Kundli-WASM is designed to address:

**Server Dependency and Availability.** Traditional web services depend on a continuously available backend. Any server outage, maintenance window, or service discontinuation renders the application non-functional, even for computations that are mathematically self-contained.

**Privacy Exposure.** Birth date, birth time, and place of birth collectively constitute personally identifying information. Transmitting this data to a third-party server—even one operated in good faith—exposes users to data retention, logging, and potential secondary use without their direct control. This concern is compounded in jurisdictions with limited data protection regulation.

**JavaScript Precision and Performance Limitations.** JavaScript, the native language of the web browser, uses IEEE 754 double-precision floating-point arithmetic and is executed by a just-in-time (JIT) compiler. While modern JIT compilers are sophisticated, they are inherently non-deterministic in performance and subject to garbage collection pauses. Astronomical calculations involving many sequential floating-point operations can accumulate rounding errors, and performance is not guaranteed across different browsers or hardware configurations.

**Opacity of Calculation.** Many commercial and semi-commercial kundli services provide no documentation of the specific algorithms, ephemeris models, or ayanāṁśa values they employ. Users have no means of auditing or verifying the calculations they receive.

**Lack of Portability.** Server-side or desktop-native applications are not portable to new runtimes without recompilation or reimplementation. A client-side WASM binary, by contrast, runs on any conforming WASM host, including browsers, Node.js, Deno, Bun, and embedded WASM runtimes.

Kundli-WASM addresses all five concerns through its core architectural choice: a Rust computation engine compiled to WebAssembly and executed exclusively on the client.

---

## 3. Background

### 3.1 Vedic Astrology: Computational Requirements

Vedic astrology (*Jyotiṣa*) differs from Western tropical astrology in several technically significant respects. The most consequential difference is the use of a sidereal zodiac rather than a tropical one. Whereas tropical astrology defines zodiac signs relative to the position of the vernal equinox, Vedic astrology defines them relative to a fixed point among the stars, requiring application of a correction factor called the *ayanāṁśa* (अयनांश) to convert computationally obtained tropical positions to sidereal ones. The Lahiri ayanāṁśa, the standard adopted by the Government of India's Calendar Reform Committee in 1955, is the most widely used convention and is employed by this system.

The core computational pipeline of a kundli engine involves the following sequential operations:

1. **Julian Day Number computation:** Converting a civil calendar date and time, adjusted to Universal Time, to a continuous count of days since the J2000.0 epoch (January 1.5, 2000 TT). This provides the scalar time input required by all subsequent astronomical calculations.

2. **Planetary longitude computation:** Computing the ecliptic longitude of each of the nine *grahas* (planets) used in Jyotiṣa—Sun, Moon, Mars, Mercury, Jupiter, Venus, Saturn, Rahu (mean north lunar node), and Ketu (mean south lunar node)—for the computed Julian Day.

3. **Sidereal transformation:** Subtracting the ayanāṁśa from each tropical longitude to obtain the corresponding sidereal longitude.

4. **Lagna and house computation:** Computing the ecliptic longitude of the eastern horizon (the ascendant or *lagna*) for the observer's geographic latitude and longitude at the moment of birth, and deriving the twelve house cusps from it.

5. **Divisional chart generation:** For each divisional chart D*n* (where *n* ∈ {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 16, 20, 24, 27, 30}), computing each planet's position within a sub-division of each zodiac sign and assigning the resulting sign position to the divisional chart.

6. **Nakshatra computation:** Determining each planet's lunar mansion (*nakshatra*) and quarter (*pada*) from its sidereal longitude.

7. **Vimshottari dasha projection:** Computing the sequence and timing of planetary periods from the Moon's nakshatra position at birth, generating Mahadasha, Antardasha, and Pratyantardasha intervals extending over the traditional 120-year cycle.

Each of these steps involves floating-point arithmetic that benefits from predictable precision and execution speed. The cumulative computation is well within the capabilities of modern consumer hardware but is sensitive to algorithmic quality, particularly in the ephemeris models used for steps 2 and 4.

### 3.2 Ephemeris Models Employed

Kundli-WASM uses two established analytical ephemeris models:

**VSOP87 (Variations Séculaires des Orbites Planétaires, 1987):** Developed by Pierre Bretagnon and Gérard Francou at the Bureau des Longitudes, VSOP87 expresses each planet's heliocentric spherical coordinates as a truncated series of periodic terms. The truncated series used in practical applications typically achieves accuracy on the order of arc-seconds over a span of several centuries around J2000.0, which is well within the requirements of astrological computation.

**ELP2000 (Éphéméride Lunaire Parisienne, 2000):** Developed by Michelle Chapront-Touzé and Jean Chapront, ELP2000 is an analytical theory of lunar motion expressed as a series expansion. The system supports both *true Moon* (the geometrically computed instantaneous position) and *mean Moon* (a smoothed position that omits short-period perturbations), with the distinction documented explicitly in the codebase.

Both of these are well-characterized, peer-reviewed models with established accuracy envelopes, providing a transparent and auditable basis for the engine's calculations.

### 3.3 WebAssembly: Architecture and Relevance

WebAssembly (WASM) is a binary instruction format designed as a portable compilation target for high-level programming languages. It was standardized by the World Wide Web Consortium (W3C) in 2019 and is natively supported by all major web browsers, as well as by non-browser runtimes such as Node.js, Deno, Bun, and Wasmtime.

WASM is relevant to Kundli-WASM for the following specific reasons:

**Near-native execution speed.** WASM is a stack-based virtual machine with a compact binary format that can be compiled to native machine code by the host runtime's ahead-of-time or baseline compiler. For computationally intensive workloads such as iterative floating-point series evaluation (as required by VSOP87 and ELP2000), WASM execution is substantially faster than interpreted JavaScript and comparable to natively compiled code.

**Memory safety within a sandbox.** WASM modules execute within a strictly isolated linear memory space. A WASM module cannot access host memory outside its own sandbox, cannot make system calls directly, and cannot perform arbitrary I/O. This sandboxing model is architecturally enforced by the WASM specification and does not depend on runtime configuration.

**Portability.** A WASM binary compiled on any host platform runs without modification on any conforming WASM runtime, regardless of the underlying processor architecture or operating system. This allows a single build artifact to serve browser, Node.js, and future embedded deployments.

**Language independence for the consumer.** JavaScript developers consuming the WASM module interact with it through a normal JavaScript API surface generated by `wasm-bindgen`. They are not required to understand Rust, WASM bytecode, or the underlying astronomical algorithms.

### 3.4 Rust as a Systems Implementation Language

Rust is a statically typed, compiled systems programming language designed to provide memory safety without a garbage collector, through a compile-time ownership and borrowing model. For a computation engine, Rust's relevant properties are:

- **No garbage collector:** Memory is deterministically allocated and freed according to ownership rules enforced at compile time. There are no garbage collection pauses that could introduce latency spikes during computation.
- **Predictable floating-point behavior:** Rust exposes IEEE 754 double-precision arithmetic directly, with no implicit coercions or runtime surprises.
- **Strong type system:** Type mismatches, integer overflows in debug mode, and unsafe memory operations are caught at compile time or flagged explicitly, reducing the risk of silent numerical errors.
- **`wasm-bindgen` integration:** The `wasm-bindgen` toolchain generates the JavaScript/TypeScript glue code necessary for browser-facing APIs, allowing Rust functions to be called from JavaScript with idiomatic syntax.

---

## 4. System Architecture

### 4.1 Architectural Overview

Kundli-WASM follows a layered architecture with a clear separation between the astronomical computation layer, the astrological interpretation layer, and the presentation layer.

```
┌───────────────────────────────────────────────────────────┐
│                     Web Browser / Node.js                 │
│                                                           │
│  ┌─────────────────────────────────────────────────────┐  │
│  │                  Presentation Layer                 │  │
│  │        HTML / CSS / JavaScript (UI)                 │  │
│  │   Dasha tree, divisional chart selector, display    │  │
│  └──────────────────────┬──────────────────────────────┘  │
│                         │ JS/WASM API boundary             │
│  ┌──────────────────────▼──────────────────────────────┐  │
│  │             wasm-bindgen Glue Layer                 │  │
│  │   Type marshalling · Error bridging · JS API        │  │
│  └──────────────────────┬──────────────────────────────┘  │
│                         │ WASM linear memory               │
│  ┌──────────────────────▼──────────────────────────────┐  │
│  │          Astrological Computation Layer (Rust)       │  │
│  │   Lagna · Divisional charts · Nakshatra · Dasha      │  │
│  └──────────────────────┬──────────────────────────────┘  │
│                         │                                  │
│  ┌──────────────────────▼──────────────────────────────┐  │
│  │          Astronomical Engine Layer (Rust)            │  │
│  │   Julian Day · VSOP87 · ELP2000 · Ayanāṁśa          │  │
│  └─────────────────────────────────────────────────────┘  │
│                                                           │
│  ← All computation occurs here; no outbound data requests │
└───────────────────────────────────────────────────────────┘
```

*Figure 1: Layered architecture of Kundli-WASM. All four layers execute within the client's WASM sandbox. No user data crosses the network boundary.*

### 4.2 Computation Flow

A single kundli generation request follows this deterministic sequence:

```
User Input
  │  (birth date, time, latitude, longitude, timezone offset)
  ▼
Julian Day Conversion
  │  civil datetime → UTC → Julian Day Number (JD)
  ▼
Planetary Position Computation
  │  VSOP87 series evaluation (Sun–Saturn)
  │  ELP2000 series evaluation (Moon)
  │  Mean node calculation (Rahu/Ketu)
  ▼
Sidereal Transformation
  │  tropical longitude − Lahiri ayanāṁśa(JD) → sidereal longitude
  ▼
Lagna and House Computation
  │  observer latitude + sidereal time → ascendant longitude
  │  → twelve house cusps
  ▼
Nakshatra and Pada Assignment
  │  sidereal longitude → nakshatra index (1–27) + pada (1–4)
  ▼
Divisional Chart Generation
  │  for each D-n: apply divisional formula to each planet's longitude
  │  → sign placement in each divisional chart
  ▼
Vimshottari Dasha Projection
  │  Moon nakshatra + birth JD → Mahadasha sequence
  │  → Antardasha subdivision
  │  → Pratyantardasha subdivision
  ▼
Structured Output
  (returned to JavaScript layer as serialized data)
```

*Figure 2: End-to-end computation flow for a single kundli generation request.*

### 4.3 Module Structure

The Rust source is organized into the following principal modules:

| Module | Responsibility |
|--------|----------------|
| `astronomy::julian` | Julian Day Number computation, epoch conversion |
| `astronomy::vsop87` | VSOP87 series coefficients and evaluation for Sun–Saturn |
| `astronomy::elp2000` | ELP2000 series coefficients and evaluation for the Moon |
| `astronomy::ayanamsa` | Lahiri ayanāṁśa computation by Julian Day |
| `astrology::planets` | Sidereal longitude computation, graha enumeration |
| `astrology::lagna` | Ascendant and house cusp computation |
| `astrology::nakshatra` | Nakshatra and pada assignment |
| `astrology::divisional` | D1–D30 divisional chart generation |
| `astrology::dasha` | Vimshottari dasha system (Maha, Antar, Pratyantara) |
| `bindings` | `wasm-bindgen` public API surface exposed to JavaScript |

The separation between the `astronomy` and `astrology` module trees reflects the design principle of distinguishing mathematically objective astronomical computation from astrologically interpreted outputs. This separation facilitates independent testing of the astronomical layer against reference ephemerides.

---

## 5. Implementation Details

### 5.1 Build Pipeline

The build pipeline from Rust source to browser-deployable WASM follows these stages:

```
Rust source (.rs)
      │
      ▼ rustc (targeting wasm32-unknown-unknown)
WASM bytecode (.wasm)
      │
      ▼ wasm-bindgen
JavaScript glue code (.js) + TypeScript definitions (.d.ts)
      │
      ▼ optional: wasm-opt (Binaryen optimizer)
Optimized WASM binary
      │
      ▼ (served as static asset)
Browser / Node.js
```

The primary build command is:

```bash
wasm-pack build --target web
```

This invokes `wasm-pack`, which in turn calls `cargo` with the `wasm32-unknown-unknown` target, runs `wasm-bindgen` to generate the JavaScript interface, and places the output artifacts in the `pkg/` directory. The resulting `pkg/` directory contains everything required for browser integration: the `.wasm` binary, the `.js` loader, and the `.d.ts` type definitions.

### 5.2 Julian Day Computation

The Julian Day Number (JDN) is the continuous count of days and fractions of days since the reference epoch used throughout the system. The conversion from civil calendar date and time to Julian Day follows the standard algorithm accounting for the Gregorian calendar reform, adjusted to Universal Time (UTC) by applying the user-provided timezone offset.

The J2000.0 epoch (Julian Day 2,451,545.0, corresponding to January 1.5, 2000 TT) is the reference epoch for the VSOP87 and ELP2000 series, so the elapsed time in Julian centuries from J2000.0 (T = (JD − 2451545.0) / 36525.0) is the fundamental scalar variable passed to all planetary evaluation functions.

### 5.3 VSOP87 Planetary Positions

For each planet from Sun through Saturn, the VSOP87 series is evaluated as a sum of cosine terms grouped by powers of T (the Julian century). The truncated series used in this implementation balances computational cost against precision requirements for astrological purposes. The result is a heliocentric ecliptic longitude that is subsequently converted to geocentric ecliptic longitude by applying the Earth's position, and then to the ecliptic of date by applying precession.

### 5.4 ELP2000 Lunar Position

The Moon's position is computed using the ELP2000 analytical theory, which expresses the Moon's longitude, latitude, and distance as sums of periodic terms involving the Moon's mean anomaly, the Sun's mean anomaly, the Moon's argument of latitude, and the elongation of the Moon from the Sun. The system explicitly distinguishes between the *true Moon* (the full series result) and the *mean Moon* (a simplified result omitting short-period terms), with the choice documented in code and exposed as a configuration parameter. True Moon is the default, consistent with standard practice in Vedic astrology software.

### 5.5 Lahiri Ayanāṁśa

The Lahiri ayanāṁśa is computed as a polynomial function of T, the Julian centuries from J2000.0, using coefficients that reproduce the official Lahiri ayanāṁśa table values. This value is subtracted from all tropical ecliptic longitudes to yield sidereal longitudes.

### 5.6 Divisional Chart Algorithm

Each divisional chart D*n* is computed by the following general algorithm: the sidereal longitude of each planet (0°–360°) is taken modulo 30° to obtain its position within its sign (0°–30°), which is then divided into *n* equal segments of 30°/*n* each. The segment index (0-based) determines the planet's placement sign in the divisional chart, starting from the planet's radical sign and proceeding in a direction specified by the particular divisional chart's convention. The specific offset rules vary by chart (D9, the *Navāṁśa*, for example, begins counting from Aries for fire signs, Cancer for earth signs, Libra for air signs, and Capricorn for water signs) and are implemented as per classical Parashari convention.

### 5.7 Vimshottari Dasha System

The Vimshottari dasha system divides a 120-year cycle among nine planets in a fixed sequence, with each planet ruling a period of between 6 and 20 years. The starting point of the cycle is determined by the Moon's nakshatra at birth. The remaining fraction of the ruling nakshatra determines the balance of the first dasha period at birth. Each dasha is subdivided proportionally into nine Antardashas, and each Antardasha is further subdivided into nine Pratyantardashas, yielding a three-tier temporal projection. All dates are computed as calendar dates by adding the computed durations in Julian days to the birth Julian Day, making the computation deterministic and reproducible.

### 5.8 `wasm-bindgen` API Surface

The public API exposed to JavaScript is defined using `wasm-bindgen` annotations. The primary entry point accepts a structured input (birth year, month, day, hour, minute, second, latitude, longitude, UTC offset) and returns a structured output containing all planetary positions, house cusps, divisional chart placements, and the dasha sequence. Rust structs annotated with `#[wasm_bindgen]` are automatically serialized for JavaScript consumption. Error conditions are returned as JavaScript `Error` objects, preserving standard JavaScript error-handling patterns.

---

## 6. Features

### 6.1 Astronomical Computation

- **VSOP87 planetary positions** for Sun, Moon (via ELP2000), Mars, Mercury, Jupiter, Venus, and Saturn.
- **True and mean Moon** computation, with explicit handling of the distinction.
- **Sidereal coordinate transformation** using the Lahiri ayanāṁśa.
- **Rahu and Ketu** positions computed from the mean lunar node.
- **Julian Day Number** computation with full Gregorian calendar support.

### 6.2 Astrological Outputs

- **Lagna (ascendant)** computation from observer coordinates and sidereal time.
- **Twelve house cusps** using the whole-sign house system standard in Vedic astrology.
- **Planetary sidereal longitudes** with sign, degree, minute, and second precision.
- **Nakshatra and pada** for each planet and the ascendant.
- **Divisional charts D1 through D30**, covering Rāśi, Horā, Drekkāṇa, Chaturtāṁśa, Saptāṁśa, Ṣaṣṭiāṁśa, Navāṁśa, Aṣṭāṁśa, Daśāṁśa, Dvādaśāṁśa, Ṣoḍaśāṁśa, Viṃśāṁśa, Siddhāṁśa, Saptaviṃśāṁśa, and Triṃśāṁśa, among others.
- **Vimshottari dasha** with Mahadasha, Antardasha, and Pratyantardasha start and end dates.

### 6.3 Runtime Characteristics

- **Fully client-side execution:** no network requests are made during computation.
- **Deterministic output:** given identical inputs, the engine produces identical outputs on any conforming WASM runtime.
- **No persistent storage access:** the engine does not read from or write to cookies, localStorage, or any browser storage API.
- **Zero external runtime dependencies:** the WASM binary is self-contained.

### 6.4 Web Interface

- Responsive layout supporting both mobile and desktop viewports.
- Expandable dasha tree view for navigating the three-tier dasha hierarchy.
- Divisional chart selector allowing switching between D1 and D30 chart views.
- No user account requirement, no login, and no analytics tracking in the reference deployment.

---

## 7. Performance Advantages

### 7.1 Comparison with JavaScript-Native Implementations

A JavaScript implementation of the same computation pipeline would face several structural disadvantages relative to the Rust/WASM approach:

**Garbage collection pauses.** JavaScript runtimes use automatic garbage collection. During a garbage collection cycle, JavaScript execution pauses. For short computations such as kundli generation (typically completing in under 100 milliseconds), a single GC pause could double or triple the observed latency. Rust's ownership model eliminates heap allocations subject to GC for the core computation path, and the WASM binary has no GC overhead at all.

**JIT compilation variability.** JavaScript JIT compilers optimize hot code paths over repeated executions. On first execution (or after the JIT cache is invalidated), JavaScript may execute at interpreted speeds. A compiled WASM binary is compiled to native code by the browser's baseline compiler on load and does not require warm-up.

**Floating-point precision semantics.** While both Rust and JavaScript use IEEE 754 double-precision arithmetic, JavaScript's JIT compiler may apply algebraic optimizations that alter the order of floating-point operations, potentially producing slightly different results across different browser versions or hardware. Rust with `wasm32-unknown-unknown` produces deterministic floating-point results within the WASM specification's constraints.

**Code size and parsing time.** A minified JavaScript bundle containing equivalent astronomical series data and evaluation code would be substantially larger in text form than the equivalent WASM binary, and would require parsing and compilation time proportional to source size.

### 7.2 Comparison with Server-Based Architectures

A server-based kundli computation service introduces latency components that are entirely absent from the client-side architecture:

| Latency Component | Server-Based | Kundli-WASM (Client-Side) |
|-------------------|-------------|--------------------------|
| DNS resolution | Required | Not applicable |
| TLS handshake | Required | Not applicable |
| Network round-trip | Required (varies by geography) | Not applicable |
| Server queue time | Variable | Not applicable |
| Response serialization + deserialization | Required | Not applicable |
| WASM module load (first request) | Not applicable | One-time cost |
| Core computation time | Milliseconds (server CPU) | Milliseconds (client CPU) |

For users geographically distant from the server, network round-trip time alone can exceed the entire computation time of the client-side engine. After the WASM module is cached by the browser (a one-time cost on the first visit), subsequent kundli generation requests incur only the computation time, with no network overhead.

Furthermore, server-based systems are subject to load scaling concerns. A client-side architecture has effectively unlimited horizontal scale: each user's browser is its own compute node. The reference deployment's server infrastructure serves only static assets (HTML, CSS, JS, and the WASM binary), which can be efficiently cached and delivered via a content delivery network.

---

## 8. Privacy and Security Benefits

### 8.1 Zero Data Transmission

The most significant privacy property of Kundli-WASM is that user birth data—specifically birth date, birth time, and birth location—is processed exclusively within the user's own browser environment. At no point during a kundli generation request is this data serialized and transmitted to any remote host. This property is architectural rather than policy-based: the engine has no mechanism by which to transmit data, as it executes entirely within the WASM sandbox and makes no outbound network calls.

This stands in contrast to server-based kundli services, which necessarily receive and may log or retain user birth data. Even if such services are operated in good faith with appropriate data protection policies, users cannot independently verify these policies or audit their enforcement.

### 8.2 WebAssembly Sandbox Isolation

The WASM sandbox provides security properties beyond those available to JavaScript:

- A WASM module can only access memory within its own linear memory space. It cannot read or write host memory, browser state, cookies, or storage outside what the host explicitly provides through imported functions.
- A WASM module cannot make system calls directly. Any I/O, network access, or browser API usage must be mediated by the JavaScript host through explicitly imported functions. Kundli-WASM imports no functions that perform network I/O, and the absence of such imports is verifiable by inspection of the WASM module's import table.
- The WASM specification enforces type safety at the boundary between the WASM module and the JavaScript host. Invalid function calls across this boundary are rejected at module validation time, not at runtime.

### 8.3 Auditability

Because Kundli-WASM is open-source under the MIT License, users and auditors can examine the full source code of both the computation engine and the web interface. The specific ephemeris models, ayanāṁśa formula, divisional chart algorithms, and dasha calculation methods are all documented in code. Users who distrust any specific calculation can reproduce it by reading and verifying the corresponding Rust source.

The WASM binary itself can be disassembled to WebAssembly Text Format (WAT) using standard tooling (`wasm2wat` from the Binaryen toolkit) and audited at the binary level if desired.

### 8.4 No External Dependencies at Runtime

The WASM binary carries no runtime dependencies on external services, external JavaScript libraries beyond the `wasm-bindgen`-generated glue, or third-party APIs. There are no analytics scripts, no advertising tags, and no telemetry calls embedded in the reference deployment.

---

## 9. Limitations

An accurate characterization of the system requires acknowledging its current limitations:

**Ephemeris coverage period.** The VSOP87 and ELP2000 analytical series are accurate to arc-second level for dates within approximately a thousand years of J2000.0. For extreme historical dates (more than a few centuries before or after this range), accumulated series truncation error may become astronomically significant, though it typically remains within astrological tolerance.

**Outer planets (Uranus, Neptune, Pluto).** The nine *grahas* of classical Vedic astrology do not include the outer planets discovered in the modern era. Kundli-WASM does not compute positions for Uranus, Neptune, or Pluto, consistent with traditional Jyotiṣa practice but limiting applicability to practitioners who incorporate outer planets.

**Ayanāṁśa convention.** The system implements only the Lahiri ayanāṁśa. Alternative ayanāṁśa conventions (Raman, Krishnamurti, Fagan-Bradley, Yukteshwar, etc.) are not currently supported. Users whose practice relies on a different ayanāṁśa will obtain results that differ from their reference system.

**House system.** The implementation uses the whole-sign house system, the most common convention in classical Vedic astrology. Equal house, Placidus, and other house systems are not implemented.

**True node vs. mean node for Rahu/Ketu.** The current implementation uses the mean lunar node for Rahu and Ketu. Some practitioners and software tools use the true (osculating) node, which can differ by up to about 1.5° at maximum excursion.

**No rectification or progression support.** The engine computes natally for a fixed birth moment. Astrological techniques requiring computed positions for multiple time points in sequence (solar arc directions, secondary progressions) are not implemented.

**Single-threaded execution.** The WASM module executes in a single thread. For the current feature set, computation completes well within interactive response time on modern consumer hardware; however, future extensions involving exhaustive multi-chart computations could benefit from Web Workers and WASM threads (the `wasm-threads` proposal).

**No offline ephemeris fallback.** The system has no mechanism to download and cache a precomputed ephemeris table for offline use. While computation is client-side, the initial WASM binary load requires network access. Subsequent uses benefit from browser HTTP caching.

---

## 10. Future Work

Several areas of extension are technically feasible within the current architecture:

**Additional ayanāṁśa conventions.** Implementing the Krishnamurti and Raman ayanāṁśa as selectable options would expand compatibility with practitioners and software tools using those conventions.

**True lunar node.** Adding true (osculating) lunar node computation would improve compatibility with tools that use it and allow users to compare mean and true node results.

**Additional house systems.** Implementing the Sripati and equal-house systems within the same computational framework would require modest additions to the house computation module.

**Outer planet positions.** For practitioners who incorporate Uranus, Neptune, and Pluto, adding VSOP87-compatible series for these bodies would be a natural extension.

**Web Worker parallelism.** Offloading the WASM computation to a Web Worker thread would prevent any blocking of the browser's main thread during calculation, improving perceived responsiveness on low-end hardware.

**Multiple chart comparison.** Computing kundlis for multiple subjects and generating compatibility or synastry analyses would require extending the API surface and potentially parallelizing across multiple WASM instances.

**Node.js and Deno packaging.** Distributing the WASM module as an npm package with TypeScript declarations would enable integration into server-side or toolchain contexts for developers building on top of the engine.

**Chart export.** Generating SVG or PDF representations of the computed charts within the WASM layer, eliminating dependence on JavaScript-side rendering, would improve portability and print quality.

**Extended ephemeris range.** For historical and predictive applications requiring accuracy over a wider time range, replacing the truncated series with higher-order coefficients or a hybrid approach using precomputed tables at the boundaries would extend reliable coverage.

---

## 11. Conclusion

Kundli-WASM demonstrates that a computationally intensive domain—astronomical ephemeris evaluation and Vedic astrological chart construction—can be implemented as a high-quality, privacy-preserving, fully client-side web application using Rust compiled to WebAssembly. The architectural choice to eliminate server-side computation is not merely a deployment convenience: it produces concrete, verifiable benefits in terms of user data privacy, availability independence, latency reduction, and horizontal scalability.

The use of Rust as the implementation language provides memory safety guarantees, predictable performance, and compatibility with the `wasm-bindgen` toolchain, enabling the construction of a clean JavaScript API surface over a systems-level computation engine. The reliance on established, peer-reviewed ephemeris models (VSOP87 and ELP2000) and the standard Lahiri ayanāṁśa provides a transparent and auditable computational basis that distinguishes the system from opaque commercial alternatives.

The engine's open-source availability under the MIT License, combined with the verifiability of the WASM binary itself, allows any interested user or researcher to audit the calculations and confirm their conformance with the documented algorithms. This transparency is itself a meaningful property in a domain where computational correctness has practical significance for users who rely on astrological outputs.

The current implementation is a functional foundation for a broader ecosystem of Vedic astrology tooling delivered as client-side WASM. The limitations identified in Section 9 represent well-characterized boundaries rather than design defects, and the future work described in Section 10 offers concrete paths toward addressing each of them. Kundli-WASM establishes that the technical quality bar for client-side astronomical computation is achievable with modern tooling, and provides an open, extensible platform for further development.

---

## References

1. Bretagnon, P., & Francou, G. (1988). Planetary theories in rectangular and spherical variables: VSOP87 solutions. *Astronomy and Astrophysics*, 202, 309–315.

2. Chapront-Touzé, M., & Chapront, J. (1983). The lunar ephemeris ELP2000. *Astronomy and Astrophysics*, 124(1), 50–62.

3. Chapront-Touzé, M., & Chapront, J. (1988). ELP2000-85: A semi-analytical lunar ephemeris adequate for historical times. *Astronomy and Astrophysics*, 190(1–2), 342–352.

4. Government of India, Calendar Reform Committee. (1955). *Report of the Calendar Reform Committee*. Council of Scientific and Industrial Research, New Delhi.

5. WebAssembly Community Group. (2019). *WebAssembly Specification, Release 1.0*. World Wide Web Consortium. https://webassembly.github.io/spec/core/

6. WebAssembly Community Group. (2022). *WebAssembly Specification, Release 2.0*. World Wide Web Consortium. https://webassembly.github.io/spec/core/

7. Klabnik, S., & Nichols, C. (2022). *The Rust Programming Language* (2nd ed.). No Starch Press.

8. `wasm-bindgen` Contributors. (2024). *wasm-bindgen: Facilitating high-level interactions between Wasm modules and JavaScript*. https://github.com/rustwasm/wasm-bindgen

9. `wasm-pack` Contributors. (2024). *wasm-pack: Your one-stop shop for building and working with Rust-generated WebAssembly*. https://github.com/rustwasm/wasm-pack

10. Binaryen Contributors. (2024). *Binaryen: Compiler infrastructure and toolchain library for WebAssembly*. https://github.com/WebAssembly/binaryen

11. Parasara, M. (Trans. Santhanam, R.). (1984). *Brihat Parasara Hora Sastra* (Vols. 1–2). Ranjan Publications, New Delhi. (Original work composed approximately 6th–7th century CE.)

12. Mx0M. (2025). *kundli-wasm: High-precision Vedic astrology engine written in Rust + WebAssembly*. GitHub. https://github.com/Mx0M/kundli-wasm

13. Meeus, J. (1998). *Astronomical Algorithms* (2nd ed.). Willmann-Bell, Inc.

14. Seidelmann, P. K. (Ed.). (1992). *Explanatory Supplement to the Astronomical Almanac*. University Science Books.

---

*This whitepaper describes the Kundli-WASM project as of its publicly available source at the time of writing. The project is released under the MIT License. Contributions, issues, and discussion may be directed to the project repository at https://github.com/Mx0M/kundli-wasm.*
