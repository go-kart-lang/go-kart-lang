use gokart_core::{BinOp, Double, GOpCode, Int, Label, NullOp, OpCode, Tag, UnOp};
use rand::Rng;
use std::{
    io::{self, Write},
    iter,
};

use gokart_runtime::{gvalue_cast, Ref};

pub trait Ops {
    fn execute(&self, machine: *mut gokart_runtime::gokart_machine);
}

impl Ops for NullOp {
    fn execute(&self, machine: *mut gokart_runtime::gokart_machine) {
        use NullOp::*;
        let ptr = match self {
            IntLit(val) => gokart_runtime::gokart_allocate_int(machine, *val),
            DoubleLit(val) => gokart_runtime::gokart_allocate_double(machine, *val),
            StrLit(val) => alloc_string(machine, val),
        };

        unsafe { &mut *machine }.env = ptr;
        unsafe { &mut *machine }.ip += 1;
    }
}

fn alloc_string(
    machine: *mut gokart_runtime::gokart_machine,
    s: &String,
) -> *mut gokart_runtime::gokart_value {
    gokart_runtime::gokart_allocate_string(machine, s.len() as u64, s.clone().as_mut_ptr())
}

fn get_string(ptr: *mut gokart_runtime::gokart_value) -> String {
    let size = *gokart_runtime::gvalue_cast::<u64>(ptr);
    let struct_size = std::mem::size_of::<gokart_runtime::GValue<u64>>();
    let str_ptr = unsafe { (ptr as *mut u8).byte_add(struct_size) };
    let slice = unsafe { std::slice::from_raw_parts(str_ptr, size as usize) };

    String::from_utf8_lossy(slice).to_string()
}

fn get_env(machine: *mut gokart_runtime::gokart_machine) -> *mut gokart_runtime::gokart_value {
    unsafe { &mut *machine }.env
}

impl Ops for UnOp {
    fn execute(&self, machine: *mut gokart_runtime::gokart_machine) {
        use UnOp::*;

        match self {
            Print => {
                let val = get_string(get_env(machine));
                println!("{val}");
                unsafe { &mut *machine }.env = std::ptr::null_mut();
            }
            Read => {
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();

                unsafe { &mut *machine }.env = alloc_string(machine, &input);
            }
            Int2Str => {
                let val = gokart_runtime::gvalue_cast::<i64>(get_env(machine));
                unsafe { &mut *machine }.env = alloc_string(machine, &val.to_string());
            }
            Str2Int => {
                let val = get_string(get_env(machine));
                let res = match val.parse::<i64>() {
                    Ok(x) => x,
                    Err(e) => panic!("Error at Str('{val}') to Int conversion: {e}"),
                };
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(machine, res);
            }
            Double2Str => {
                let val = gokart_runtime::gvalue_cast::<f64>(get_env(machine));
                unsafe { &mut *machine }.env = alloc_string(machine, &val.to_string());
            }
            Str2Double => {
                let val = get_string(get_env(machine));
                let res = match val.parse::<f64>() {
                    Ok(x) => x,
                    Err(e) => panic!("Error at Str('{val}') to Double conversion: {e}"),
                };
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_double(machine, res);
            }
            Double2Int => {
                let val = gokart_runtime::gvalue_cast::<f64>(get_env(machine));
                unsafe { &mut *machine }.env =
                    gokart_runtime::gokart_allocate_int(machine, *val as i64);
            }
            Int2Double => {
                let val = gokart_runtime::gvalue_cast::<i64>(get_env(machine));
                unsafe { &mut *machine }.env =
                    gokart_runtime::gokart_allocate_double(machine, *val as f64);
            }
            VectorIntLength => {
                let val = gokart_runtime::gvalue_cast::<rpds::Vector<Int>>(get_env(machine));
                unsafe { &mut *machine }.env =
                    gokart_runtime::gokart_allocate_int(machine, val.len() as i64);
            }
            VectorIntFillRandom => {
                let size = *gokart_runtime::gvalue_cast::<i64>(get_env(machine));
                let mut vec = rpds::Vector::new();
                let mut rng = rand::thread_rng();
                vec.extend((0..size).map(|_| rng.gen::<i64>()));

                let ptr = gokart_runtime::gokart_allocate_vector_int(machine);
                let val = gokart_runtime::gvalue_cast::<rpds::Vector<Int>>(ptr);
                *val = vec;
                unsafe { &mut *machine }.env = ptr;
            }
        }
        unsafe { &mut *machine }.ip += 1;
    }
}

