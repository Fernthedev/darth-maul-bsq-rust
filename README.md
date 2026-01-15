# Darth Maul Rust 

Darth Maul is a mod that allows you to hold a double blade saber in Beat Saber. Originally written by Pink, this was a 1:1 port of the C++ mod without functional UI. Not much more than that.

# Pros (applied in this PoC):
- Rust!
- Structured logging
- Memory safety
- Concurrently safe
- Clean error logging on panics 
- Serde!

# Caveats: 
- Big mod size for debug mods
- Involved build process for C++ (optional). Helpers in quest_hook to improve this
- Large binary size for debug builds
- Cordl includes per class as features
- Long compile times (slightly better incrementally)

This is also using a limited subset of what mods usually do. UI is implemented in C++. This however is a proof of concept to show how Rust mods can work, coexist with the current ecosystem and diversify the scene. Have fun!


Port of https://github.com/ModdingPink/QuestDarthMaul in Rust.

Missing:
- UI

Based off Rust mod template https://github.com/Fernthedev/pink_cute_rust/