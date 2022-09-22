#include <iostream>
#include <vector>
#include <memory>

struct Indent {
    int depth;
public:
    Indent(int depth): depth(depth) {}
    friend std::ostream &operator<<(std::ostream &os, const Indent &indent) {
        for(int i = 0; i < indent.depth; i++) os << "  ";
        return os;
    }
};

namespace ir {
    struct Value {
        virtual std::shared_ptr<Value> copy() = 0;
        virtual ~Value() = default;
        virtual void debug_print(int = 0) = 0;
    };
    struct Int : public Value {
        int value;
        Int(int value): value(value) {}
        std::shared_ptr<Value> copy() override {
            return std::make_shared<Int>(value);
        }
        void debug_print(int depth) override {
            std::cout << Indent(depth) << "int: " << value << std::endl;
        }
    };
    struct Bool : public Value {
        bool value;
        Bool(bool value): value(value) {}
        std::shared_ptr<Value> copy() override {
            return std::make_shared<Bool>(value);
        }
        void debug_print(int depth) override {
            std::cout << Indent(depth) << "bool: " << value << std::endl;
        }
    };

    struct Func {
        virtual ~Func() = default;
        virtual std::shared_ptr<Value> invoke(
            std::vector<std::shared_ptr<Value>> &,
            std::vector<std::shared_ptr<Value>> &
        ) = 0;
    };
    struct RExpr {
        virtual ~RExpr() = default;
        virtual std::shared_ptr<Value> rvalue(
            std::vector<std::shared_ptr<Value>> &,
            std::vector<std::shared_ptr<Value>> &
        ) = 0;
        void exec(
            std::vector<std::shared_ptr<Value>> &globals
        ){
            std::vector<std::shared_ptr<Value>> locals;
            rvalue(globals, locals);
        }
    };
    struct Imm : public RExpr {
        std::shared_ptr<Value> value;
        Imm(std::shared_ptr<Value> value):
            value(value) {}
        std::shared_ptr<Value> rvalue(
            std::vector<std::shared_ptr<Value>> &,
            std::vector<std::shared_ptr<Value>> &
        ) override {
            return value;
        }
    };
    struct LExpr : public RExpr {
        virtual std::shared_ptr<Value> &lvalue(
            std::vector<std::shared_ptr<Value>> &,
            std::vector<std::shared_ptr<Value>> &
        ) = 0;
        std::shared_ptr<Value> rvalue(
            std::vector<std::shared_ptr<Value>> &globals,
            std::vector<std::shared_ptr<Value>> &locals
        ) override {
            return lvalue(globals, locals);
        }
    };
    struct Global : public LExpr {
        std::size_t index;
        Global(std::size_t index): index(index) {}
        std::shared_ptr<Value> &lvalue(
            std::vector<std::shared_ptr<Value>> &globals,
            std::vector<std::shared_ptr<Value>> &
        ) override {
            return globals[index];
        }
    };
    struct Local : public LExpr {
        std::size_t index;
        Local(std::size_t index): index(index) {}
        std::shared_ptr<Value> &lvalue(
            std::vector<std::shared_ptr<Value>> &,
            std::vector<std::shared_ptr<Value>> &locals
        ) override {
            return locals[index];
        }
    };
    struct Subst : public RExpr {
        std::unique_ptr<LExpr> left;
        std::unique_ptr<RExpr> right;
        Subst(std::unique_ptr<LExpr> left, std::unique_ptr<RExpr> right):
            left(std::move(left)),
            right(std::move(right)) {}
        std::shared_ptr<Value> rvalue(
            std::vector<std::shared_ptr<Value>> &globals,
            std::vector<std::shared_ptr<Value>> &locals
        ) override {
            auto val = right->rvalue(globals, locals)->copy();
            left->lvalue(globals, locals) = val;
            return val;
        }
    };
    struct Call : public RExpr {
        std::shared_ptr<Func> func;
        std::vector<std::unique_ptr<RExpr>> args;
        Call(std::shared_ptr<Func> func, std::vector<std::unique_ptr<RExpr>> args):
            func(func),
            args(std::move(args)) {}
        std::shared_ptr<Value> rvalue(
            std::vector<std::shared_ptr<Value>> &globals,
            std::vector<std::shared_ptr<Value>> &locals
        ) override {
            std::size_t num_args = args.size();
            std::vector<std::shared_ptr<Value>> vals(num_args);
            for(std::size_t i = 0; i < num_args; i++){
                vals[i] = args[i]->rvalue(globals, locals);
            }
            return func->invoke(globals, vals);
        }
    };
    struct Term {
        std::vector<RExpr> stmts;
        virtual ~Term() = default;
        virtual std::size_t eval(
            std::vector<std::shared_ptr<Value>> &,
            std::vector<std::shared_ptr<Value>> &,
            std::shared_ptr<Value> &
        ) = 0;
    };
    struct Ret : Term {
        std::unique_ptr<RExpr> expr;
        Ret(std::unique_ptr<RExpr> expr):
            expr(std::move(expr)) {}
        std::size_t eval(
            std::vector<std::shared_ptr<Value>> &globals,
            std::vector<std::shared_ptr<Value>> &locals,
            std::shared_ptr<Value> &ret
        ) override {
            ret = expr->rvalue(globals, locals)->copy();
            return 0;
        }
    };
    struct Jmp : Term {
        std::size_t index;
        Jmp(std::size_t index): index(index) {}
        std::size_t eval(
            std::vector<std::shared_ptr<Value>> &,
            std::vector<std::shared_ptr<Value>> &,
            std::shared_ptr<Value> &
        ) override {
            return index;
        }
    };
    struct Br : Term {
        std::unique_ptr<RExpr> expr;
        std::size_t index_true, index_false;
        Br(std::unique_ptr<RExpr> expr, std::size_t index_true, std::size_t index_false):
            expr(std::move(expr)),
            index_true(index_true),
            index_false(index_false) {}
        std::size_t eval(
            std::vector<std::shared_ptr<Value>> &globals,
            std::vector<std::shared_ptr<Value>> &locals,
            std::shared_ptr<Value> &
        ) override {
            if(static_cast<Bool &>(*expr->rvalue(globals, locals)).value){
                return index_true;
            }else{
                return index_false;
            }
        }
    };
    struct IAdd : public Func {
        std::shared_ptr<Value> invoke(
            std::vector<std::shared_ptr<Value>> &,
            std::vector<std::shared_ptr<Value>> &args
        ) override {
            return std::make_shared<Int>(
                dynamic_cast<Int &>(*args[0]).value
                + dynamic_cast<Int &>(*args[1]).value
            );
        }
    };
    struct IMul : public Func {
        std::shared_ptr<Value> invoke(
            std::vector<std::shared_ptr<Value>> &,
            std::vector<std::shared_ptr<Value>> &args
        ) override {
            return std::make_shared<Int>(
                dynamic_cast<Int &>(*args[0]).value
                * dynamic_cast<Int &>(*args[1]).value
            );
        }
    };
    struct IEq : public Func {
        std::shared_ptr<Value> invoke(
            std::vector<std::shared_ptr<Value>> &,
            std::vector<std::shared_ptr<Value>> &args
        ) override {
            return std::make_shared<Bool>(
                dynamic_cast<Int &>(*args[0]).value
                == dynamic_cast<Int &>(*args[1]).value
            );
        }
    };
    struct Def : public Func {
        std::size_t num_local;
        std::vector<std::pair<std::vector<std::unique_ptr<RExpr>>, std::unique_ptr<Term>>> blocks;
        std::shared_ptr<Value> invoke(
            std::vector<std::shared_ptr<Value>> &globals,
            std::vector<std::shared_ptr<Value>> &args
        ) override {
            std::size_t index = 0;
            std::shared_ptr<Value> ret;
            args.resize(num_local);
            while(!ret){
                auto &[stmts, term] = blocks[index];
                for(auto &stmt : stmts) stmt->rvalue(globals, args);
                index = term->eval(globals, args, ret);
            }
            return ret;
        }
    };
}

