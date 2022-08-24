Brainstorm for data-oriented ARPG data MVP:

1. Player Character

- Model
- Animations
  - A mapping of the model file's animations to a fixed set of expected animations would be useful I think, would allow working with all sorts of model file animations regardless of their names
  - Idle
  - Running
  - Hitting
- Stats
  - Hit points
  - Mana
- Skills
- Equipment

2. Ground tile

- Size?
  - Fixed size for each tile for now
- Model
- Position

3. Enemy

- Name
- Model
- Animations
- Stats
- Skills

4. Loot

- Name
- Type
- Stats, if any
- Model
- Equipment slot, if any
- Size
- Image (for inventory)

5. Skill

- Type: Projectile, Target
  - Effect based on type, for example:
    - Projectiles: Number of targets they pass through, size, speed, damage
    - Target: Single target or AoE, if AoE; size.
- Range
- Mana cost
- Cooldown
- Icon

6. Inventory

- Size
