use crate::state::BrainfuckState;
use crate::stdlib::{DefaultStdLib, StdLib};
use cranelift::codegen::ir::function::DisplayFunctionAnnotations;
use cranelift::codegen::write_function;
use cranelift::prelude::*;
use cranelift_module::{default_libcall_names, FuncId, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};
use std::mem;

fn putc<T: StdLib>(stdlib: *mut T, a: u8) -> u8 {
    return unsafe { stdlib.as_mut().expect("OOPS").putc(a) };
}

fn getc<T: StdLib>(stdlib: *mut T) -> u8 {
    return unsafe { stdlib.as_mut().expect("OOPS").getc() };
}

struct State<'a> {
    memory: &'a Value,
    pos: &'a Value,
}

fn mem_read(builder: &mut FunctionBuilder, state: &State, int: Type, int8: Type) -> (Value, Value) {
    let p = builder.ins().load(int8, MemFlags::new(), *state.pos, 0);
    let p_large = builder.ins().uextend(int, p);
    let mem_addr = builder.ins().iadd(*state.memory, p_large);
    return (
        mem_addr,
        builder.ins().load(int8, MemFlags::new(), mem_addr, 0),
    );
}

pub struct JIT<'a, T: StdLib> {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    module: Module<SimpleJITBackend>,
    putc: FuncId,
    getc: FuncId,
    int: Type,
    int8: Type,
    stdlib: &'a T,
}

