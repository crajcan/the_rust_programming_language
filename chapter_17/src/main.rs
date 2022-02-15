#![allow(dead_code)]

extern crate chapter_17;
use chapter_17::other_post::OtherPost;
use chapter_17::post::Post;
use chapter_17::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a SelectBox
    }
}

fn use_gui_example() {
    let select_box = SelectBox {
        width: 180,
        height: 20,
        options: vec!["blue".to_string(), "red".to_string()],
    };

    let button = Button {
        width: 180,
        height: 20,
        label: "Search".to_string(),
    };

    let screen = Screen {
        components: vec![Box::new(button), Box::new(select_box)],
    };

    screen.run();
}

fn use_blog_post_example() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    assert_eq!(post.status(), "draft".to_string());

    post.request_review();
    assert_eq!("", post.content());
    assert_eq!(post.status(), "pending review".to_string());

    post.reject();
    assert_eq!("", post.content());
    assert_eq!(post.status(), "draft".to_string());

    post.request_review();
    assert_eq!("", post.content());
    assert_eq!(post.status(), "pending review".to_string());

    post.add_text(", it was fine.");

    post.approve();
    assert_eq!("", post.content());
    assert_eq!(post.status(), "pending review".to_string());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    assert_eq!(post.status(), "published".to_string());
}

fn use_other_post_example() {
    let mut post = OtherPost::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.reject();

    let post = post.request_review();

    let post = post.approve();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

#[derive(Debug)]
struct StringHolder {
    val: Option<String>,
}

fn mutate_field_of_owned_struct(holder: &mut StringHolder) {}

fn use_take() {
    let mut holder = StringHolder {
        val: Some("Hello".to_string()),
    };
    let holder_ref = &mut holder;

    if let Some(s) = holder_ref.val.take() {
        holder_ref.val = Some(s + " world!")
    }

    println!("holder: {:?}", holder);
}

fn call_method_on_field_of_owned_struct(holder: &StringHolder) -> usize {
    holder.val.as_ref().unwrap().capacity()
}

fn use_as_ref() {
    let holder = StringHolder {
        val: Some("Hello World".to_string()),
    };
    let holder_ref = &holder;

    let c = holder_ref.val.as_ref().unwrap().capacity();

    println!("capacity of string: {}", c);
}

fn mut_from_owned() {
    let holder = StringHolder {
        val: Some("Hello World".to_string()),
    };

    // not declared as mutable
    // holder.val = (Some("no way".to_string()));

    let mut holder_ref = holder;
    holder_ref.val = Some("no way".to_string());

    // borrow after move
    // println!("holder: {:?}", holder)

    // reclaim ownership
    let holder = holder_ref;
    println!("holder: {:?}", holder)
}

fn main() {
    use_gui_example();
    use_blog_post_example();
    use_other_post_example();
    use_take();
    use_as_ref();
    mut_from_owned();
}
