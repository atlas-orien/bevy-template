# Asset Guidelines

- Use lowercase kebab-case file names.
- Keep runtime assets under `assets/`.
- Prefer stable paths, because Bevy asset handles and AI-generated code often refer to them directly.
- `assets/` contains only runtime-ready files loaded by Bevy.
- Source files used by offline tools belong in `workbench/`.
- Document imported asset packs and licenses here when a project adds real assets.

## Runtime Layout

- `assets/2d/static`: static 2D images.
- `assets/2d/animated`: packed frame animation outputs.
- `assets/3d`: runtime 3D models and materials.
- `assets/ui`: UI images and UI-specific visual assets.
- `assets/audio`: runtime audio files.
- `assets/fonts`: runtime font files.
- `assets/levels`: runtime level data.
- `assets/scenes`: runtime scene data.

## Imported Asset Records

When importing third-party assets, add a project-specific record with:

- runtime path
- source URL or local source path
- license
- intended use
