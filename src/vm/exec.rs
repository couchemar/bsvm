use vm::stack::Stack;
use vm::instructions::Instructions;


pub struct VM {
    pub stack: Stack,
    pub instructions: Instructions
}
