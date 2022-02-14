pub struct OtherPost {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl OtherPost {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> UnApprovedPost {
        UnApprovedPost {
            content: self.content,
        }
    }
}

pub struct UnApprovedPost {
    content: String,
}

impl UnApprovedPost {
    pub fn approve(self) -> OnceApprovedPost {
        OnceApprovedPost {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

pub struct OnceApprovedPost {
    content: String,
}

impl OnceApprovedPost {
    pub fn approve(self) -> OtherPost {
        OtherPost {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
