//! Intro to how traits are implemented
//! 
//! traits are used to define "shared behaviour" across
//! different TYPES
//! 
//! Traits allow us to define a set of methodes that
//! can be shared between multiple TYPES



//<-------------------------------------------------------------------
pub struct NewArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}


impl Summary for NewArticle {
    fn summarize_1(&self) -> String {
        format!("{} {}", self.author, self. headline)
    }

    /// we are overriding the default summarize_2
    fn summarize_2(&self) -> String {
        String::from("THIS OVER-RODE DEFAULT")
    }
}
//<-------------------------------------------------------------------



//<-------------------------------------------------------------------
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_1(&self) -> String {
        format!("{} {}", self.username, self.content)
    }

    // remember this - in syntax we don't 
    // have to state the trait fn in impl block
    // learn more about the syntax. Why pub? etc
    // fn summarize_2(&self) -> String;
}
//<-------------------------------------------------------------------




// Now we may want to implement function that can be
// shared by both struct Tweet and struct NewArticle

/// We now have the ability to SUMMARIZE BOTH 
/// Tweet and NewArticle (even thought they are of different TYPES)
pub trait Summary {
    /*
    --> we only need to specify the fn signature
    --> we don't specify fn body because we don't 
        want to dictate how the type "impl" the fn
    */

    /// "reference to self" and returns a "string"
    fn summarize_1(&self) -> String ;
    
    /*
    sometimes we may need that all trait functions have 
    certain DEFAULT IMPLEMENTATION for body.
    These can be overridden IF wewant to design impl ourself
    */
    fn summarize_2(&self) -> String {
        String::from("DEFAULT IMPLEMENTATION OF summarize_2 ")
    }
    
}





fn main(){
    let news_article = NewArticle{
        author: "JK".to_owned(),
        headline: "Trans and Women".to_owned(),
        content: "WOMEN v TRANS".to_string()

    };
    
    let tweet = Tweet{
        username: "Me".to_owned(),
        content: "dumb".to_string(),
        reply: true,
        retweet: false
    };

    // instances share same functionality
    println!("{}",tweet.summarize_1());
    println!("{}",news_article.summarize_1());

    // tweet default impl of trait summarize_2
    // news_article has it's own impl of trait summarize_2 
    println!("{}",tweet.summarize_2());
    println!("{}",news_article.summarize_2());

}