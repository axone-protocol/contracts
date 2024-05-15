valid_did(DID, Addr) :-
    did_components(DID, did_components(Method, Addr, _, _, _)),
    allow_did_method(Method),
    allow_addr(Addr).

min_amount(exec_workflow, MinAmount) :-
    min_exec_workflow_amount(MinAmount).

min_amount(create_dataset, MinAmount) :-
    min_create_dataset_amount(MinAmount).

min_amount(create_service, MinAmount) :-
    min_create_service_amount(MinAmount).

has_sufficient_balance(Addr, MinAmount) :-
    bank_spendable_balances(Addr, Balances),
    member(Denom-Amount, Balances),
    allow_denom(Denom),
    Amount @>= MinAmount.

can(change_governance, DID) :-
    valid_did(DID, Addr),
    admin_addr(Addr).

can(Action, DID) :-
    valid_did(DID, Addr),
    min_amount(Action, MinAmount),
    has_sufficient_balance(Addr, MinAmount).
