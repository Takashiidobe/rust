use rustc_abi::TyAbiInterface;

use crate::callconv::{ArgAbi, FnAbi};

fn classify_ret_ty<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    if ret.layout.is_aggregate() {
        ret.make_indirect();
    }
}

fn classify_arg_ty<'a, Ty, C>(cx: &C, arg: &mut ArgAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if arg.layout.pass_indirectly_in_non_rustic_abis(cx) || arg.layout.is_aggregate() {
        arg.make_indirect();
    }
}

pub(crate) fn compute_abi_info<'a, Ty, C>(cx: &C, fty: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if !fty.ret.is_ignore() {
        classify_ret_ty(&mut fty.ret);
    }

    for arg in fty.args.iter_mut() {
        if !arg.is_ignore() {
            classify_arg_ty(cx, arg);
        }
    }
}
