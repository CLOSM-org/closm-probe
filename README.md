# CLOSM Probe

**3D Storage Universe Explorer** - Visualize your files as an explorable solar system.

![Status](https://img.shields.io/badge/status-MVP-blue)
![Next.js](https://img.shields.io/badge/Next.js-16-black)
![License](https://img.shields.io/badge/license-MIT-green)

---

## Overview

CLOSM Probe transforms traditional file management into an intuitive 3D space exploration experience. Instead of navigating folders and files in a flat hierarchy, you explore a **solar system** where:

- **Sun** = Current directory (center of your view)
- **Planets** = Subdirectories (orbiting the sun)
- **Satellites** = Files (orbiting their parent planets)
- **Asteroid Belts** = Small files consolidated into rings

### Why Space Metaphor?

| Traditional Problem | CLOSM Solution |
|---------------------|----------------|
| Hard to see what's eating storage | Large planets = large folders |
| Can't distinguish old vs new files | Bright = recent, dim = old |
| File types are hidden | Color-coded by type |
| Deep hierarchies are confusing | Drill-down navigation |

---

## Features

### Current (MVP)

- 3D solar system visualization with React Three Fiber
- Drill-down navigation (double-click to explore)
- Visual encoding:
  - **Size** = File/folder capacity
  - **Color** = File type (code, design, image, video, etc.)
  - **Brightness** = Recency (bright = recently modified)
  - **Orbital radius** = Creation date (newer = inner orbit)
  - **Angular position** = Last modified (newest at 12 o'clock)
- Asteroid belt model for small files (<100KB)
- Smooth rotation and zoom
- Selection and detail panel
- Post-processing effects (bloom, vignette)

### Planned

- Google Drive integration
- File type / date / size filtering
- AI-powered relationship analysis
- Physics simulation for natural clustering
- Desktop app (Electron/Tauri)
- VR/AR exploration

---

## Getting Started

### Prerequisites

- Node.js 18+
- npm or yarn

### Installation

```bash
git clone https://github.com/CLOSM-org/closm-probe.git
cd closm-probe
npm install
```

### Development

```bash
npm run dev
```

Open [http://localhost:3100](http://localhost:3100) in your browser.

### Build

```bash
npm run build
npm start
```

---

## Tech Stack

| Category | Technology |
|----------|------------|
| Framework | Next.js 16 (App Router) |
| Language | TypeScript |
| 3D Rendering | Three.js + React Three Fiber |
| Styling | Tailwind CSS |

---

## Documentation

| Document | Description |
|----------|-------------|
| [Product Design](./docs/design/product-design.md) | Full design specification |
| [Canvas Rendering](./docs/specifications/canvas-rendering.md) | Technical rendering details |
| [CLAUDE.md](./CLAUDE.md) | Development guidelines |

---

## Project Structure

```
closm-probe/
├── src/
│   ├── app/                    # Next.js App Router
│   └── components/             # React components
│       ├── PhysicalStorageUniverse.tsx  # Main visualizer
│       └── universe/           # 3D visualization components
│           ├── controls/       # Camera controls
│           ├── effects/        # Star field, orbit lines
│           ├── nodes/          # Planet, Satellite, AsteroidBelt
│           ├── postprocessing/ # Bloom, vignette effects
│           └── types.ts        # Shared types & helpers
├── docs/
│   ├── design/                 # Design specifications
│   └── specifications/         # Technical specifications
└── public/                     # Static assets
```

---

## Contributing

Contributions are welcome! Please read our documentation before submitting PRs.

---

## License

MIT License - see [LICENSE](./LICENSE) for details.

---

## Links

- [GitHub Repository](https://github.com/CLOSM-org/closm-probe)
- [Documentation](./docs/README.md)
