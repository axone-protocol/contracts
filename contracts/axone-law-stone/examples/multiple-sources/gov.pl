:- consult('cosmwasm:axone-objectarium:${STORAGE_ADDRESS}?query=%7B%22object_data%22%3A%7B%22id%22%3A%22b118d79b4a368028b34d564448e5f1082e098613434370f3c15d6a2bf9979dfc%22%7D%7D').

admin_addr('axone1p8u47en82gmzfm259y6z93r9qe63l25d858vqu').

allow_denom('uaxone').
allow_did_method('example').
allow_addr(Addr) :- bech32_address(-('axone', _), Addr).

min_exec_workflow_amount(1000000).
min_create_dataset_amount(10000).
min_create_service_amount(100000).
