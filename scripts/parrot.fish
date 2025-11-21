function parrot
    # If the first argument is "replay", use the parrot_replay function
    if test (count $argv) -ge 1 -a "$argv[1]" = "replay"
        # Forward all remaining arguments to parrot_replay
        _parrot_replay $argv[2..-1]
    else
        # For any other subcommand, run the real parrot binary
        command parrot $argv
    end
end

function _parrot_replay
    set cmds (command parrot replay $argv | string split \n)
    for cmd in $cmds
        echo "> $cmd"
        eval $cmd
    end
end
