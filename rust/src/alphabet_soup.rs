pub fn alphabet_soup(input: &str) -> String {

    let mut buf = String::with_capacity(input.len());
    for c in input.chars() {
       println!("{:?}", c);
    //    to_ascii_lowercase
       if c != ' ' {
          buf.push(c);
       }
    }
    buf.v.sort_unstable();
}
#[cfg(test)]
mod tests {
    use super::alphabet_soup;
    #[test]
    fn test_alphabet_soup() {
        assert_eq!(alphabet_soup("hello"),"ehllo");
        assert_eq!(alphabet_soup("edabit"),"abdeit");
        assert_eq!(alphabet_soup("hacker"),"acehkr");
        assert_eq!(alphabet_soup("geek"),"eegk");
        assert_eq!(alphabet_soup("javascript"),"aacijprstv");
        assert_eq!(alphabet_soup("credibility"),"bcdeiiilrty");
        assert_eq!(alphabet_soup("apostrophe"),"aehoopprst");
        assert_eq!(alphabet_soup("determination"),"adeeiimnnortt");
        assert_eq!(alphabet_soup("win"),"inw");
        assert_eq!(alphabet_soup("synthesis"),"ehinsssty");
    }
}
