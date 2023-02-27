/// Doing a ~ constant time comparision for
pub(crate) fn compare_input(a : &untrusted::Input, b : &untrusted::Input) -> bool {
    // This should be done in ~ constant time.
    let mut equal = a.len() == b.len();

    let a = a.as_slice_less_safe();
    let b = b.as_slice_less_safe();

    let max_len = a.len().max(b.len());
    for i in 0..max_len {
        let a = a.get(i).cloned();
        let b = b.get(i).cloned();
        equal &= a == b;
    }

    equal
} 