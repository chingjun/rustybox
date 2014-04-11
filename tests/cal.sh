describe "seq"

_test() {
    cmd=$1
    shift
    test "$(./rustybox $cmd "$@")" == "$($cmd "$@")"
}

it_cal_normal() {
    _test cal
}

it_cal_february_leap_year() {
    _test cal 2 1956
}

it_cal_february_common_year() {
    _test cal 2 1957
}

it_cal_february_common_year_divisible_by_four() {
    _test cal 2 1900
}

it_cal_year() {
    # alignment of the year is different in rustybox and bsd cal
    test "$(./rustybox cal 1956 | sed 1d)" == "$(cal 1956 | sed 1d)"
}

it_cal_fail1() {
    ! ./rustybox cal 1 2 3
}
