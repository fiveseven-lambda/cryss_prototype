#include <iostream>
#include "ir.hpp"

int main(){
    auto assign = std::make_shared<ir::Assign>();
    auto deref = std::make_shared<ir::Deref>();
    auto iadd = std::make_shared<ir::IAdd>();
    auto imul = std::make_shared<ir::IMul>();
    auto ieq = std::make_shared<ir::IEq>();

    auto add_one_to_arg = std::make_shared<ir::FuncDef>();
    add_one_to_arg->num_locals = 1;
    add_one_to_arg->stmts.push_back(ir::ExprStmt(
        std::make_unique<ir::Call>(
            std::make_unique<ir::Imm>(assign),
            [&]{
                std::vector<std::unique_ptr<ir::Expr>> tmp;
                tmp.push_back(std::make_unique<ir::Call>(
                    std::make_unique<ir::Imm>(deref),
                    [&]{
                        std::vector<std::unique_ptr<ir::Expr>> tmp2;
                        tmp2.push_back(std::make_unique<ir::Local>(0));
                        return tmp2;
                    }()
                ));
                tmp.push_back(std::make_unique<ir::Call>(
                    std::make_unique<ir::Imm>(iadd),
                    [&]{
                        std::vector<std::unique_ptr<ir::Expr>> tmp2;
                        tmp2.push_back(std::make_unique<ir::Call>(
                            std::make_unique<ir::Imm>(deref),
                            [&]{
                                std::vector<std::unique_ptr<ir::Expr>> tmp3;
                                tmp3.push_back(std::make_unique<ir::Call>(
                                    std::make_unique<ir::Imm>(deref),
                                    [&]{
                                        std::vector<std::unique_ptr<ir::Expr>> tmp4;
                                        tmp4.push_back(std::make_unique<ir::Local>(0));
                                        return tmp4;
                                    }()
                                ));
                                return tmp3;
                            }()
                        ));
                        tmp2.push_back(std::make_unique<ir::Imm>(1));
                        return tmp2;
                    }()
                ));
                return tmp;
            }()
        ),
        1
    ));

    auto add_one = std::make_shared<ir::FuncDef>();
    add_one->num_locals = 1;
    add_one->stmts.push_back(ir::ExprStmt(
        std::make_unique<ir::Call>(
            std::make_unique<ir::Imm>(add_one_to_arg),
            [&]{
                std::vector<std::unique_ptr<ir::Expr>> tmp;
                tmp.push_back(std::make_unique<ir::Local>(0));
                return tmp;
            }()
        ),
        1
    ));
    add_one->stmts.push_back(ir::ExprStmt(
        std::make_unique<ir::Call>(
            std::make_unique<ir::Imm>(deref),
            [&]{
                std::vector<std::unique_ptr<ir::Expr>> tmp;
                tmp.push_back(std::make_unique<ir::Local>(0));
                return tmp;
            }()
        ),
        2
    ));

    auto fact_loop = std::make_shared<ir::FuncDef>();
    fact_loop->num_locals = 2;
    fact_loop->stmts.push_back(ir::ExprStmt(
        std::make_unique<ir::Call>(
            std::make_unique<ir::Imm>(assign),
            [&]{
                std::vector<std::unique_ptr<ir::Expr>> tmp;
                tmp.push_back(std::make_unique<ir::Local>(1));
                tmp.push_back(std::make_unique<ir::Imm>(1));
                return tmp;
            }()
        ),
        1
    ));
    fact_loop->stmts.push_back(ir::ExprStmt(
        std::make_unique<ir::Call>(
            std::make_unique<ir::Imm>(assign),
            [&]{
                std::vector<std::unique_ptr<ir::Expr>> tmp;
                tmp.push_back(std::make_unique<ir::Local>(1));
                tmp.push_back(std::make_unique<ir::Call>(
                    std::make_unique<ir::Imm>(imul),
                    [&]{
                        std::vector<std::unique_ptr<ir::Expr>> tmp2;
                        tmp2.push_back(std::make_unique<ir::Call>(
                            std::make_unique<ir::Imm>(deref),
                            [&]{
                                std::vector<std::unique_ptr<ir::Expr>> tmp3;
                                tmp3.push_back(std::make_unique<ir::Local>(1));
                                return tmp3;
                            }()
                        ));
                        tmp2.push_back(std::make_unique<ir::Call>(
                            std::make_unique<ir::Imm>(deref),
                            [&]{
                                std::vector<std::unique_ptr<ir::Expr>> tmp3;
                                tmp3.push_back(std::make_unique<ir::Local>(0));
                                return tmp3;
                            }()
                        ));
                        return tmp2;
                    }()
                ));
                return tmp;
            }()
        ),
        2
    ));
    fact_loop->stmts.push_back(ir::ExprStmt(
        std::make_unique<ir::Call>(
            std::make_unique<ir::Imm>(assign),
            [&]{
                std::vector<std::unique_ptr<ir::Expr>> tmp;
                tmp.push_back(std::make_unique<ir::Local>(0));
                tmp.push_back(std::make_unique<ir::Call>(
                    std::make_unique<ir::Imm>(iadd),
                    [&]{
                        std::vector<std::unique_ptr<ir::Expr>> tmp2;
                        tmp2.push_back(std::make_unique<ir::Call>(
                            std::make_unique<ir::Imm>(deref),
                            [&]{
                                std::vector<std::unique_ptr<ir::Expr>> tmp3;
                                tmp3.push_back(std::make_unique<ir::Local>(0));
                                return tmp3;
                            }()
                        ));
                        tmp2.push_back(std::make_unique<ir::Imm>(-1));
                        return tmp2;
                    }()
                ));
                return tmp;
            }()
        ),
        3
    ));
    fact_loop->stmts.push_back(ir::BrStmt(
        std::make_unique<ir::Call>(
            std::make_unique<ir::Imm>(ieq),
            [&]{
                std::vector<std::unique_ptr<ir::Expr>> tmp;
                tmp.push_back(std::make_unique<ir::Call>(
                    std::make_unique<ir::Imm>(deref),
                    [&]{
                        std::vector<std::unique_ptr<ir::Expr>> tmp2;
                        tmp2.push_back(std::make_unique<ir::Local>(0));
                        return tmp2;
                    }()
                ));
                tmp.push_back(std::make_unique<ir::Imm>(0));
                return tmp;
            }()
        ),
        4,
        1
    ));
    fact_loop->stmts.push_back(ir::ExprStmt(
        std::make_unique<ir::Call>(
            std::make_unique<ir::Imm>(deref),
            [&]{
                std::vector<std::unique_ptr<ir::Expr>> tmp;
                tmp.push_back(std::make_unique<ir::Local>(1));
                return tmp;
            }()
        ),
        5
    ));

    auto fact_rec = std::make_shared<ir::FuncDef>();
    fact_rec->num_locals = 1;
    fact_rec->stmts.push_back(ir::BrStmt(
        std::make_unique<ir::Call>(
            std::make_unique<ir::Imm>(ieq),
            [&]{
                std::vector<std::unique_ptr<ir::Expr>> tmp;
                tmp.push_back(std::make_unique<ir::Call>(
                    std::make_unique<ir::Imm>(deref),
                    [&]{
                        std::vector<std::unique_ptr<ir::Expr>> tmp2;
                        tmp2.push_back(std::make_unique<ir::Local>(0));
                        return tmp2;
                    }()
                ));
                tmp.push_back(std::make_unique<ir::Imm>(0));
                return tmp;
            }()
        ),
        1,
        2
    ));
    fact_rec->stmts.push_back(ir::ExprStmt(
        std::make_unique<ir::Imm>(1),
        3
    ));
    fact_rec->stmts.push_back(ir::ExprStmt(
        std::make_unique<ir::Call>(
            std::make_unique<ir::Imm>(imul),
            [&]{
                std::vector<std::unique_ptr<ir::Expr>> tmp;
                tmp.push_back(std::make_unique<ir::Call>(
                    std::make_unique<ir::Imm>(deref),
                    [&]{
                        std::vector<std::unique_ptr<ir::Expr>> tmp2;
                        tmp2.push_back(std::make_unique<ir::Local>(0));
                        return tmp2;
                    }()
                ));
                tmp.push_back(std::make_unique<ir::Call>(
                    std::make_unique<ir::Imm>(fact_rec),
                    [&]{
                        std::vector<std::unique_ptr<ir::Expr>> tmp2;
                        tmp2.push_back(std::make_unique<ir::Call>(
                            std::make_unique<ir::Imm>(iadd),
                            [&]{
                                std::vector<std::unique_ptr<ir::Expr>> tmp3;
                                tmp3.push_back(std::make_unique<ir::Call>(
                                    std::make_unique<ir::Imm>(deref),
                                    [&]{
                                        std::vector<std::unique_ptr<ir::Expr>> tmp4;
                                        tmp4.push_back(std::make_unique<ir::Local>(0));
                                        return tmp4;
                                    }()
                                ));
                                tmp3.push_back(std::make_unique<ir::Imm>(-1));
                                return tmp3;
                            }()
                        ));
                        return tmp2;
                    }()
                ));
                return tmp;
            }()
        ),
        3
    ));

    ir::Env env;
    std::cout << std::get<ir::Int>(add_one->invoke(env, {ir::Int(1)})) << std::endl;
    std::cout << std::get<ir::Int>(fact_loop->invoke(env, {ir::Int(10)})) << std::endl;
    std::cout << std::get<ir::Int>(fact_rec->invoke(env, {ir::Int(10)})) << std::endl;
}
