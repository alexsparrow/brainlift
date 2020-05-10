
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module, FuncId};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};

fn putc(a: u8) -> u8 {
    print!("{}", a as char);
    a
}

pub struct JIT {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    module: Module<SimpleJITBackend>,
    putc: FuncId
}

impl JIT {
    pub fn new() -> Self {
        let putc_addr: *const u8 = putc as *const u8;

        let mut builder = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
        builder.symbol("putc", putc_addr);

        let mut module = Module::new(builder);

        let mut sig_a = module.make_signature();
        sig_a.params.push(AbiParam::new(types::I8));
        sig_a.returns.push(AbiParam::new(types::I8));

        let func_a = module
        .declare_function("putc", Linkage::Import, &sig_a)
        .unwrap();

        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            module,
            putc: func_a
        }
    }

    fn print_ir(&self) {
        let mut buf = String::new();
        cranelift::codegen::write_function(&mut buf, &self.ctx.func, &cranelift::codegen::ir::function::DisplayFunctionAnnotations::default())
            .expect("Oops");
        println!("{}", buf);
    }

    pub fn compile(&mut self, input: &str) -> Result<*const u8, String> {
        self.translate(input)
            .map_err(|e| e.to_string())?;

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

    fn translate(
        &mut self,
        s: &str,
    ) -> Result<(), String> {
        let int = self.module.target_config().pointer_type();
        let int8 = Type::int(8).expect("oops");

        let putc = self.module.declare_func_in_func(self.putc, &mut self.ctx.func);

        // Pointer to memory array
        self.ctx.func.signature.params.push(AbiParam::new(int));
        // Pointer to current memory address
        self.ctx.func.signature.params.push(AbiParam::new(int));
        // Arbitrary return value
        self.ctx.func.signature.returns.push(AbiParam::new(int));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);

        let memory = Variable::new(0);
        let pos: Variable = Variable::new(1);
        builder.declare_var(memory, int);
        builder.declare_var(pos, int);
        builder.def_var(memory, *builder.block_params(entry_block).first().expect("Oops"));
        builder.def_var(pos, *builder.block_params(entry_block).get(1).expect("Oops"));
        builder.seal_block(entry_block);

        let mut pc = 0;
        let memory_val = builder.use_var(memory);
        let pos_val = builder.use_var(pos);

        let mut stack: Vec<(Block, Block)> = Vec::new();

        while pc < s.len() {
            let c = s.chars().nth(pc).expect("Out of range");
            match c {
                '+' => {
                    let p = builder.ins().load(int8, MemFlags::new(), pos_val, 0);
                    let p_large = builder.ins().uextend(int, p);
                    let mem_addr = builder.ins().iadd(memory_val, p_large);
                    let v = builder.ins().load(int8, MemFlags::new(), mem_addr, 0);
                    let o = builder.ins().iadd_imm(v, 1);
                    builder.ins().store(MemFlags::new(), o, mem_addr, 0);
                },
                '-' => {
                    let p = builder.ins().load(int8, MemFlags::new(), pos_val, 0);
                    let p_large = builder.ins().uextend(int, p);
                    let mem_addr = builder.ins().iadd(memory_val, p_large);
                    let v = builder.ins().load(int8, MemFlags::new(), mem_addr, 0);
                    let o = builder.ins().iadd_imm(v, -1);
                    builder.ins().store(MemFlags::new(), o, mem_addr, 0);
                },
                '>' => {
                    let p = builder.ins().load(int8, MemFlags::new(), pos_val, 0);
                    let new_p = builder.ins().iadd_imm(p, 1);
                    builder.ins().store(MemFlags::new(), new_p, pos_val, 0);
                },
                '<' => {
                    let p = builder.ins().load(int8, MemFlags::new(), pos_val, 0);
                    let new_p = builder.ins().iadd_imm(p, -1);
                    builder.ins().store(MemFlags::new(), new_p, pos_val, 0);
                },
                '[' => {
                    let header_block = builder.create_block();
                    let current_block = builder.create_block();
                    let exit_block = builder.create_block();

                    builder.ins().jump(header_block, &[]);
                    builder.switch_to_block(header_block);

                    let p = builder.ins().load(int8, MemFlags::new(), pos_val, 0);
                    let p_large = builder.ins().uextend(int, p);
                    let mem_addr = builder.ins().iadd(memory_val, p_large);
                    let v = builder.ins().load(int8, MemFlags::new(), mem_addr, 0);
                    builder.ins().brz(v, exit_block, &[]);
                    builder.ins().jump(current_block, &[]);
                    builder.switch_to_block(current_block);
                    builder.seal_block(current_block);
                    builder.seal_block(exit_block);
                    stack.push((header_block, exit_block));
                },
                ']' => {
                    let (header_block, exit_block) = stack.pop().expect("Unbalanced loop");
                    builder.ins().jump(header_block, &[]);
                    builder.seal_block(header_block);
                    builder.switch_to_block(exit_block);

                    let current_block = builder.create_block();
                    builder.ins().jump(current_block, &[]);

                    builder.seal_block(current_block);
                    builder.switch_to_block(current_block);
                },
                '.' => {
                    let p = builder.ins().load(int8, MemFlags::new(), pos_val, 0);
                    let p_large = builder.ins().uextend(int, p);
                    let mem_addr = builder.ins().iadd(memory_val, p_large);
                    let v = builder.ins().load(int8, MemFlags::new(), mem_addr, 0);
                    builder.ins().call(putc, &[v]);
                },
                _ => ()
            }
            pc += 1
        }

        let c = builder.ins().iconst(int, 0);
        builder.ins().return_(&[c]);
        builder.finalize();
        Ok(())
    }
}

