pub enum ExecutionError {
    ProgramComplete,
    InvalidJump,
    EmptyStack,
    LimitExceeded,
    PushLast,
    FinishedEmptyStack,
}

pub enum Instruction {
    STOP,
    ADD,
    PUSH,
    SUB,
    MUL,
    DIV,
    AND,
    OR,
    GT,
    LT,
    EQ,
    JUMP,
    JUMPI,
    Value(i32),
}

pub struct Interpreter {
    stack: Vec<i32>,
    code: Vec<Instruction>,
    program_counter: i32,
    execution_limit: i32,
    execution_count: i32,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            code: Vec::new(),
            stack: Vec::new(),
            program_counter: 0,
            execution_limit: 100000,
            execution_count: 0,
        }
    }

    fn jump(&mut self) -> Result<(), ExecutionError> {
        let destination = self.pop_stack();

        match (destination) {
            Some(destination) => {
                if (destination < 0) {
                    return Err(ExecutionError::InvalidJump);
                } else {
                    self.program_counter = destination - 1;
                    return Ok(());
                }
            }
            _ => return Err(ExecutionError::EmptyStack),
        }
    }

    fn pop_stack(&mut self) -> Option<i32> {
        return self.stack.pop();
    }

    pub fn run_code(&mut self, new_code: Vec<Instruction>) -> Result<(i32), ExecutionError> {
        self.code = new_code;

        while self.program_counter < self.code.len() as i32 {
            let op_code = &self.code[self.program_counter as usize];
            self.execution_count += 1;

            if (self.execution_count > self.execution_limit) {
                return Err(ExecutionError::LimitExceeded);
            }

            match op_code {
                Instruction::STOP => match self.stack.last().cloned() {
                    Some(result) => {
                        return Ok(result);
                    }
                    _ => {
                        return Err(ExecutionError::FinishedEmptyStack);
                    }
                },
                Instruction::ADD => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();

                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let sum = a + b;
                            self.stack.push(sum);
                        }
                        _ => return Err(ExecutionError::EmptyStack),
                    }
                }
                Instruction::SUB => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();

                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let difference = a - b;

                            self.stack.push(difference);
                        }
                        _ => return Err(ExecutionError::EmptyStack),
                    }
                }
                Instruction::MUL => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();

                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let product = a * b;

                            self.stack.push(product);
                        }
                        _ => return Err(ExecutionError::EmptyStack),
                    }
                }
                Instruction::DIV => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();

                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let quotient = a / b;

                            self.stack.push(quotient);
                        }
                        _ => return Err(ExecutionError::EmptyStack),
                    }
                }
                Instruction::AND => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();

                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let result = if a != 0 && b != 0 { 1 } else { 0 };

                            self.stack.push(result);
                        }
                        _ => return Err(ExecutionError::EmptyStack),
                    }
                }
                Instruction::OR => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();

                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let result = if a != 0 || b != 0 { 1 } else { 0 };

                            self.stack.push(result);
                        }
                        _ => return Err(ExecutionError::EmptyStack),
                    }
                }
                Instruction::PUSH => {
                    self.program_counter += 1;
                    match self.code.get(self.program_counter as usize) {
                        Some(&Instruction::Value(value)) => {
                            self.stack.push(value);
                        }
                        _ => return Err(ExecutionError::PushLast),
                    }
                }
                Instruction::LT => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();

                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let result = if a < b { 1 } else { 0 };

                            self.stack.push(result);
                        }
                        _ => return Err(ExecutionError::EmptyStack),
                    }
                }
                Instruction::GT => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();

                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let result = if a > b { 1 } else { 0 };

                            self.stack.push(result);
                        }
                        _ => return Err(ExecutionError::EmptyStack),
                    }
                }
                Instruction::EQ => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();

                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let result = if a == b { 1 } else { 0 };

                            self.stack.push(result);
                        }
                        _ => return Err(ExecutionError::EmptyStack),
                    }
                }
                Instruction::JUMP => match self.jump() {
                    Ok(_) => {}
                    Err(error) => {
                        return Err(error);
                    }
                },
                Instruction::JUMPI => {
                    let condition = self.pop_stack();

                    match (condition) {
                        Some(condition) => {
                            if condition != 0 {
                                match self.jump() {
                                    Ok(_) => {}
                                    Err(error) => {
                                        return Err(error);
                                    }
                                }
                            }
                        }
                        _ => return Err(ExecutionError::EmptyStack),
                    }
                }
                Instruction::Value(_value) => {}
            }

            self.program_counter += 1;
        }

        return Err(ExecutionError::ProgramComplete);
    }
}
