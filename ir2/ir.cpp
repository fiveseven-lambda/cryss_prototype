#include <iostream>

#include "ir.hpp"

namespace ir {
    GlobalAddr::GlobalAddr(std::size_t pos): pos(pos) {}
    StackAddr::StackAddr(StackId stack_id, std::size_t pos): stack_id(stack_id), pos(pos) {}
    Env::Env(): num_stack(0) {}
    Value &Env::operator()(const GlobalAddr &addr){
        return global.at(addr.pos);
    }
    Value &Env::operator()(const StackAddr &addr){
        auto it = stack.find(addr.stack_id);
        if(it == stack.end()) throw;
        return it->second.at(addr.pos);
    }
    Func::~Func() = default;
    Value Assign::invoke(Env &env, std::vector<Value> args) const {
        std::visit(env, std::get<Addr>(args[0])) = args[1];
        return args[1];
    }
    Value Deref::invoke(Env & env, std::vector<Value> args) const {
        return std::visit(env, std::get<Addr>(args[0]));
    }
    Value IAdd::invoke(Env &, std::vector<Value> args) const {
        return std::get<Int>(args[0]) + std::get<Int>(args[1]);
    }
    Value IMul::invoke(Env &, std::vector<Value> args) const {
        return std::get<Int>(args[0]) * std::get<Int>(args[1]);
    }
    Value IEq::invoke(Env &, std::vector<Value> args) const {
        return std::get<Int>(args[0]) == std::get<Int>(args[1]);
    }
    Expr::~Expr() = default;
    Imm::Imm(Value value): value(value) {}
    Local::Local(std::size_t index): index(index) {}
    Value Imm::eval(Env &, StackId) const {
        return value;
    }
    Value Local::eval(Env &, StackId stack_id) const {
        return StackAddr(stack_id, index);
    }
    Call::Call(std::unique_ptr<Expr> func_expr, std::vector<std::unique_ptr<Expr>> args_expr):
        func_expr(std::move(func_expr)),
        args_expr(std::move(args_expr)) {}
    Value Call::eval(Env &env, StackId stack_id) const {
        auto func = std::get<std::shared_ptr<Func>>(func_expr->eval(env, stack_id));
        std::size_t num_args = args_expr.size();
        std::vector<Value> args(num_args);
        for(std::size_t i = 0; i < num_args; i++){
            args[i] = args_expr[i]->eval(env, stack_id);
        }
        return func->invoke(env, std::move(args));
    }
    ExprStmt::ExprStmt(std::unique_ptr<Expr> expr, std::size_t next):
        expr(std::move(expr)),
        next(next) {}
    std::size_t ExprStmt::run(Env &env, Value &dest, StackId stack_id) const {
        dest = expr->eval(env, stack_id);
        return next;
    }
    BrStmt::BrStmt(std::unique_ptr<Expr> cond, std::size_t next_true, std::size_t next_false):
        cond(std::move(cond)),
        next_true(next_true),
        next_false(next_false) {}
    std::size_t BrStmt::run(Env &env, Value &, StackId stack_id) const {
        if(std::get<Bool>(cond->eval(env, stack_id))){
            return next_true;
        }else{
            return next_false;
        }
    }
    Value FuncDef::invoke(Env &env, std::vector<Value> args) const {
        StackId stack_id = env.num_stack;
        args.resize(num_locals);
        auto stack_iter = env.stack.emplace(stack_id, std::move(args)).first;
        env.num_stack++;
        std::size_t cursor = 0;
        Value ret;
        while(cursor != stmts.size()){
            cursor = std::visit(
                [&](const auto &stmt){
                    return stmt.run(env, ret, stack_id);
                },
                stmts[cursor]
            );
        }
        env.stack.erase(stack_iter);
        return ret;
    }
}
