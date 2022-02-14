pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approvals: u32,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approvals: 0,
        }
    }

    pub fn status(&self) -> String {
        self.state.as_ref().unwrap().status()
    }

    pub fn add_text(&mut self, text: &str) {
        let changes = self.state.as_ref().unwrap().changes(text);

        self.content.push_str(changes);
    }

    pub fn content(&self) -> &str {
        // we make the state object responsible for displaying the content
        // even though it is stored on Post. This is so the state objects can
        // be entirely responsible for managing the implications of a given
        // state. The advantage is that this function never needs to change if
        // were to decide, for instance, that posts that are "PendingReview"
        // can also print their content
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(self))
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    // having self be an owned parameter drops the old state we were holding
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // It would be nice to create default implementations of #request_review
    // and #approve as most State types just return self. This requires,
    // however, that the compiler knows the size of self at compile time.
    // Since we are creating trait object with State, this won't compile,
    // because State objects can be of arbitrary type. So we won't know
    // the size of self at compile time.
    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn status(&self) -> String;

    fn changes<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>, _post: &mut Post) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn status(&self) -> String {
        "draft".to_string()
    }

    fn changes<'a>(&self, text: &'a str) -> &'a str {
        text
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        match post.approvals {
            0 => {
                post.approvals = 1;
                self
            }
            _ => {
                post.approvals = 2;
                Box::new(Published {})
            }
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn status(&self) -> String {
        "pending review".to_string()
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, _post: &mut Post) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn status(&self) -> String {
        "published".to_string()
    }
}