impl Ops for BinOp {
    fn execute(&self, machine: *mut gokart_runtime::gokart_machine) {
        use BinOp::*;

        let a_ref = gokart_runtime::gokart_stack_pop(machine);
        let b_ref = get_env(machine);

        match self {
            IntPlus => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    *gvalue_cast::<Int>(a_ref) + *gvalue_cast::<Int>(b_ref),
                );
            }
            IntMul => {
                let a = *gvalue_cast::<Int>(a_ref);
                let b = *gvalue_cast::<Int>(b_ref);
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(machine, a * b);
            }
            IntMinus => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    *gvalue_cast::<Int>(a_ref) - *gvalue_cast::<Int>(b_ref),
                );
            }
            IntDiv => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    *gvalue_cast::<Int>(a_ref) / *gvalue_cast::<Int>(b_ref),
                );
            }
            IntLt => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Int>(a_ref) < *gvalue_cast::<Int>(b_ref)) as Int,
                );
            }
            IntLe => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Int>(a_ref) <= *gvalue_cast::<Int>(b_ref)) as Int,
                );
            }
            IntEq => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Int>(a_ref) == *gvalue_cast::<Int>(b_ref)) as Int,
                );
            }
            IntNe => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Int>(a_ref) != *gvalue_cast::<Int>(b_ref)) as Int,
                );
            }
            IntGt => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Int>(a_ref) > *gvalue_cast::<Int>(b_ref)) as Int,
                );
            }
            IntGe => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Int>(a_ref) >= *gvalue_cast::<Int>(b_ref)) as Int,
                );
            }
            DoublePlus => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_double(
                    machine,
                    *gvalue_cast::<Double>(a_ref) + *gvalue_cast::<Double>(b_ref),
                );
            }
            DoubleMul => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_double(
                    machine,
                    *gvalue_cast::<Double>(a_ref) * *gvalue_cast::<Double>(b_ref),
                )
            }
            DoubleMinus => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_double(
                    machine,
                    *gvalue_cast::<Double>(a_ref) - *gvalue_cast::<Double>(b_ref),
                )
            }
            DoubleDiv => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_double(
                    machine,
                    *gvalue_cast::<Double>(a_ref) / *gvalue_cast::<Double>(b_ref),
                )
            }
            DoubleLt => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Double>(a_ref) < *gvalue_cast::<Double>(b_ref)) as Int,
                )
            }
            DoubleLe => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Double>(a_ref) <= *gvalue_cast::<Double>(b_ref)) as Int,
                )
            }
            DoubleEq => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Double>(a_ref) == *gvalue_cast::<Double>(b_ref)) as Int,
                )
            }
            DoubleNe => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Double>(a_ref) != *gvalue_cast::<Double>(b_ref)) as Int,
                )
            }
            DoubleGt => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Double>(a_ref) > *gvalue_cast::<Double>(b_ref)) as Int,
                )
            }
            DoubleGe => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (*gvalue_cast::<Double>(a_ref) >= *gvalue_cast::<Double>(b_ref)) as Int,
                )
            }
            StrPlus => {
                let a_str = get_string(a_ref);
                let b_str = get_string(b_ref);
                unsafe { &mut *machine }.env = alloc_string(machine, &(a_str + &b_str));
            }
            StrEq => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (get_string(a_ref) == get_string(b_ref)) as Int,
                )
            }
            StrNe => {
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(
                    machine,
                    (get_string(a_ref) != get_string(b_ref)) as Int,
                )
            }
            VectorIntFill => {
                let size = *gvalue_cast::<Int>(a_ref);
                let val = *gvalue_cast::<Int>(b_ref);
                let mut vec = rpds::Vector::new();
                vec.extend(iter::repeat(val).take(size as usize));

                let ptr = gokart_runtime::gokart_allocate_vector_int(machine);
                let val = gokart_runtime::gvalue_cast::<rpds::Vector<Int>>(ptr);
                *val = vec;
                unsafe { &mut *machine }.env = ptr;
            }
            VectorIntGet => {
                let vec = gvalue_cast::<rpds::Vector<Int>>(a_ref);
                let idx = *gvalue_cast::<Int>(b_ref) as usize;
                let val = *vec.get(idx).unwrap();
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_int(machine, val);
            }
            VectorIntUpdate => {
                let vec = gvalue_cast::<rpds::Vector<Int>>(a_ref);
                let (idx_ref, v_ref) = *gvalue_cast::<(Ref, Ref)>(b_ref);
                let idx = *gvalue_cast::<Int>(idx_ref) as usize;
                let v = *gvalue_cast::<Int>(v_ref);

                let ptr = gokart_runtime::gokart_allocate_vector_int(machine);
                let val = gokart_runtime::gvalue_cast::<rpds::Vector<Int>>(ptr);
                *val = vec.set(idx, v).unwrap();
                unsafe { &mut *machine }.env = ptr;
            }
            VectorIntUpdateMut => {
                let vec = gvalue_cast::<rpds::Vector<Int>>(a_ref);
                let (idx_ref, v_ref) = *gvalue_cast::<(Ref, Ref)>(b_ref);
                let idx = *gvalue_cast::<Int>(idx_ref) as usize;
                let v = *gvalue_cast::<Int>(v_ref);

                vec.set_mut(idx, v);
            }
        };

        unsafe { &mut *machine }.ip += 1;
    }
}

