describe "seq"

_test() {
    cmd=$1
    shift
    test "$(./rustybox $cmd "$@")" == "$($cmd "$@")"
}

it_seq_normal1() {
    _test seq 5
}

it_seq_normal2() {
    _test seq 6 31
}

it_seq_normal3() {
    _test seq 1 3 8
}

it_seq_normal4() {
    _test seq -5 1.5 8
}

it_seq_fail1() {
    ! ./rustybox seq
}

it_seq_fail2() {
    ! ./rustybox seq 1 2 3 4
}

it_seq_fail3() {
    ! ./rustybox seq -1 -1 1
}

it_seq_fail4() {
    ! ./rustybox seq -1 0 1
}
