mod traits;
mod trait_params;
mod into;
mod drop;
mod operator_overloading;
mod static_dispatch;
mod dynamic_dispatch;
mod vector_diff_obj;
mod ownership;
mod lifetime;
mod rc_variables;
mod arc_variables;
mod mutex;
mod circular_references;
mod concurency;


fn main() {
    println!("=====trait=====");
    traits::main_trait();
    println!("=====trait_params=====");
    trait_params::main_trait_param();
    println!("=====into=====");
    into::main_into();
    println!("====drop====");
    drop::main_drop();
    println!("====operator overloadin=====");
    operator_overloading::main_operator_o();
    println!("=====static dispatch====");
    static_dispatch::main_static_d();
    println!("====dynamic dispatch====");
    dynamic_dispatch::main_dynamic_d();
    println!("====vector different object====");
    vector_diff_obj::main_vctr_diff_obj();
    println!("====ownership====");
    ownership::main_ownership();
    println!("====lifetime====");
    lifetime::main_lifetime();
    println!("====Reference-counter variables====");
    rc_variables::main_rc_variable();
    println!("====atomic reference-counter variables====");
    arc_variables::main_arc();
    println!("====Mutex====");
    mutex::main_mutex();
    println!("====Circular references====");
    circular_references::main_cr();
    println!("====concurency====");
    concurency::main_concurancy();
}