impl Ops for OpCode {
    #[inline]
    fn execute(&self, machine: *mut gokart_runtime::gokart_machine) {
        use GOpCode::*;

        match self {
            Acc(n) => {
                for _ in 0..*n {
                    unsafe { &mut *machine }.env = gvalue_cast::<(Ref, Ref)>(get_env(machine)).0
                }
                unsafe { &mut *machine }.env = gvalue_cast::<(Ref, Ref)>(get_env(machine)).1;
                unsafe { &mut *machine }.ip += 1;
            }
            Rest(n) => {
                for _ in 0..*n {
                    unsafe { &mut *machine }.env = gvalue_cast::<(Ref, Ref)>(get_env(machine)).0
                }
                unsafe { &mut *machine }.ip += 1;
            }
            Push => {
                gokart_runtime::gokart_stack_push(machine, get_env(machine));
                unsafe { &mut *machine }.ip += 1;
            }
            Swap => {
                let m = unsafe { &mut *machine };
                let top = unsafe { &mut *m.stack.data.add((m.stack.length - 1) as usize) };
                let tmp = *top;
                *top = m.env;
                m.env = tmp;

                unsafe { &mut *machine }.ip += 1;
            }
            Sys0(op) => op.execute(machine),
            Sys1(op) => op.execute(machine),
            Sys2(op) => op.execute(machine),
            Cur(label) => {
                unsafe { &mut *machine }.env =
                    gokart_runtime::gokart_allocate_closure(machine, get_env(machine), *label);
                unsafe { &mut *machine }.ip += 1;
            }
            Return => {
                let r = gokart_runtime::gokart_stack_pop(machine);
                unsafe { &mut *machine }.ip = *gvalue_cast::<Label>(r);
            }
            Clear => {
                unsafe { &mut *machine }.env = std::ptr::null_mut();
                unsafe { &mut *machine }.ip += 1;
            }
            Cons => {
                let a = gokart_runtime::gokart_stack_pop(machine);
                let b = get_env(machine);
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_pair(machine, a, b);
                unsafe { &mut *machine }.ip += 1;
            }
            App => {
                let b = gokart_runtime::gokart_stack_pop(machine);
                let (a, label) = gvalue_cast::<(Ref, Label)>(get_env(machine));
                unsafe { &mut *machine }.env = gokart_runtime::gokart_allocate_pair(machine, *a, b);

                let r =
                    gokart_runtime::gokart_allocate_label(machine, unsafe { &mut *machine }.ip + 1);

                gokart_runtime::gokart_stack_push(machine, r);
                unsafe { &mut *machine }.ip = *label;
            }
            Pack(tag) => {
                unsafe { &mut *machine }.env =
                    gokart_runtime::gokart_allocate_tagged(machine, *tag, get_env(machine));
                unsafe { &mut *machine }.ip += 1;
            }
            Skip => {
                unsafe { &mut *machine }.ip += 1;
            }
            Stop => {
                unsafe { &mut *machine }.is_running = 0;
            }
            Call(label) => {
                let r =
                    gokart_runtime::gokart_allocate_label(machine, unsafe { &mut *machine }.ip + 1);
                gokart_runtime::gokart_stack_push(machine, r);
                unsafe { &mut *machine }.ip = *label;
            }
            GotoFalse(label) => {
                let new_env = gokart_runtime::gokart_stack_pop(machine);
                let b = gvalue_cast::<Int>(get_env(machine));
                unsafe { &mut *machine }.env = new_env;
                if *b == 0 {
                    unsafe { &mut *machine }.ip = *label;
                } else {
                    unsafe { &mut *machine }.ip += 1;
                }
            }
            Switch(tag, label) => {
                let (cur_tag, b) = gvalue_cast::<(Tag, Ref)>(get_env(machine));
                if *cur_tag == *tag {
                    let a = gokart_runtime::gokart_stack_pop(machine);
                    unsafe { &mut *machine }.env =
                        gokart_runtime::gokart_allocate_pair(machine, a, *b);
                    unsafe { &mut *machine }.ip = *label;
                } else {
                    unsafe { &mut *machine }.ip += 1;
                }
            }
            Goto(label) => {
                unsafe { &mut *machine }.ip = *label;
            }
        }
    }
}
