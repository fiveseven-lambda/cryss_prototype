#include <iostream>
#include <sstream>
#include <string_view>
#include <memory>
#include <vector>
#include <utility>
#include <unordered_map>

struct indent {
    int depth;
public:
    indent(int depth): depth(depth) {}
    friend std::ostream &operator<<(std::ostream &os, const indent &idt) {
        for(int i = 0; i < idt.depth; i++) os << "  ";
        return os;
    }
};
struct Node {
    virtual ~Node() = default;
    void debug_print(){
        std::unordered_map<const Node *, std::size_t> labels;
        labels[nullptr] = 0;
        labels[this] = 1;
        debug_print_impl(labels);
    }
    virtual void debug_print_impl(std::unordered_map<const Node *, std::size_t> &) const = 0;
};
struct Stmt {
    virtual ~Stmt() = default;
    virtual void debug_print(int) const = 0;
    virtual std::shared_ptr<Node> translate(std::shared_ptr<Node>) = 0;
};
struct Expr : public Stmt {
    std::string expr;
    Expr(std::string expr):
        expr(expr) {}
    void debug_print(int depth) const override {
        std::cout << indent(depth) << expr << std::endl;
    }
    std::shared_ptr<Node> translate(std::shared_ptr<Node>) override;
};
struct If: public Stmt {
    std::string cond;
    std::unique_ptr<Stmt> stmt_true, stmt_false;
    If(std::string cond, std::unique_ptr<Stmt> stmt_true, std::unique_ptr<Stmt> stmt_false = nullptr):
        cond(cond),
        stmt_true(std::move(stmt_true)),
        stmt_false(std::move(stmt_false)) {}
    void debug_print(int depth) const override {
        std::cout << indent(depth) << "if " << cond << " then" << std::endl;
        if(stmt_true) stmt_true->debug_print(depth + 1);
        std::cout << indent(depth) << "else" << std::endl;
        if(stmt_false) stmt_false->debug_print(depth + 1);
        std::cout << indent(depth) << "endif" << std::endl;
    }
    std::shared_ptr<Node> translate(std::shared_ptr<Node>) override;
};
struct Seq: public Stmt {
    std::unique_ptr<Stmt> first, second;
    Seq(std::unique_ptr<Stmt> first, std::unique_ptr<Stmt> second):
        first(std::move(first)),
        second(std::move(second)) {}
    void debug_print(int depth) const override {
        first->debug_print(depth);
        second->debug_print(depth);
    }
    std::shared_ptr<Node> translate(std::shared_ptr<Node>) override;
};


struct ExprNode : public Node {
    std::string expr;
    std::shared_ptr<Node> next;
    ExprNode(std::string expr, std::shared_ptr<Node> next):
        expr(std::move(expr)),
        next(std::move(next)) {}
    void debug_print_impl(std::unordered_map<const Node *, std::size_t> &labels) const override {
        std::cout << labels[this] << ": " << expr << ",";
        auto it = labels.find(next.get());
        bool call_next = false;
        if(it == labels.end()){
            std::size_t label = labels.size();
            it = labels.emplace(next.get(), label).first;
            call_next = true;
        }
        std::cout << it->second << std::endl;
        if(call_next) next->debug_print_impl(labels);
    }
};
struct BrNode : public Node {
    std::string cond;
    std::shared_ptr<Node> next_true, next_false;
    BrNode(std::string cond, std::shared_ptr<Node> next_true, std::shared_ptr<Node> next_false):
        cond(std::move(cond)),
        next_true(std::move(next_true)),
        next_false(std::move(next_false)) {}
    void debug_print_impl(std::unordered_map<const Node *, std::size_t> &labels) const override {
        std::cout << labels[this] << ": Br " << cond << ",";
        auto it_true = labels.find(next_true.get());
        bool call_true = false;
        if(it_true == labels.end()){
            std::size_t label = labels.size();
            it_true = labels.emplace(next_true.get(), label).first;
            call_true = true;
        }
        std::cout << it_true->second << ",";
        auto it_false = labels.find(next_false.get());
        bool call_false = false;
        if(it_false == labels.end()){
            std::size_t label = labels.size();
            it_false = labels.emplace(next_false.get(), label).first;
            call_false = true;
        }
        std::cout << it_false->second << std::endl;
        if(call_true) next_true->debug_print_impl(labels);
        if(call_false) next_false->debug_print_impl(labels);
    }
};

std::shared_ptr<Node> Expr::translate(std::shared_ptr<Node> end){
    return std::make_shared<ExprNode>(expr, end);
}
std::shared_ptr<Node> If::translate(std::shared_ptr<Node> end){
    auto next_true = stmt_true->translate(end);
    auto next_false = stmt_false ? stmt_false->translate(end) : end;
    return std::make_shared<BrNode>(cond, next_true, next_false);
}
std::shared_ptr<Node> Seq::translate(std::shared_ptr<Node> end){
    return first->translate(second->translate(end));
}

int main(){
    int expr_num = 0;
    auto expr = [&expr_num]{
        std::stringstream ss;
        ss << '#' << expr_num++;
        return ss.str();
    };
    auto stmt = std::make_unique<Seq>(
        std::make_unique<Expr>(expr()),
    std::make_unique<Seq>(
        std::make_unique<Expr>(expr()),
    std::make_unique<Seq>(
        std::make_unique<If>(
            expr(),
            std::make_unique<Expr>(expr())
        ),
    std::make_unique<Seq>(
        std::make_unique<If>(
            expr(),
            std::make_unique<Expr>(expr()),
            std::make_unique<Expr>(expr())
        ),
    std::make_unique<Seq>(
        std::make_unique<If>(
            expr(),
            std::make_unique<Seq>(
                std::make_unique<Expr>(expr()),
                std::make_unique<Expr>(expr())
            ),
            std::make_unique<Seq>(
                std::make_unique<Expr>(expr()),
                std::make_unique<Expr>(expr())
            )
        ),
        std::make_unique<If>(
            expr(),
            std::make_unique<If>(
                expr(),
                std::make_unique<Expr>(expr()),
                std::make_unique<Expr>(expr())
            ),
            std::make_unique<Expr>(expr())
        )
    )))));
    stmt->debug_print(0);
    stmt->translate(nullptr)->debug_print();
}
