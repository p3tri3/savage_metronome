# Structure Rules
- Domain logic must not import UI modules.
- No business logic inside UI layer.
- All side effects isolated to infrastructure/audio layers.

# Testing Rules
- Pure functions preferred.
- Avoid global state.
- Deterministic tests only.

# Complexity Rules
- Avoid deep nesting.
- Favor small cohesive modules.
- No premature abstraction.

# Refactoring Policy
- Prefer incremental refactoring.
- Do not introduce new patterns without justification.
