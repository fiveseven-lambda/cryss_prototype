use super::*;

impl Use {
    pub fn _debug_print(&self) {
        println!("{}", self.ty);
        for (i, call) in self.calls.iter().enumerate() {
            println!(
                "  Call #{i}: {}",
                call.args
                    .iter()
                    .map(|arg| arg.ty.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        println!("    {}", self.ret_ty);
    }
}
