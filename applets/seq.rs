use std;
use common;

fn err_not_float(arg: &str) {
    common::err_write_line(format!("seq: invalid floating point argument: {:s}", arg));
    std::os::set_exit_status(1);
}

pub fn main(args: &[~str]) {
    let (first, step, last) = match args {
        [_, ref last] => {
            match from_str::<f64>(*last) {
                Some(last_f) => (1.0, 1.0, last_f),
                _ => {
                    err_not_float(*last);
                    return;
                }
            }
        }
        [_, ref first, ref last] => {
            match (from_str::<f64>(*first), from_str::<f64>(*last)) {
                (Some(first_f), Some(last_f)) => if last_f >= first_f {
                    (first_f, 1.0, last_f)
                } else {
                    (first_f, -1.0, last_f)
                },
                (f, _) => {
                    err_not_float(*(if f.is_none() {first} else {last}));
                    return;
                }
            }
        }
        [_, ref first, ref step, ref last] => {
            match (from_str::<f64>(*first), from_str::<f64>(*step), from_str::<f64>(*last)) {
                (Some(first_f), Some(step_f), Some(last_f)) => (first_f, step_f, last_f),
                (f, s, _) => {
                    err_not_float(*first);
                    err_not_float(*(if f.is_none() {first} else if s.is_none() {step} else {last}));
                    return;
                }
            }
        }
        _ => {
            common::err_write_line("usage: seq [first [incr]] last");
            std::os::set_exit_status(1);
            return;
        }
    };

    if step == 0.0 {
        common::err_write_line("seq: zero increment");
        std::os::set_exit_status(1);
        return;
    }
    if first > last && step > 0.0 {
        common::err_write_line("seq: needs negative decrement");
        std::os::set_exit_status(1);
        return;
    }
    if first < last && step < 0.0 {
        common::err_write_line("seq: needs positive increment");
        std::os::set_exit_status(1);
        return;
    }
    
    let mut cur = first;
    while cur <= last {
        println!("{}", cur);
        cur += step;
    }
}
