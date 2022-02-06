# Luajit 2.0.X Decompiler

Currently, the decompilation is not working as of yet; however, this can still disassemble LJ compiled files.

Work is currently in progress on an intermediate representation in between LJ and Lua source.

# Usage

All usage is done through unit tests at the moment. But here is a quick guide to using the rs files.

Getting prototypes (Disassembling):
- Create a new Prototyper.rs by using Prototyper.new() and supply it the path to a single compiled luajit file.
- Use Prototyper.next() to get the next prototype in the file.


Showing bytecode instructions:
- For each bytecode instruction in a prototype, use the default formatter to print them out.


Showing bytecode instructions separated into basic blocks:
- Create a new Blocker.rs by: Blocker{};
- Call Blocker.make_blocks() and supply it a reference to the prototype. It will return a vec of blocked bytecode instructions.
- For each basic block in the vec, use the default formatter to print them out.


(WIP) Showing bytecode instructions translated into the IR
- Follow the instructions to create a vec of blocks above.
- Make a new bci_translator.rs by Translater{};
- For each block and for each bytecode instruction in a block, use Translator.translate_bci() on each instruction to get an IR representation. Use the default formatter to print.
