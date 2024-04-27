macro_rules! enum_expand_inner_match {
    { match $value:ident for $inner:ident in [$($variant:path,)+] {
        => $target:expr,
        _ => $default:expr$(,)?
    }}=> {
        match $value {
            $($variant($inner) => $target,)+
            _ => $default,
        }
    };
    { match $value:ident for $inner:ident in [$($variant:path,)+] {
        => $target:expr$(,)?
    }}=> {
        match $value {
            $($variant($inner) => $target,)+
        }
    };
}
pub(crate) use enum_expand_inner_match;

macro_rules! enum_wrap_inner_fn {
    { { $(#[$meta:meta])* $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&mut self$(, $arg:ident: $arg_type:ty)*) $(-> $($return_value:tt)+)? } for [$($variant:path,)+]} => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&mut self$(, $arg: $arg_type)*) $(-> $($return_value)+)? {
            $crate::macros::enum_expand_inner_match! {
                match self for variant in [$($variant,)+] {
                    => variant.$fn_name($($arg),*),
                }
            }
        }
    };
    { { $(#[$meta:meta])* $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&self$(, $arg:ident: $arg_type:ty)*) $(-> $($return_value:tt)+)? } for [$($variant:path,)+]} => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&self$(, $arg: $arg_type)*) $(-> $($return_value)+)? {
            $crate::macros::enum_expand_inner_match! {
                match self for variant in [$($variant,)+] {
                    => variant.$fn_name($($arg),*),
                }
            }
        }
    };
    { { $(#[$meta:meta])* $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(self$(, $arg:ident: $arg_type:ty)*) $(-> $($return_value:tt)+)? } for [$($variant:path,)+]} => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(self$(, $arg: $arg_type)*) $(-> $($return_value)+)? {
            $crate::macros::enum_expand_inner_match! {
                match self for variant in [$($variant,)+] {
                    => variant.$fn_name($($arg),*),
                }
            }
        }
    };
}
pub(crate) use enum_wrap_inner_fn;
