# RenderWalk

>  ### A minimal 3D software renderer and camera walkthrough built with Rust + SDL2.

## Installation

```sh
# Clone
git clone https://github.com/Adel-Ayoub/renderwalk.git
cd renderwalk

# Build
cargo build --release

# Run with an example model (omit arg to see generated cubes)
cargo run --release -- objects/airboat.obj
```

---

## Requirements

- Rust (stable) and Cargo
- SDL2 and SDL2_gfx built from source (universal)
- Build tools: C compiler, make, cmake, pkg-config, autoconf/automake/libtool

Install SDL2 from source:

```sh
# SDL2 (clone latest; you may checkout a specific release tag)
git clone https://github.com/libsdl-org/SDL.git third_party/SDL
cmake -S third_party/SDL -B third_party/SDL/build \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_INSTALL_PREFIX=$PWD/.local
cmake --build third_party/SDL/build --config Release -- -j
cmake --install third_party/SDL/build
```

Install SDL2_gfx from source:

```sh
git clone https://github.com/ferzkopp/SDL2_gfx.git third_party/SDL2_gfx
cd third_party/SDL2_gfx
./autogen.sh 2>/dev/null || autoreconf -fi
./configure --prefix=$PWD/../../.local
make -j
make install
cd -
```

Make sure your environment can find the installed libraries and pkg-config files:

```sh
export PKG_CONFIG_PATH=$PWD/.local/lib/pkgconfig:$PKG_CONFIG_PATH
export LIBRARY_PATH=$PWD/.local/lib:$LIBRARY_PATH
export LD_LIBRARY_PATH=$PWD/.local/lib:$LD_LIBRARY_PATH
export CPATH=$PWD/.local/include:$CPATH
```

---

## Usage

```text
RenderWalk — interactive camera walkthrough of OBJ scenes

Usage: renderwalk [OBJ_PATH]

Arguments:
  OBJ_PATH   Optional path to a .obj file to render. If omitted, a demo scene
             of generated cubes is shown.
```

## Controls

### Movement:

| Direction | Key        |
|-----------|------------|
| Forward   | W          |
| Backward  | S          |
| Left      | A          |
| Right     | D          |
| Up        | Arrow up   |
| Down      | Arrow down |

### Zoom:

| Zoom  | Key   |
|-------|-------|
| In    | Q     |
| Out   | E     |
| Reset | Space |

### Looking around:

| Look direction | Key |
|----------------|-----|
| Up             | I   |
| Down           | K   |
| Left           | J   |
| Right          | L   |

### Tilting camera:

| Tilt direction | Key |
|----------------|-----|
| Left           | U   |
| Right          | O   |

---

## Architecture overview

```text
keyboard ───────────────▶ event loop (main.rs) ─▶ scene: transform ▶ project ▶ screen_coords ─▶ SDL2 canvas
                                   ▲                       ▲
                                   │                       │
                   optional .obj ──┘ (obj.rs → polygons)   │
                                   │                       │
                             cgmath (matrices/quaternions) ┘
```

---

## Usage Examples

```sh
# Load classic teapot
cargo run --release -- example_objects/teapot.obj

# Load airboat
cargo run --release -- example_objects/airboat.obj

# No argument → generated cubes demo
cargo run --release
```
---

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.
