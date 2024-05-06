fold_left(Goal, [H|T], Result) :-
    fold_left(Goal, T, H, Result).

fold_left(Goal, [H|T], Acc, Result) :-
    call(Goal, Acc, H, NewAcc),
    fold_left(Goal, T, NewAcc, Result).

fold_left(_Goal, [], Result, Result).

% Concatenate all the elements in the list using the atom_concat predicate.
atoms_concat(List, Result) :-
    fold_left(atom_concat, List, Result).

% Forge a cosmwasm URI that can be handled by the logic module. Take care of the query json serialization & url encoding.
cosmwasm_query(ContractName, ContractAddr, Query, DecodeBase64, URI) :-
    json_prolog(RawQuery, Query),
    uri_encoded(query, RawQuery, EncodedQuery),
    atoms_concat([
        'cosmwasm:',
        ContractName,
        ':',
        ContractAddr,
        '?query=',
        EncodedQuery,
        '&base64Decode=',
        DecodeBase64
    ], URI).

% Execute the cosmwasm query by opening the URI stream relying on the logic module interpreter virtual filesystem.
% It then consumes the stream unifying it with its string reprensation, close the stream and make the json conversion of
% the contract response.
cosmwasm_call(URI, Response) :-
    open(URI, 'read', Stream),
    read_string(Stream, _, Raw),
    close(Stream),
    json_prolog(Raw, Response).

% Represents the cognitarium Select query input.
cognitarium_select(Prefixes, Select, Where, Limit, Query) :-
    Query = json([
        select-json([
            query-json([
                prefixes-Prefixes,
                select-Select,
                where-Where,
                limit-Limit
            ])
        ])
    ]).

% Extract the bindings field of a cognitarium Select query response.
cognitarium_select_bindings(SelectResponse, Bindings) :-
    SelectResponse = json([head-_,results-json([bindings-Bindings])]).

% Extract the tag variable value from a single Select response binding.
cognitarium_extract_binding_tag(Binding, Tag) :-
    Binding = json([tag-json([datatype- @(null),type-literal,value-Tag,'xml:lang'- @(null)])]).

% Given a cognitarium address and a dataset identifier, resolves the tags contained in any metadata of the type GeneralMetadata.
cognitarium_dataset_tags(CognitariumAddr, DatasetDID, Tags) :-
    cognitarium_select(
        [
            json([prefix-'rdf', namespace-'http://www.w3.org/1999/02/22-rdf-syntax-ns#']),
            json([prefix-'core', namespace-'https://ontology.axone.space/core/']),
            json([prefix-'meta', namespace-'https://ontology.axone.space/metadata/dataset/'])
        ],
        [
            json([variable-'tag'])
        ],
        [
            json([
                simple-json([
                    triple_pattern-json([
                        subject-json([variable-'meta']),
                        predicate-json([node-json([named_node-json([prefixed-'core:describes'])])]),
                        object-json([node-json([named_node-json([full-DatasetDID])])])
                    ])
                ])
            ]),
            json([
                simple-json([
                    triple_pattern-json([
                        subject-json([variable-'meta']),
                        predicate-json([node-json([named_node-json([prefixed-'rdf:type'])])]),
                        object-json([node-json([named_node-json([prefixed-'meta:GeneralMetadata'])])])
                    ])
                ])
            ]),
            json([
                simple-json([
                    triple_pattern-json([
                        subject-json([variable-'meta']),
                        predicate-json([node-json([named_node-json([prefixed-'core:hasTag'])])]),
                        object-json([variable-'tag'])
                    ])
                ])
            ])
        ],
        @(null),
        Query
    ),
    cosmwasm_query(cognitarium, CognitariumAddr, Query, false, URI),
    cosmwasm_call(URI, Response),
    cognitarium_select_bindings(Response, Bindings),
    maplist(cognitarium_extract_binding_tag, Bindings, Tags).

% True if a given dataset identifier has the given tag through a GeneralMetadata in the provided cognitarium address.
cognitarium_dataset_has_tag(CognitariumAddr, DatasetDID, Tag) :-
    cognitarium_dataset_tags(CognitariumAddr, DatasetDID, Tags),
    member(Tag, Tags).
