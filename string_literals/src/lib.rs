pub fn is_empty(v: &str) -> bool {
    return v.is_empty();
}

pub fn is_ascii(v: &str) -> bool {
    return v.is_ascii();
}

pub fn contains(v: &str, pat: &str) -> bool {
    return v.contains(pat);
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    return (&v[..index], &v[index..]);
}

pub fn find(v: &str, pat: char) -> usize {
    //t mut res = 0;
    for (i, v) in v.chars().enumerate(){
        if v == pat{
            return i;
        }
    }
    return 0;
}