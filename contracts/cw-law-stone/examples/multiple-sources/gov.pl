:- consult('cosmwasm:cw-storage:${STORAGE_ADDRESS}?query=%7B%22object_data%22%3A%7B%22id%22%3A%2220a9286b574c41af1d6742964fba0a161fb5c446b4720c2f928bdb33afcb2406%22%7D%7D').

admin_addr('okp41p8u47en82gmzfm259y6z93r9qe63l25dfwwng6').

allow_denom('uknow').
allow_did_method('key').
allow_addr(Addr) :- bech32_address(-('okp4', _), Addr).

min_exec_workflow_amount(1000000).
min_create_dataset_amount(10000).
min_create_service_amount(100000).