int main(){
    std::vector<std::shared_ptr<ir::Value>> globals(10);
    auto print_globals = [&]{
        for(std::size_t i = 0; i < globals.size(); i++){
            if(globals[i]){
                std::cout << "#" << i << ":" << std::endl;
                globals[i]->debug_print();
            }
        }
        std::cout << std::endl;
    };

    std::make_unique<ir::Subst>(
        std::make_unique<ir::Global>(0),
        std::make_unique<ir::Subst>(
            std::make_unique<ir::Global>(1),
            std::make_unique<ir::Imm>(
                std::make_shared<ir::Int>(42)
            )
        )
    )->exec(globals);
    std::make_unique<ir::Subst>(
        std::make_unique<ir::Global>(0),
        std::make_unique<ir::Call>(
            std::make_shared<ir::IAdd>(),
            []{
                std::vector<std::unique_ptr<ir::RExpr>> args;
                args.push_back(std::make_unique<ir::Global>(0));
                args.push_back(std::make_unique<ir::Global>(1));
                return args;
            }()
        )
    )->exec(globals);
    std::make_unique<ir::Subst>(
        std::make_unique<ir::Global>(2),
        std::make_unique<ir::Call>(
            std::make_shared<ir::IEq>(),
            []{
                std::vector<std::unique_ptr<ir::RExpr>> args;
                args.push_back(std::make_unique<ir::Global>(0));
                args.push_back(std::make_unique<ir::Imm>(
                    std::make_shared<ir::Int>(84)
                ));
                return args;
            }()
        )
    )->exec(globals);
    print_globals();

    /*
     * fact_loop(n: int): int {
     *   int ret;
     *   ret = 1;
     * loop:
     *   ret *= n;
     *   n -= 1;
     *   goto n == 0 ? end : loop;
     * end:
     *   return ret;
     * }
     */
    auto fact_loop = std::make_shared<ir::Def>();
    fact_loop->blocks.emplace_back(
        []{
            std::vector<std::unique_ptr<ir::RExpr>> stmts;
            stmts.push_back(std::make_unique<ir::Subst>(
                std::make_unique<ir::Local>(1),
                std::make_unique<ir::Imm>(
                    std::make_shared<ir::Int>(1)
                )
            ));
            return stmts;
        }(),
        std::make_unique<ir::Jmp>(1)
    );
    fact_loop->blocks.emplace_back(
        []{
            std::vector<std::unique_ptr<ir::RExpr>> stmts;
            stmts.push_back(std::make_unique<ir::Subst>(
                std::make_unique<ir::Local>(1),
                std::make_unique<ir::Call>(
                    std::make_shared<ir::IMul>(),
                    []{
                        std::vector<std::unique_ptr<ir::RExpr>> args;
                        args.push_back(std::make_unique<ir::Local>(0));
                        args.push_back(std::make_unique<ir::Local>(1));
                        return args;
                    }()
                )
            ));
            stmts.push_back(std::make_unique<ir::Subst>(
                std::make_unique<ir::Local>(0),
                std::make_unique<ir::Call>(
                    std::make_shared<ir::IAdd>(),
                    []{
                        std::vector<std::unique_ptr<ir::RExpr>> args;
                        args.push_back(std::make_unique<ir::Local>(0));
                        args.push_back(std::make_unique<ir::Imm>(
                            std::make_shared<ir::Int>(-1)
                        ));
                        return args;
                    }()
                )
            ));
            return stmts;
        }(),
        std::make_unique<ir::Br>(
            std::make_unique<ir::Call>(
                std::make_shared<ir::IEq>(),
                []{
                    std::vector<std::unique_ptr<ir::RExpr>> args;
                    args.push_back(std::make_unique<ir::Local>(0));
                    args.push_back(std::make_unique<ir::Imm>(
                        std::make_shared<ir::Int>(0)
                    ));
                    return args;
                }()
            ),
            2, 1
        )
    );
    fact_loop->blocks.emplace_back(
        std::vector<std::unique_ptr<ir::RExpr>>(),
        std::make_unique<ir::Ret>(
            std::make_unique<ir::Local>(1)
        )
    );
    fact_loop->num_local = 2;
    std::make_unique<ir::Subst>(
        std::make_unique<ir::Global>(0),
        std::make_unique<ir::Call>(
            fact_loop,
            []{
                std::vector<std::unique_ptr<ir::RExpr>> args;
                args.push_back(std::make_unique<ir::Imm>(
                    std::make_shared<ir::Int>(5)
                ));
                return args;
            }()
        )
    )->exec(globals);
    print_globals();
    /*
     * fact_rec(n: int): int {
     *   if(n == 0){
     *     return 1;
     *   }else{
     *     return n * fact_rec(n - 1);
     *   }
     * }
     */
    auto fact_rec = std::make_shared<ir::Def>();
    fact_rec->blocks.emplace_back(
        std::vector<std::unique_ptr<ir::RExpr>>(),
        std::make_unique<ir::Br>(
            std::make_unique<ir::Call>(
                std::make_shared<ir::IEq>(),
                []{
                    std::vector<std::unique_ptr<ir::RExpr>> args;
                    args.push_back(std::make_unique<ir::Local>(0));
                    args.push_back(std::make_unique<ir::Imm>(
                        std::make_shared<ir::Int>(0)
                    ));
                    return args;
                }()
            ),
            1, 2
        )
    );
    fact_rec->blocks.emplace_back(
        std::vector<std::unique_ptr<ir::RExpr>>(),
        std::make_unique<ir::Ret>(
            std::make_unique<ir::Imm>(
                std::make_shared<ir::Int>(1)
            )
        )
    );
    fact_rec->blocks.emplace_back(
        std::vector<std::unique_ptr<ir::RExpr>>(),
        std::make_unique<ir::Ret>(
            std::make_unique<ir::Call>(
                std::make_shared<ir::IMul>(),
                [=]{
                    std::vector<std::unique_ptr<ir::RExpr>> args;
                    args.push_back(std::make_unique<ir::Local>(0));
                    args.push_back(std::make_unique<ir::Call>(
                        fact_rec,
                        []{
                            std::vector<std::unique_ptr<ir::RExpr>> args;
                            args.push_back(std::make_unique<ir::Call>(
                                std::make_shared<ir::IAdd>(),
                                []{
                                    std::vector<std::unique_ptr<ir::RExpr>> args;
                                    args.push_back(std::make_unique<ir::Local>(0));
                                    args.push_back(std::make_unique<ir::Imm>(
                                        std::make_shared<ir::Int>(-1)
                                    ));
                                    return args;
                                }()
                            ));
                            return args;
                        }()
                    ));
                    return args;
                }()
            )
        )
    );
    fact_rec->num_local = 1;
    std::make_unique<ir::Subst>(
        std::make_unique<ir::Global>(1),
        std::make_unique<ir::Call>(
            fact_rec,
            []{
                std::vector<std::unique_ptr<ir::RExpr>> args;
                args.push_back(std::make_unique<ir::Imm>(
                    std::make_shared<ir::Int>(5)
                ));
                return args;
            }()
        )
    )->exec(globals);
    print_globals();
}
