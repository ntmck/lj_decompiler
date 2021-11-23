// Uses a modified version of 3-address code as an intermediate representation.
//
// id      ->  identifer
// op      ->  operation
// L#:     ->  basic block identifier. Example: L0: ...
// args:   ->  args id-0, id-1, ..., id-N.
//
// Assignment  :   L0: id = id op id           //!
// Func Call   :   L0: call id # ; args        //! # is the number of args
// Conditional :   L0: if condition goto L#    //! if condition is true, go to L#
// Branch      :   L0: goto L#                 //! basic blocks will always have this as a last instruction.
// Unary       :   L0: id = id op              //!
//                                             //!
// LJ Specific                                 //!
//                                             //!
// Table Ops   :   L0: table[id] = id          //! includes _G as a table.
//                     id = table[id]          //!
// Constants   :   L0: id = constant           //!

mod ir_gen;
mod blocker;