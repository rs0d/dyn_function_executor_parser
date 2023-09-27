mod ast;

use ast::parse::{parser, DynType};

use crate::ast::{DynamicFunction, FUNCTION_REGISTRY};



fn sum(args: &[DynType]) -> Option<DynType> {
    let sum = args
        .iter()
        .filter_map(|arg| {
            if let DynType::I64(num) = arg {
                Some(num)
            } else {
                None
            }
        })
        .sum::<i64>();

    Some(DynType::I64(sum))
}

fn main() ->Result<(),anyhow::Error>{

    register_function!(
        sum
    );

   
 
    let input = "sum(6,74444,14564156416)";
    let ast = parser(input)?;


    let dyn_func   = ast.1;
        
    if let Some(result) = FUNCTION_REGISTRY.lock().unwrap().call_function(&dyn_func.name, dyn_func.clone().get_all_value().as_slice()) {
        println!("{:#?}", <DynType as Into<i64>>::into(result));
    } else {
        println!("Unknown function: sum");
    }

    Ok(())
}
