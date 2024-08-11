
#[macro_export]
macro_rules! todo {
    () => {
        Err(Box::<dyn Error>::from("TODO"))
    }
}

#[macro_export]
macro_rules! unfinished {
    () => {
        Err(Box::<dyn Error>::from("UNFINISHED"))
    };
}