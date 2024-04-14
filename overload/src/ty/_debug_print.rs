use super::*;
use std::collections::HashMap;

impl Ty {
    pub fn _debug_print(&self, vars: &HashMap<usize, Info>) -> String {
        match self {
            Ty::Var(id) => {
                if let Some(info) = vars.get(id) {
                    match info {
                        Info::Equal(ty) => format!("{id}={}", ty._debug_print(vars)),
                        Info::Ret(ty) => format!("{id}=..{}", ty._debug_print(vars)),
                    }
                } else {
                    format!("{id}")
                }
            }
            Ty::Const(constructor, args) => {
                if args.is_empty() {
                    constructor.to_owned()
                } else {
                    format!(
                        "{}[{}]",
                        constructor,
                        args.iter()
                            .map(|arg| arg._debug_print(vars))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            Ty::Func(args, ret) => {
                format!(
                    "({}){}",
                    args.iter()
                        .map(|arg| arg._debug_print(vars))
                        .collect::<Vec<_>>()
                        .join(", "),
                    ret._debug_print(vars)
                )
            }
        }
    }
}
impl Arg {
    pub fn _debug_print(&self, vars: &HashMap<usize, Info>) -> String {
        let mut ret = String::new();
        if let Some(from) = self.from {
            ret.push_str(&format!("<{}>", from));
        }
        if let Some(ty) = &self.ty {
            ret.push_str(&ty._debug_print(vars));
        }
        ret
    }
}

impl Field {
    pub fn _debug_print(&self, vars: &HashMap<usize, Info>) {
        for (
            index,
            Tower {
                candidates,
                calls,
                ret_to,
            },
        ) in self.0.iter().enumerate()
        {
            println!("<{index}>");
            for candidate in candidates {
                println!("  {}", candidate._debug_print(vars));
            }
            for (i, Call { args }) in calls.iter().enumerate() {
                println!(
                    "<{index}.{i}> {}",
                    args.iter()
                        .map(|arg| arg._debug_print(vars))
                        .collect::<Vec<_>>()
                        .join(",")
                );
            }
            if let Some((i, j, k)) = ret_to {
                println!("  -> <{i}.{j}.{k}>")
            }
        }
    }
}
