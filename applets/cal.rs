use getopts::{getopts,optflag,optopt};
use std;
use std::io::{println,print};
use common;
use time;

static WEEK_HEADER : &'static str = "Su Mo Tu We Th Fr Sa";
static MONTH_NAMES : [&'static str, ..13] = ["", "January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
static CUMULATIVE_DAYS : [int, ..14] = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];

pub fn main(args: &[~str]) {
    let opts = ~[
        optopt("m", "", "Display the specified month.", "month"),
        optflag("y", "", "Display a calendar for the specified year."),
    ];
    let usage = "cal [-y] [-m month] [[month] year]";
    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            common::err_write_line(f.to_err_msg());
            common::print_usage(usage, opts);
            std::os::set_exit_status(1);
            return;
        }
        Ok(m) => { m }
    };

    let (mut month_specified, mut month, year) = match matches.free.as_slice() {
        [ref m, ref y] => {
            let month = match convert_month(*m) {
                Some(m) => m,
                None => {
                    std::os::set_exit_status(1);
                    return;
                }
            };
            let year = match convert_year(*y) {
                Some(y) => y,
                None => {
                    std::os::set_exit_status(1);
                    return;
                }
            };
            (true, month, year)
        }
        [ref y] => {
            let year = match convert_year(*y) {
                Some(y) => y,
                None => {
                    std::os::set_exit_status(1);
                    return;
                }
            };
            (false, 1, year)
        }
        [] => {
            //current month, current year
            let tm = time::now();
            (true, (tm.tm_mon + 1) as int, (tm.tm_year + 1900) as int)
        }
        _ => {
            //show error
            common::print_usage(usage, opts);
            std::os::set_exit_status(1);
            return;
        }
    };

    if !month_specified && matches.opt_present("m") {
        month_specified = true;
        month = match convert_month(matches.opt_str("m").unwrap()) {
            Some(m) => m,
            None => {
                std::os::set_exit_status(1);
                return;
            }
        };
    }

    if month_specified {
        let month_grid = build_month(month, year, true);
        for i in month_grid.iter() {
            println(*i);
        }
    } else {
        println(build_year_title(year));
        println("");
        for i in range(0, 4) {
            let mut months = ~[];
            for j in range(1, 4) {
                let m = i*3 + j;
                months.push(build_month(m, year, false));
            }
            // print three months
            let row_len = WEEK_HEADER.len();
            for i in range(0, months[0].len()) {
                print(months[0][i]);
                print(" ".repeat(row_len - months[0][i].len() + 2));
                print(months[1][i]);
                print(" ".repeat(row_len - months[1][i].len() + 2));
                println(months[2][i]);
            }
        }
    }
}

fn convert_month(m: &str) -> Option<int> {
    match from_str::<int>(m) {
        Some(m) if m >= 1 && m <= 12 => Some(m),
        _ => {
            common::err_write_line(format!("cal: {:s} is neither a month number (1..12) nor a name", m));
            None
        }
    }
}

fn convert_year(y: &str) -> Option<int> {
    match from_str::<int>(y) {
        Some(y) if y >= 1 && y <= 9999 => Some(y),
        _ => {
            common::err_write_line(format!("cal: year {:s} not in range 1..9999", y));
            None
        }
    }
}

fn number_of_days_in_month(month: int, year: int) -> int {
    match month {
        1|3|5|7|8|10|12 => 31,
        2 if year%400 == 0 || (year%4 == 0 && year%100 != 0) => 29,
        2 => 28,
        _ => 30
    }
}

fn day_of_week(d: int, m: int, y: int) -> int {
    // number of days since year 1
    // gregorian calendar expected
    let days = (y-1) * 365 // all days for previous years
             + (y-1)/4 // plus leap days
             - (y-1)/100 // it is not a leap year if year is divisible by 100
             + (y-1)/400 // but it is a leap year if year is divisible by 400
             + CUMULATIVE_DAYS[m] // cumulative days for a given month
             + if (y%400 == 0 || (y%4 == 0 && y%100 != 0)) && m >= 2 {1} else {0} // adjustment for leap year
             + d;

    return days%7;
}

fn build_title(month: int, year: int) -> ~str {
    let title = if year > 0 {
        format!("{:s} {:d}", MONTH_NAMES[month], year)
    } else {
        format!("{:s}", MONTH_NAMES[month])
    };
    let prepend_len = (WEEK_HEADER.len() - title.len())/2;
    return " ".repeat(prepend_len) + title;
}

fn build_year_title(year: int) -> ~str {
    let title = format!("{:d}", year);
    let prepend_len = (WEEK_HEADER.len()*3 + 4 - title.len())/2;
    return " ".repeat(prepend_len) + title;
}

fn build_month(month: int, year: int, year_in_title: bool) -> ~[~str] {
    let first_day = day_of_week(1, month, year);
    let day_num = number_of_days_in_month(month, year);

    let mut out = ~[];

    out.push(build_title(month, if year_in_title { year } else { 0 }));
    out.push(WEEK_HEADER.to_owned());

    let mut s = ~"";
    let mut count = 0;
    for i in range(-first_day + 1, day_num+1) {
        if i <= 0 {
            s.push_str("  ");
        } else {
            s.push_str(format!("{:>2d}", i));
        }
        count += 1;
        if count%7 == 0 {
            out.push(s);
            s = ~"";
        } else if i != day_num {
            s.push_char(' ');
        }
    }
    out.push(s);
    while out.len() < 8 {
        out.push(~"");
    }
    return out;
}
