// Uses a modified version of 3-address code as an intermediate representation.
//
// id      ->  identifer
// op      ->  operation
// L#:     ->  basic block identifier. Example: L0: ...
// args:   ->  args id-0, id-1, ..., id-N.
//
// Assignment  :   L0: id = id op id           //!
// Func Call   :   L0: id(args)                //!
// Conditional :   L0: if condition L#         //! if condition is true, go to L#
// Unary       :   L0: id = id op              //!
//                                             //!
// LJ Specific                                 //!
//                                             //!
// Table Ops   :   L0: table[id] = id          //! includes _G as a table.
//                     id = table[id]          //!
// Constants   :   L0: id = constant           //!

//mod ir_gen;
//pub mod blocker;
//mod statement_builder;
