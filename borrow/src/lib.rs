pub fn str_len(s: &str) -> usize {
    let mut count = 0;
   let bytes = s.chars();
   for _ in bytes{
        count+= 1
    }
    count
}