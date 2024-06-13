#!/bin/bash -e

crate_name() {
    local cargo_toml=$1
    grep '^name =' "$cargo_toml" | awk -F '=' '{print $2}' | tr -d '[:space:]"'
}

crate_dependencies() {
    local cargo_toml=$1
    
    # Extract dependency names and dev dependency names from Cargo.toml
    awk '
        BEGIN { in_dependencies = 0; }
        /^\[dependencies\]$/ { in_dependencies = 1; next }
        /^\[dev-dependencies\]$/ { in_dependencies = 1; next }
        /^\[/ { in_dependencies = 0 }
        in_dependencies && /^[a-zA-Z0-9_-]+/ { gsub(/\.workspace/, "", $1); gsub(/[[:space:]]/, "", $1); dependencies[$1] }
        END {
            for (dependency in dependencies) print dependency;
        }
    ' "$cargo_toml" | sort | uniq | tr '\n' ' '
}

crate_dependency_graph=()

topological_sort_crates() {
    local sorted_nodes=()

    sort_crate_node() {
        local node=$1
        local edges=$2

        for edge in $edges; do
            if [[ ! "${sorted_nodes[*]}" =~ $edge ]]; then
                for crate_dependencies_pair in "${crate_dependency_graph[@]}"; do
                    if [[ "${crate_dependencies_pair%%:*}" == "$edge" ]]; then
                        sort_crate_node "$edge" "${crate_dependencies_pair#*:}"
                        break
                    fi
                done
            fi
        done

        sorted_nodes+=("$node")
    }

    for node_graph in "${crate_dependency_graph[@]}"; do
        local node=${node_graph%%:*}
        local edges=${node_graph#*:}

        if [[ ! "${sorted_nodes[*]}" =~ $node ]]; then
            sort_crate_node "$node" "$edges"
        fi
    done

    echo "${sorted_nodes[@]}"
}

# Populate crate_dependency_graph
while IFS= read -r cargo_toml; do
    crate_dependency_graph+=("$(crate_name "$cargo_toml"):$(crate_dependencies "$cargo_toml")")
done < <(find . -type d -name target -prune -o \( -name Cargo.toml -type f ! -path './Cargo.toml' \) -print)

internal_crates_list=()
filtered_crate_dependency_graph=()

# Populate internal_crates_list
for crate_dependencies_pair in "${crate_dependency_graph[@]}"; do
    crate_name=${crate_dependencies_pair%%:*}
    internal_crates_list+=("$crate_name")
done

# Remove external dependencies
for crate_dependencies_pair in "${crate_dependency_graph[@]}"; do
    crate_name=${crate_dependencies_pair%%:*}
    read -r -a crate_dependencies <<< "${crate_dependencies_pair#*:}"
    crate_internal_dependencies=()
    

    for dependency in "${crate_dependencies[@]}"; do
        if [[ "${internal_crates_list[*]}" =~ $dependency ]]; then
            crate_internal_dependencies+=("$dependency")
        fi
    done

    filtered_crate_dependency_graph+=("$crate_name:${crate_internal_dependencies[*]}")
done

crate_dependency_graph=("${filtered_crate_dependency_graph[@]}")

for crate in $(topological_sort_crates); do
    cargo publish -n -p $crate
done