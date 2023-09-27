use std::{collections::HashMap, sync::{Mutex, Arc}};
use lazy_static::lazy_static;
use self::parse::DynType;

pub mod parse;


pub trait DynamicFunctionTrait {
    fn call(&self, args: &[DynType]) -> Option<DynType>;
}

pub struct DynamicFunction {
    pub(crate) func: fn(args: &[DynType]) -> Option<DynType>,
}

impl DynamicFunctionTrait for DynamicFunction {
    fn call(&self, args: &[DynType]) -> Option<DynType> {
        (self.func)(args)
    }
}

pub struct FunctionRegistry {
    functions: Mutex<HashMap<String, Box<dyn DynamicFunctionTrait>>>,
}

unsafe impl Sync for FunctionRegistry {}

unsafe impl Send for FunctionRegistry {}

impl FunctionRegistry {
    pub fn new() -> Self {
        FunctionRegistry {
            functions: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_function(&mut self, name: String, function: Box<dyn DynamicFunctionTrait>) {
        self.functions.lock().unwrap().insert(name, function);
    }

    pub fn call_function(&self, name: &str, args: &[DynType]) -> Option<DynType> {
        if let Some(func) = self.functions.lock().unwrap().get(name) {
            func.call(args)
        } else {
            None
        }
    }
}

lazy_static! {
   pub static ref FUNCTION_REGISTRY: Arc<Mutex<FunctionRegistry>> = Arc::new(FunctionRegistry::new().into());
}



#[macro_export]
macro_rules! register_function {
    ($func:expr) => {
        {
            use $crate::ast::DynamicFunctionTrait;
            use $crate::ast::FUNCTION_REGISTRY;

            let function_name = stringify!($func);
            let boxed_func: Box<dyn DynamicFunctionTrait> = Box::new(DynamicFunction{func:$func});
            let mut registry = FUNCTION_REGISTRY.lock().unwrap();
            registry.register_function(function_name.to_string(), boxed_func);
        }
    };
}
