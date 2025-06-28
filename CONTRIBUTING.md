# Topic
**Bug fixes:** Always welcome! \
**New features:** Please open a new issue before opening a pull request for new features to discuss if it fits the library.

# Design goals of adar
- No custom `syntax` (for proper syntax highlighting and code navigation, faster to learn)
- Keep the generated code `minimal` (To see as much as possible)
- Retain as much `performance` as possible
- Make the resulting code as `verbose` as possible


# Design decisions
## Proc macro attribute vs Derive macro
Derive macros are more commonly used but they have a limitation. They cannot modify the existing code. In order to retain consistency along the library only proc macro attributes are used uniformly.
## Clean module files
In order to keep the file structure clean mod.rs and lib.rs files doesn't contain any meaningful code. Just the minimal required to define the project structure.