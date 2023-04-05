:- consult('cosmwasm:okp4-objectarium:${STORAGE_ADDRESS}?query=%7B%22object_data%22%3A%7B%22id%22%3A%221a88ca1632c7323c0aa594000cda26ed9f48b36351c29c3d1e35e0a0474e862e%22%7D%7D').

admin_addr('okp41p8u47en82gmzfm259y6z93r9qe63l25dfwwng6').

allow_denom('uknow').
allow_did_method('key').
allow_addr(Addr) :- bech32_address(-('okp4', _), Addr).

min_exec_workflow_amount(1000000).
min_create_dataset_amount(10000).
min_create_service_amount(100000).
