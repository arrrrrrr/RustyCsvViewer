
#[macro_export]
macro_rules! make_strvec {
    [ $($a:expr),+ ]
        =>
    {
        vec![ $($a.to_owned()),+ ]
    }
}