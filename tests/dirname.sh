describe "dirname"

_test() {
    cmd=$1
    shift
    test "$(./rustybox $cmd "$@")" == "$($cmd "$@")"
}

it_dirname_normal1() {
    _test dirname $PWD
}

it_dirname_normal2() {
    _test dirname /
}

it_dirname_normal3() {
    _test dirname .
}

it_dirname_normal4() {
    _test dirname asd
}

it_dirname_normal5() {
    _test dirname asd/asd
}

it_dirname_normal5() {
    _test dirname ""
}

it_dirname_fail1() {
    ! ./rustybox dirname
}
