#include <cstddef>
#include <cstdint>
#include <variant>
#include <vector>
#include <memory>
#include <unordered_map>

namespace ir {
    using Int = std::int32_t;
    using Bool = bool;

    using StackId = std::size_t;

    struct GlobalAddr {
        std::size_t pos;
        GlobalAddr(std::size_t);
    };
    struct StackAddr {
        StackId stack_id;
        std::size_t pos;
        StackAddr(std::size_t, std::size_t);
    };

    using Addr = std::variant<GlobalAddr, StackAddr>;

    struct Func;

    using Value = std::variant<Int, Bool, Addr, std::shared_ptr<Func>>;

    struct Env {
        std::vector<Value> global;
        std::unordered_map<StackId, std::vector<Value>> stack;
        std::size_t num_stack;
        Env();
        Value &operator()(const GlobalAddr &);
        Value &operator()(const StackAddr &);
    };

    struct Func {
        virtual ~Func();
        virtual Value invoke(Env &, std::vector<Value>) const = 0;
    };

    struct Assign : public Func { Value invoke(Env &, std::vector<Value>) const override; };
    struct Deref : public Func { Value invoke(Env &, std::vector<Value>) const override; };
    struct IAdd : public Func { Value invoke(Env &, std::vector<Value>) const override; };
    struct IMul : public Func { Value invoke(Env &, std::vector<Value>) const override; };
    struct IEq : public Func { Value invoke(Env &, std::vector<Value>) const override; };

    struct Expr {
        virtual ~Expr();
        virtual Value eval(Env &, StackId) const = 0;
    };
    struct Imm : public Expr {
        Value value;
        Imm(Value);
        Value eval(Env &, StackId) const override;
    };
    struct Local : public Expr {
        std::size_t index;
        Local(std::size_t);
        Value eval(Env &, StackId) const override;
    };
    struct Call : public Expr {
        std::unique_ptr<Expr> func_expr;
        std::vector<std::unique_ptr<Expr>> args_expr;
        Call(std::unique_ptr<Expr>, std::vector<std::unique_ptr<Expr>>);
        Value eval(Env &, StackId) const override;
    };

    struct ExprStmt {
        std::unique_ptr<Expr> expr;
        std::size_t next;
        ExprStmt(std::unique_ptr<Expr>, std::size_t);
        std::size_t run(Env &, Value &, StackId) const;
    };
    struct BrStmt {
        std::unique_ptr<Expr> cond;
        std::size_t next_true, next_false;
        BrStmt(std::unique_ptr<Expr>, std::size_t, std::size_t);
        std::size_t run(Env &, Value &, StackId) const;
    };
    using Stmt = std::variant<ExprStmt, BrStmt>;

    struct FuncDef : public Func {
        std::size_t num_locals;
        std::vector<Stmt> stmts;
        Value invoke(Env &, std::vector<Value>) const override;
    };
}
