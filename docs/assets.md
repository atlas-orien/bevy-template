# Asset Guidelines

- Use lowercase kebab-case file names.
- Keep runtime assets under `assets/`.
- Prefer stable paths, because Bevy asset handles and AI-generated code often refer to them directly.
- Document imported asset packs and licenses in this file.

## Imported Assets

- `assets/2d/characters/bevy/gabe/gabe-idle-run.png`
  - Source: Bevy official repository, `assets/textures/rpg/chars/gabe/gabe-idle-run.png`
  - Local reference: `/Users/ancient/src/github/bevy/assets/textures/rpg/chars/gabe/gabe-idle-run.png`
  - License: Bevy repository asset license
  - Use: 2D player character animation demo
