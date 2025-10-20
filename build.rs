const ENV_FOLLOW: &[&str] = &[
    "SIN_TAYLOR_TERMS",
    "COS_TAYLOR_TERMS",
    "EXP_TAYLOR_TERMS",
    "ATAN_TAYLOR_TERMS",
    "SINH_TAYLOR_TERMS",
    "COSH_TAYLOR_TERMS",
    "LN_SERIES_TERMS",
];

fn main() {
    for env in ENV_FOLLOW {
        println!("cargo:rerun-if-env-changed={env}");
    }
}
