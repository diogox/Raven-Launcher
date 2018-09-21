pub trait Query {

    fn get_keyword(&self) -> Option<&str>;
    fn get_argument(&self) -> Option<Vec<&str>>;

    /// Mode is active when query 
    /// starts with keyword + space.
    fn is_mode_active(&self) -> bool;
}

impl Query for String {

    fn get_keyword(&self) -> Option<&str> {
        let words = self.split(" ").collect::<Vec<&str>>();
        if words.len() >= 1 {
            return Some( words[0] );
        }
        None
    }

    fn get_argument(&self) -> Option<Vec<&str>> {
        let words = self.split(" ").collect::<Vec<&str>>();
        if words.len() >= 2 {
            return Some( words[1..].to_vec() );
        }
        None
    }

    fn is_mode_active(&self) -> bool {
        if let Some(kw) = self.get_keyword() {
            let expected = format!("{} ", kw);
            return self.starts_with(&expected);
        }
        false
    }
}