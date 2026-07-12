controller('axone1gel7g6wzjyt9zpz6r3ea3uewlfljl0uyel84c5').

decide(Case, Verdict) :-
    decide(Case, Verdict, _).

decide(
    ctx{
        intent: 'gov:revise_constitution',
        'gov:proposed_constitution_sha256': _,
        'gov:current_constitution_sha256': _,
        'gov:current_constitution_revision': _,
        'gov:module': _,
        'cw:tx': tx{
            message: msg{sender: Sender, funds: _},
            block: _
        }
    },
    'gov:permitted',
    'controller may revise constitution'
) :-
    controller(Sender).

decide(
    ctx{
        intent: 'gov:revise_constitution',
        'gov:proposed_constitution_sha256': _,
        'gov:current_constitution_sha256': _,
        'gov:current_constitution_revision': _,
        'gov:module': _,
        'cw:tx': tx{
            message: msg{sender: _, funds: _},
            block: _
        }
    },
    'gov:forbidden',
    'only controller may revise constitution'
).

decide(_, 'gov:permitted', 'permitted by default').
