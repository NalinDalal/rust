pub struct Post {
    //1. to hold some content
    state: Option<Box<dyn State>>,
    content: String,
}

//9.
pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    //3. Storing the Text of the Post Content

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    //4. Ensuring the Content of a Draft Post Is Empty
    pub fn content(&self) -> &str {
        "";
        &self.content
    }
    //5. functionality to request a review of a post to change status from Draft to PendingReview
    pub fn request_review(&mut self) {
        //consumes the current state and returns a new state.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    //6. Adding approve to Change the Behavior of content
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    //7. update the content method-> return type depend on the current state of the Post
    pub fn content(&self) -> &str {
        //return type is Option<&Box<dyn State>> cause state is Option<&Box<dyn State>>
        self.state.as_ref().unwrap().content(self) //as_ref cause we want a refernce rather than
                                                   //ownership
    }

    //9.
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

trait State {
    //2. define the behavior that all state objects for a Post must have.
    //defines the behavior shared by different post states.
    //Objects are: Draft, Pending Review, Published
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    //6. Adding approave method to Change Behavious of content
    fn approve(self: Box<Self>) -> Box<dyn State>;
    //call the approve method on a Draft, it will have no effect because approve will return self
    //call on PendingReview, it returns a new, boxed instance of the Published struct.

    //8. deref coercion will take effect on the & and the Box so the content method will ultimately be called on the type that implements the State trait.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    //5. functionality to request a review of a post to change status from Draft to PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    //6. Adding approave method to Change Behavious of content
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

//8. deref coercion will take effect on the & and the Box so the content method will ultimately be called on the type that implements the State trait.
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

//5. functionality to request a review of a post to change status from Draft to PendingReview
struct PendingReview {}

impl State for PendingReview {
    //5.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        //method is only valid when called on a Box holding the type
        //takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state.

        self
    }

    //6.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

//9.
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    //10.
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

//10.
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

//10. Encoding States and Behavior as Types
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
