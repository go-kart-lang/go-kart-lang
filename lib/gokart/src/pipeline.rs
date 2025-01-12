use gokart_compile::Compiler;
use gokart_core::Code;
use gokart_decay::decay;
use gokart_parse::parse;
use gokart_vm::{State, Value, GC, VM};

//     let start = Instant::now();

//     {
//         let pipe = Pipeline::new(10_000);
//         let res = pipe.run_from_string(code_with_io, false);
//         println!("{:?}", res);
//     }

//     let elapsed = start.elapsed();
//     println!("===============================");
//     println!("Execution time: {:.3?}", elapsed);
//

pub struct Pipeline {
    gc_size: usize,
}

impl Pipeline {
    pub fn new(gc_size: usize) -> Self {
        Self { gc_size }
    }

    pub fn run_from_string(&self, input: &str, debug: bool) -> Result<String, String> {
        self.process(input, debug)
    }

    fn process(&self, input: &str, debug: bool) -> Result<String, String> {
        println!("{}", input);

        let ast = parse(input).map_err(|e| format!("Parse error: {:?}", e))?;
        if debug {
            println!("\n{:?}\n", ast);
        }

        let exp = decay(ast).map_err(|e| format!("Decay error: {:?}", e))?;
        if debug {
            println!("\n{:?}\n", exp);
        }

        let code = Compiler::compile(&exp);
        if debug {
            println!("\n{:?}\n", code);
        }

        let state = State::init_with(|h| h.alloc(Value::Empty));
        let gc = GC::new(self.gc_size);
        let mut vm = VM::new(state, Code::from(code), gc);

        vm.run();

        let res = vm.cur_env();
        if debug {
            println!("\n{:?}\n", res);
        }

        Ok(format!("Result: {:?}", res))
    }
}
