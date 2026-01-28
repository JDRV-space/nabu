# Nabu - Speed Reader

Mesopotamian god of writing, scribes, literacy, and wisdom.

## Tech Stack
- **Framework:** Leptos (Rust WASM)
- **Storage:** IndexedDB with AES-GCM encryption
- **Deploy:** Vercel (auto-deploy from GitHub)
- **UI Style:** Dark theme, amber accents, Space Grotesk

---

## Design Rules
- **No goofy emojis.** Serious only if used (e.g., fire for streak)
- **ORP highlight is AMBER (#ffaa00), not red** - red conflicts with error states
- **Immersive focus** - controls hidden during reading, appear on pause
- **Glassmorphism** - semi-transparent panels with blur
- **Text scramble** - Signature animation on word transitions

---

## CRITICAL CONSTRUCTION RULES

**1. CLEAN REPO**
- Extremely clean, organized, easy to understand
- Clear folder structure, no clutter
- Meaningful file/function names
- Comments only where logic is non-obvious

**2. NO OVERENGINEERING**
- Solve the problem, nothing more
- No premature abstractions
- No "future-proofing" for hypotheticals
- Simple > clever

**3. TEST BEFORE COMMIT**
- After each component is finished: TEST
- If test succeeds: commit and add
- If test fails: fix before moving on
- No broken code in main branch

---

## Build Phases

### PHASE 1: Foundation (Sequential)
```
[1] Leptos project setup
[2] GitHub repo (JDRV-space/nabu)
[3] Vercel + CI/CD integration
```

### PHASE 2: Core (Parallel)
```
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│  A: UI Theme    │  │  B: File Parse  │  │  C: Storage     │
│  - Dark/light   │  │  - PDF/TXT/DOCX │  │  - IndexedDB    │
│  - Colors       │  │  - MD parsing   │  │  - Encryption   │
│  - Typography   │  │  - Validation   │  │  - Resume pos   │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

### PHASE 3: Reader (Sequential, depends on Phase 2)
```
[1] RSVP word display + amber ORP
[2] WPM slider control
[3] Progress bar + time remaining
[4] Keyboard shortcuts
[5] Touch gestures
[6] Fullscreen mode
[7] Word size adjustment
```

### PHASE 4: Features (Parallel, depends on Phase 3)
```
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│  A: Library UI  │  │  B: Settings    │  │  C: Enhancements│
│  - Grid/list    │  │  - Panel UI     │  │  - Chunk mode   │
│  - Empty state  │  │  - Preferences  │  │  - Bionic read  │
│  - Doc cards    │  │  - Save/load    │  │  - Speed ramp   │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

### PHASE 5: Polish (Sequential)
```
[1] Text scramble animation
[2] Stats dashboard
[3] All animations (hover, transitions)
```

### PHASE 6: Security (Sequential, final)
```
[1] XSS sanitization (ammonia)
[2] File validation (magic bytes)
[3] CSP headers + SRI
[4] cargo audit
```

---

## Checklist

### Setup
- [x] Set up Leptos project with WASM target

### UI/UX
- [x] Dark theme (dark bg #070910, amber accents #ffaa00, Space Grotesk font)
- [ ] Dark/light theme toggle
- [x] Progress bar + time remaining display
- [x] Saved documents library UI
- [x] Fullscreen mode toggle
- [x] Word size adjustment (S / M / L / XL)

### Core Reader
- [x] File upload component (TXT, MD parsing - PDF/DOCX coming soon)
- [x] RSVP word display with amber ORP highlighting
- [x] WPM slider control (100-1000 range)
- [x] Punctuation pause logic (longer delay on . , ;)
- [ ] Chunk mode (2-3 words display option)
- [ ] Bionic reading toggle (bold first letters)
- [ ] Text scramble animation on word transitions

### Controls
- [x] Keyboard shortcuts (space=pause, arrows=speed, F=fullscreen, ESC=exit, R=restart)
- [ ] Touch gestures (swipe speed, tap pause, long-press controls)
- [ ] Speed ramping (gradual WPM increase)

### Storage
- [x] IndexedDB with encryption (aes-gcm crate)
- [ ] Resume position per document
- [ ] Reading stats dashboard

### Security
- [x] XSS sanitization (ammonia crate)
- [x] File validation (magic bytes, size limits)
- [x] CSP headers + SRI for WASM
- [x] cargo audit - only unmaintained warnings, no vulnerabilities

### DevOps
- [x] GitHub repo (JDRV-space/nabu)
- [x] Vercel project with GitHub integration
- [x] CI/CD: push to main auto-deploys to Vercel
- [x] **LIVE:** https://nabu-reader.vercel.app

---

## Color System

```
PRIMARY PALETTE
--bg: #070910              Void black (backgrounds)
--bg-elevated: #0d1117     Cards, modals, panels
--amber: #ffaa00           ORP, accents, CTAs, progress
--amber-glow: rgba(255, 170, 0, 0.15)     Hover states, glows
--text: #f5f5f5            Primary text
--text-muted: rgba(245, 245, 245, 0.6)    Secondary text
--border: rgba(255, 255, 255, 0.08)       Glassmorphism borders
--success: #4ade80         Completed, streaks

TYPOGRAPHY
Font Family: 'Space Grotesk', system-ui, sans-serif
Word Display: 6rem/96px (desktop), 3rem/48px (mobile), font-weight: 500
Headings: 1.5rem/24px, font-weight: 600
Body: 1rem/16px, font-weight: 400
```

---

## UI Mockups

### Reader View (Active - Immersive)
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║    Background: #070910 (void black)                                           ║
║    Subtle noise texture at 2% opacity                                         ║
║    Faint radial vignette at edges                                             ║
║    Logo watermark top-left at 3% opacity                                      ║
║                                                                               ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │                                                                         │  ║
║  │   (Nabu logo 3% opacity)                                                │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │                              .                                          │  ║
║  │                        conte|mplate                                     │  ║
║  │                              .                                          │  ║
║  │                              _                                          │  ║
║  │                        (amber underglow)                                │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │    NO CONTROLS VISIBLE - pure immersion                                 │  ║
║  │                                                                         │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                                                                               ║
║    Typography: Space Grotesk, 6rem (desktop) / 3rem (mobile)                  ║
║    ORP character: amber (#ffaa00) vertical line + underglow                   ║
║    Other characters: off-white (#f5f5f5)                                      ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Reader View (Paused - Controls Visible)
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║    Controls fade in with glassmorphism panels                                 ║
║    background: rgba(13, 17, 23, 0.85)                                         ║
║    backdrop-filter: blur(16px)                                                ║
║    border: 1px solid rgba(255, 255, 255, 0.08)                                ║
║                                                                               ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │                                                                         │  ║
║  │  ┌────────────────────────────────────────────────────────────────────┐ │  ║
║  │  │  Menu   Library                        Settings   [  ] Fullscreen  │ │  ║
║  │  └────────────────────────────────────────────────────────────────────┘ │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │                              .                                          │  ║
║  │                        conte|mplate                                     │  ║
║  │                              .                                          │  ║
║  │                              _                                          │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │  ┌────────────────────────────────────────────────────────────────────┐ │  ║
║  │  │                                                                    │ │  ║
║  │  │        <<           ┌─────┐           >>        ┌──────────┐       │ │  ║
║  │  │       -50          │ PLAY │          +50       │  420 WPM │       │ │  ║
║  │  │                     └─────┘                     └──────────┘       │ │  ║
║  │  │                                                                    │ │  ║
║  │  │   ######################__________________   47%                   │ │  ║
║  │  │   Word 1,842 of 3,921              8:34 remaining                  │ │  ║
║  │  │                                                                    │ │  ║
║  │  │   Font: [S] [M] [L] [XL]                                           │ │  ║
║  │  │                                                                    │ │  ║
║  │  └────────────────────────────────────────────────────────────────────┘ │  ║
║  │                                                                         │  ║
║  │  ┌────────────────────────────────────────────────────────────────────┐ │  ║
║  │  │  SPACE pause   <-/-> +/-50 WPM   R restart   F fullscreen   ESC    │ │  ║
║  │  └────────────────────────────────────────────────────────────────────┘ │  ║
║  │                                                                         │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                                                                               ║
║    Progress bar: clickable to seek, expands on hover                          ║
║    Keyboard hints: fade out after 3 seconds                                   ║
║    Font size buttons: S=3rem, M=4.5rem, L=6rem, XL=8rem                       ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Word Transition Animation (Text Scramble)
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║    TEXT SCRAMBLE EFFECT (signature animation)                                 ║
║    Duration: 60ms total | ORP character stays stable                          ║
║                                                                               ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │                                                                         │  ║
║  │    Frame 0 (0ms):      conte|mplate         <- current word             │  ║
║  │                              _                                          │  ║
║  │                                                                         │  ║
║  │    Frame 1 (15ms):     c0nt3|mpl@t3         <- scramble starts          │  ║
║  │                              _                                          │  ║
║  │                                                                         │  ║
║  │    Frame 2 (30ms):     kn#w|l*dg&           <- mixing in new word       │  ║
║  │                             _                                           │  ║
║  │                                                                         │  ║
║  │    Frame 3 (45ms):     know|l+dge           <- resolving                │  ║
║  │                             _                                           │  ║
║  │                                                                         │  ║
║  │    Frame 4 (60ms):     know|ledge           <- new word complete        │  ║
║  │                             _                                           │  ║
║  │                                                                         │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                                                                               ║
║    Characters that scramble: !"#$%&'()*+,-./0123456789:;<=>?@                 ║
║    ORP position recalculates for each word (25-35% into word)                 ║
║    Easing: linear (must feel instant at high WPM)                             ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Document Library
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │                                                                         │  ║
║  │  ┌──────────────────────────────────────────────────────────────────┐   │  ║
║  │  │  LIBRARY                          [Search...]   [Grid] [List] +  │   │  ║
║  │  └──────────────────────────────────────────────────────────────────┘   │  ║
║  │                                                                         │  ║
║  │  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐      │  ║
║  │  │##################│  │##################│  │#########_________│      │  ║
║  │  │                  │  │                  │  │                  │      │  ║
║  │  │                  │  │       Done       │  │                  │      │  ║
║  │  │  Deep Work       │  │  Atomic          │  │  The Art of      │      │  ║
║  │  │  Cal Newport     │  │  Habits          │  │  War             │      │  ║
║  │  │                  │  │  James Clear     │  │  Sun Tzu         │      │  ║
║  │  │                  │  │                  │  │                  │      │  ║
║  │  │  34%    PDF      │  │  100%   TXT      │  │  52%    PDF      │      │  ║
║  │  │  2 days ago      │  │  Completed       │  │  yesterday       │      │  ║
║  │  └──────────────────┘  └──────────────────┘  └──────────────────┘      │  ║
║  │        |                                            |                   │  ║
║  │   amber border                                 amber border             │  ║
║  │   (in progress)                                (in progress)            │  ║
║  │                                                                         │  ║
║  │  Card styling:                                                          │  ║
║  │  - background: rgba(255, 255, 255, 0.03)                                │  ║
║  │  - border: 1px solid rgba(255, 255, 255, 0.06)                          │  ║
║  │  - border-radius: 12px                                                  │  ║
║  │  - In-progress: amber left border (3px solid #ffaa00)                   │  ║
║  │  - Completed: muted styling, checkmark                                  │  ║
║  │  - Hover: translateY(-4px), amber glow                                  │  ║
║  │                                                                         │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Library Empty State
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │                                                                         │  ║
║  │  ┌──────────────────────────────────────────────────────────────────┐   │  ║
║  │  │  LIBRARY                                            + Add        │   │  ║
║  │  └──────────────────────────────────────────────────────────────────┘   │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │                              +-------+                                  │  ║
║  │                              |       |                                  │  ║
║  │                              |  //\  |  <- cuneiform symbol             │  ║
║  │                              |       |    pulsing amber glow            │  ║
║  │                              +-------+    (3s cycle)                    │  ║
║  │                                                                         │  ║
║  │                         No documents yet                                │  ║
║  │                                                                         │  ║
║  │                   ┌─────────────────────────────┐                       │  ║
║  │                   │                             │                       │  ║
║  │                   │   + Upload your first       │                       │  ║
║  │                   │        document             │                       │  ║
║  │                   │                             │                       │  ║
║  │                   └─────────────────────────────┘                       │  ║
║  │                                                                         │  ║
║  │                      PDF  .  TXT  .  DOCX                               │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │  + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +   │  ║
║  │  |                                                                 |   │  ║
║  │  |               or drag and drop anywhere                         |   │  ║
║  │  |                                                                 |   │  ║
║  │  + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +   │  ║
║  │                   ^                                                     │  ║
║  │             dashed border                                               │  ║
║  │         animates on drag-over                                           │  ║
║  │                                                                         │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                                                                               ║
║    Drag-over state:                                                           ║
║    - Dashed border -> solid amber                                             ║
║    - Background gets amber-glow overlay                                       ║
║    - Scale up slightly (1.01)                                                 ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Stats Dashboard
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │                                                                         │  ║
║  │  ┌──────────────────────────────────────────────────────────────────┐   │  ║
║  │  │  STATISTICS                                    This Week v       │   │  ║
║  │  └──────────────────────────────────────────────────────────────────┘   │  ║
║  │                                                                         │  ║
║  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐│  ║
║  │  │              │  │              │  │              │  │              ││  ║
║  │  │   127,432    │  │     6.2      │  │     423      │  │      12      ││  ║
║  │  │   words      │  │    hours     │  │   avg WPM    │  │    streak    ││  ║
║  │  │   ^ 18%      │  │   ^ 12%      │  │   ^ 8%       │  │     days     ││  ║
║  │  │              │  │              │  │              │  │              ││  ║
║  │  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘│  ║
║  │         |                                                      |       │  ║
║  │    amber glow                                             gold border  │  ║
║  │    (positive)                                             (streak>=7)  │  ║
║  │                                                                         │  ║
║  │  ================================================================       │  ║
║  │                          (cuneiform divider)                            │  ║
║  │                                                                         │  ║
║  │  READING ACTIVITY                                                       │  ║
║  │  ┌──────────────────────────────────────────────────────────────────┐   │  ║
║  │  │                                                                  │   │  ║
║  │  │   3h |                    ##                                     │   │  ║
║  │  │      |              ##    ##                                     │   │  ║
║  │  │   2h |        ##    ##    ##          ##                         │   │  ║
║  │  │      |  ##    ##    ##    ##    ##    ##                         │   │  ║
║  │  │   1h |  ##    ##    ##    ##    ##    ##    ##                   │   │  ║
║  │  │      |  ##    ##    ##    ##    ##    ##    ##                   │   │  ║
║  │  │    0 +--##----##----##----##----##----##----##--                 │   │  ║
║  │  │       Mon   Tue   Wed   Thu   Fri   Sat   Sun                    │   │  ║
║  │  │                                                                  │   │  ║
║  │  └──────────────────────────────────────────────────────────────────┘   │  ║
║  │                                                                         │  ║
║  │  SPEED PROGRESSION                                                      │  ║
║  │  ┌──────────────────────────────────────────────────────────────────┐   │  ║
║  │  │                                                                  │   │  ║
║  │  │  550 |                                        o======            │   │  ║
║  │  │      |                              o---------                   │   │  ║
║  │  │  450 |                    o---------                             │   │  ║
║  │  │      |          o---------                                       │   │  ║
║  │  │  350 |o---------                                                 │   │  ║
║  │  │      +---------+---------+---------+---------+---------          │   │  ║
║  │  │        Week 1    Week 2    Week 3    Week 4    Week 5            │   │  ║
║  │  │                                                                  │   │  ║
║  │  └──────────────────────────────────────────────────────────────────┘   │  ║
║  │                                                                         │  ║
║  │    Charts: animate on scroll into view                                  │  ║
║  │    Bars grow up (600ms ease-out)                                        │  ║
║  │    Line draws left-to-right (800ms ease-out)                            │  ║
║  │    Area fill below line at 15% amber opacity                            │  ║
║  │                                                                         │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Settings Panel
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │                                                                         │  ║
║  │  SETTINGS                                                          X    │  ║
║  │                                                                         │  ║
║  │  -----------------------------------------------------------------      │  ║
║  │  READING                                                                │  ║
║  │  -----------------------------------------------------------------      │  ║
║  │                                                                         │  ║
║  │  Default Speed                                                          │  ║
║  │  100 ---------------------o=============== 1000                         │  ║
║  │                          420 WPM                                        │  ║
║  │                                                                         │  ║
║  │  Chunk Size                                                             │  ║
║  │  (o) 1 word    ( ) 2 words    ( ) 3 words                               │  ║
║  │                                                                         │  ║
║  │  Bionic Reading                                                         │  ║
║  │  ( ) Off    (o) On                                                      │  ║
║  │                                                                         │  ║
║  │     Preview: "THis is HOw biONic REAding looks"                         │  ║
║  │              (bold = darker/bolder letters)                             │  ║
║  │                                                                         │  ║
║  │  Punctuation Pause                                                      │  ║
║  │  (o) On    ( ) Off            Multiplier: [1.5x v]                      │  ║
║  │                                                                         │  ║
║  │  Speed Ramping                                                          │  ║
║  │  ( ) Off    (o) On            +10 WPM every [60s v]                     │  ║
║  │                                                                         │  ║
║  │  -----------------------------------------------------------------      │  ║
║  │  DISPLAY                                                                │  ║
║  │  -----------------------------------------------------------------      │  ║
║  │                                                                         │  ║
║  │  Theme                                                                  │  ║
║  │  (o) Dark    ( ) Light    ( ) System                                    │  ║
║  │                                                                         │  ║
║  │  Font Size                                                              │  ║
║  │  ( ) S    (o) M    ( ) L    ( ) XL                                      │  ║
║  │                                                                         │  ║
║  │  ORP Style                                                              │  ║
║  │  (o) Underline    ( ) Background    ( ) None                            │  ║
║  │                                                                         │  ║
║  │                                                                         │  ║
║  │             ┌────────────────────────┐                                  │  ║
║  │             │     Save Settings      │  <- amber border                 │  ║
║  │             │                        │     fills on hover               │  ║
║  │             └────────────────────────┘                                  │  ║
║  │                                                                         │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                                                                               ║
║    Modal: centered, max-width 500px                                           ║
║    Opens with scale(0.95) -> scale(1) + fade                                  ║
║    Backdrop: rgba(0, 0, 0, 0.8) with click-to-close                           ║
║    Radio buttons: custom amber circles                                        ║
║    (o) = filled amber    ( ) = empty with border                              ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Mobile Views
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║   READER (ACTIVE)              READER (PAUSED)             LIBRARY            ║
║  ┌─────────────────┐          ┌─────────────────┐        ┌─────────────────┐  ║
║  │                 │          │                 │        │ LIBRARY    + =  │  ║
║  │                 │          │  =         @ [] │        ├─────────────────┤  ║
║  │                 │          │                 │        │                 │  ║
║  │                 │          │                 │        │ ┌─────────────┐ │  ║
║  │                 │          │                 │        │ │####________│ │  ║
║  │       .         │          │       .         │        │ │ Deep Work  │ │  ║
║  │  kno|wledge     │          │  kno|wledge     │        │ │ 34% . PDF  │ │  ║
║  │       .         │          │       .         │        │ └─────────────┘ │  ║
║  │                 │          │                 │        │                 │  ║
║  │                 │          │    ┌───────┐    │        │ ┌─────────────┐ │  ║
║  │                 │          │    │ PLAY  │    │        │ │############│ │  ║
║  │                 │          │    └───────┘    │        │ │ Atomic     │ │  ║
║  │                 │          │                 │        │ │ 100% . TXT │ │  ║
║  │                 │          │    420 WPM      │        │ └─────────────┘ │  ║
║  │                 │          │                 │        │                 │  ║
║  │                 │          │ <<    47%    >> │        │ ┌─────────────┐ │  ║
║  │                 │          │                 │        │ │____________│ │  ║
║  │ ###### 47%      │          │ ######_________ │        │ │ Meditation │ │  ║
║  │                 │          │  5:23 remaining │        │ │ 0% . DOCX  │ │  ║
║  └─────────────────┘          └─────────────────┘        │ └─────────────┘ │  ║
║                                                          │                 │  ║
║                                                          ├─────────────────┤  ║
║   GESTURES:                                              │ Lib  Read Stats │  ║
║   . Tap center = pause/play                              └─────────────────┘  ║
║   . Swipe <-/-> = +/-50 WPM                                                   ║
║   . Swipe ^/v = +/-10 WPM                                                     ║
║   . Long press = show controls                                                ║
║   . Double tap = restart                                                      ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Animations Summary

| Element | Animation | Duration | Easing |
|---------|-----------|----------|--------|
| Word transition | Text scramble | 50-80ms | linear |
| Controls fade in | Opacity 0 -> 1 | 200ms | ease-out |
| Card hover | translateY(-4px) | 150ms | ease-out |
| Progress fill | Width transition | 100ms | linear |
| Modal open | Scale 0.95 -> 1, fade | 200ms | ease-out |
| Button press | Scale 0.95 | 100ms | ease-in-out |
| Loading shimmer | Gradient sweep | 1500ms | linear (infinite) |
| Glow pulse | Box-shadow intensity | 3000ms | ease-in-out (infinite) |
| Stat bars | Height 0 -> value | 600ms | ease-out |

---

## Security Notes

### CSP Header
```
Content-Security-Policy:
  default-src 'self';
  script-src 'self' 'wasm-unsafe-eval';
  style-src 'self' 'unsafe-inline';
  img-src 'self' blob: data:;
  connect-src 'self';
  object-src 'none';
  base-uri 'self';
```

### Key Crates
- `ammonia` - HTML sanitization (XSS prevention)
- `aes-gcm` - Client-side encryption for IndexedDB
- `indexed_db_futures` - IndexedDB access from WASM
- `gloo` - Web APIs for Rust/WASM
- `pdf-extract` or JS interop with PDF.js - PDF parsing
- `docx-rs` - DOCX parsing

---

## References
- [Leptos Docs](https://leptos.dev/)
- [OWASP Client-Side Security](https://owasp.org/www-project-top-10-client-side-security-risks/)
- [Nabu - World History Encyclopedia](https://www.worldhistory.org/Nabu/)
