//! Intro to how traits are implemented
//! 
//! traits are used to define "shared behaviour" across
//! different TYPES
//! 
//! Traits allow us to define a set of methodes that
//! can be shared between multiple TYPES



pub struct NewArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}


impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{} {}", self.author, self. headline)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }
}


// Now we may want to implement function that can be
// shared by both struct Tweet and struct NewArticle

/// We now have the ability to SUMMARIZE BOTH 
/// Tweet and NewArticle (even thought they are of different TYPES)
pub trait Summary {
    // --> we only need to specify the fn signature
    // --> we don't specify fn body because we don't 
    //     want to dictate how the type "impl" the fn


    /// "reference to self" and returns a "string"
    fn summarize(&self) -> String ;
    
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

    println!("{}",tweet.summarize());
    println!("{}",news_article.summarize());

}