impl<'a, T: StdLib> JIT<'a, T> {
    pub fn new(stdlib: &'a T) -> Self {
        let mut builder = SimpleJITBuilder::new(default_libcall_names());
        builder.symbol("putc", putc::<T> as *const u8);
        builder.symbol("getc", getc::<T> as *const u8);

        let mut module = Module::new(builder);

        let mut sig_putc = module.make_signature();
        sig_putc
            .params
            .push(AbiParam::new(module.target_config().pointer_type()));
        sig_putc.params.push(AbiParam::new(types::I8));
        sig_putc.returns.push(AbiParam::new(types::I8));

        let putc = module
            .declare_function("putc", Linkage::Import, &sig_putc)
            .unwrap();

        let mut sig_getc = module.make_signature();
        sig_getc
            .params
            .push(AbiParam::new(module.target_config().pointer_type()));
        sig_getc.returns.push(AbiParam::new(types::I8));

        let getc = module
            .declare_function("getc", Linkage::Import, &sig_getc)
            .unwrap();

        let int = module.target_config().pointer_type();
        let int8 = Type::int(8).expect("oops");

        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            module,
            putc,
            getc,
            int,
            int8,
            stdlib,
        }
    }

    fn print_ir(&self) {
        let mut buf = String::new();
        write_function(
            &mut buf,
            &self.ctx.func,
            &DisplayFunctionAnnotations::default(),
        )
        .expect("Oops");
        println!("{}", buf);
    }

    pub fn compile(&mut self, input: &str) -> Result<*const u8, String> {
        self.translate(input).map_err(|e| e.to_string())?;

        self.print_ir();

        let id = self
            .module
            .declare_function(&"foo", Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;

        self.module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())?;

        self.module.clear_context(&mut self.ctx);

        self.module.finalize_definitions();

        let code = self.module.get_finalized_function(id);
        Ok(code)
    }

    fn translate(&mut self, s: &str) -> Result<(), String> {
        let putc = self
            .module
            .declare_func_in_func(self.putc, &mut self.ctx.func);
        let getc = self
            .module
            .declare_func_in_func(self.getc, &mut self.ctx.func);

        // Pointer to memory array
        self.ctx.func.signature.params.push(AbiParam::new(self.int));
        // Pointer to current memory address
        self.ctx.func.signature.params.push(AbiParam::new(self.int));
        // Arbitrary return value
        self.ctx
            .func
            .signature
            .returns
            .push(AbiParam::new(self.int));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);

        let memory = Variable::new(0);
        let pos: Variable = Variable::new(1);
        builder.declare_var(memory, self.int);
        builder.declare_var(pos, self.int);
        builder.def_var(
            memory,
            *builder.block_params(entry_block).first().expect("Oops"),
        );
        builder.def_var(
            pos,
            *builder.block_params(entry_block).get(1).expect("Oops"),
        );
        builder.seal_block(entry_block);

        let mut pc = 0;
        let memory_val = builder.use_var(memory);
        let pos_val = builder.use_var(pos);

        let state = State {
            memory: &memory_val,
            pos: &pos_val,
        };
        let mut stack: Vec<(Block, Block)> = Vec::new();

        while pc < s.len() {
            let c = s.chars().nth(pc).expect("Out of range");
            match c {
                '+' => {
                    let (addr, value) = mem_read(&mut builder, &state, self.int, self.int8);
                    let o = builder.ins().iadd_imm(value, 1);
                    builder.ins().store(MemFlags::new(), o, addr, 0);
                }
                '-' => {
                    let (addr, value) = mem_read(&mut builder, &state, self.int, self.int8);
                    let o = builder.ins().iadd_imm(value, -1);
                    builder.ins().store(MemFlags::new(), o, addr, 0);
                }
                '>' => {
                    let p = builder.ins().load(self.int8, MemFlags::new(), pos_val, 0);
                    let new_p = builder.ins().iadd_imm(p, 1);
                    builder.ins().store(MemFlags::new(), new_p, pos_val, 0);
                }
                '<' => {
                    let p = builder.ins().load(self.int8, MemFlags::new(), pos_val, 0);
                    let new_p = builder.ins().iadd_imm(p, -1);
                    builder.ins().store(MemFlags::new(), new_p, pos_val, 0);
                }
                '[' => {
                    let header_block = builder.create_block();
                    let current_block = builder.create_block();
                    let exit_block = builder.create_block();

                    builder.ins().jump(header_block, &[]);
                    builder.switch_to_block(header_block);

                    let (_, value) = mem_read(&mut builder, &state, self.int, self.int8);
                    builder.ins().brz(value, exit_block, &[]);
                    builder.ins().jump(current_block, &[]);
                    builder.switch_to_block(current_block);
                    builder.seal_block(current_block);
                    builder.seal_block(exit_block);
                    stack.push((header_block, exit_block));
                }
                ']' => {
                    let (header_block, exit_block) = stack.pop().expect("Unbalanced loop");
                    builder.ins().jump(header_block, &[]);
                    builder.seal_block(header_block);
                    builder.switch_to_block(exit_block);

                    let current_block = builder.create_block();
                    builder.ins().jump(current_block, &[]);

                    builder.seal_block(current_block);
                    builder.switch_to_block(current_block);
                }
                '.' => {
                    let (_, value) = mem_read(&mut builder, &state, self.int, self.int8);
                    let stdlib_ptr = builder
                        .ins()
                        .iconst(self.int, self.stdlib as *const _ as i64);
                    builder.ins().call(putc, &[stdlib_ptr, value]);
                }
                ',' => {
                    let p = builder.ins().load(self.int8, MemFlags::new(), pos_val, 0);
                    let p_large = builder.ins().uextend(self.int, p);
                    let mem_addr = builder.ins().iadd(memory_val, p_large);

                    let stdlib_ptr = builder
                        .ins()
                        .iconst(self.int, self.stdlib as *const _ as i64);
                    let call = builder.ins().call(getc, &[stdlib_ptr]);
                    let value = {
                        let results = builder.inst_results(call);
                        assert_eq!(results.len(), 1);
                        results[0].clone()
                    };

                    builder.ins().store(MemFlags::new(), value, mem_addr, 0);
                }
                _ => (),
            }
            pc += 1
        }

        let c = builder.ins().iconst(self.int, 0);
        builder.ins().return_(&[c]);
        builder.finalize();
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct BrainfuckJITFunction {
    f: *const u8,
}

pub fn brainfuck_jit_compile<T: StdLib>(input: &str, stdlib: &T) -> BrainfuckJITFunction {
    let mut j = JIT::new(stdlib);
    return BrainfuckJITFunction {
        f: j.compile(input).expect("Wat"),
    };
}

pub fn brainfuck_jit_run(func: BrainfuckJITFunction, state: &mut BrainfuckState) {
    let func = unsafe { mem::transmute::<_, fn(&mut [u8; 1024], &mut usize) -> isize>(func.f) };
    func(&mut state.mem, &mut state.pos);
}

pub fn brainfuck_jit_state<T: StdLib>(input: &str, stdlib: &mut T, state: &mut BrainfuckState) {
    let f = brainfuck_jit_compile(input, stdlib);
    brainfuck_jit_run(f, state);
}

pub fn brainfuck_jit(input: &str) {
    brainfuck_jit_state(input, &mut DefaultStdLib {}, &mut BrainfuckState::new());
}
