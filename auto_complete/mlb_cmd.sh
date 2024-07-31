function read_file {
    readarray -t key_array < "/home/joe/RustroverProjects/mlb/auto_complete/$1.txt"
    keys=" ${key_array[*]} "
}

function mlb_auto_complete {
    if [[ $COMP_CWORD = 1 ]]; then
        reply="games results schedule stats teams leaders league-batting-stats league-pitching-stats update"
    elif [[ $COMP_CWORD = 2 ]]; then
        case ${COMP_WORDS[1]} in
            "g" | "games" | "r" | "results" | "u" | "schedule" | "t" | "teams")
                read_file "teams"
                reply=$keys;;
            "s" | "stats")
                read_file "players"
                reply=$keys;;
            "l" | "leaders")
                reply="b p avg hr rbi h sb wins era saves so whip all";;
            "b" | "league-batting-stats" | "p" | "league-pitching-stats")
                reply="all-time";;
            "update")
                reply="players teams";;
        esac
    elif [[ $COMP_CWORD = 3 ]]; then
        query_type=${COMP_WORDS[1]}
        input=" ${COMP_WORDS[2]} "

        if [[ $query_type == "s" || $query_type == "stats" ]]; then
            read_file "players"
            if [[ $keys =~ $input ]]; then
                reply="career yearByYear season"
            fi
        elif [[ $query_type == "t" || $query_type == "teams" ]]; then
            read_file "teams"
            if [[ $keys =~ $input ]]; then
                reply="hitting pitching"
            fi
        fi
    fi

    COMPREPLY=($(compgen -W "$reply" -- "${COMP_WORDS[COMP_CWORD]}"))
}

function mlb {
    cargo run --manifest-path /home/joe/RustroverProjects/mlb/Cargo.toml --release "$@"
}

complete -F mlb_auto_complete mlb
