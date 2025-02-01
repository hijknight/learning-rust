use blog::{PendingReviewPost, Post};

fn main() {
    let mut post = Post::new(); // content is ""

    post.add_text("I ate salad");

    let post: PendingReviewPost = post.request_review();

    let post: Post = post.approve();

    assert_eq!("I ate salad", post.content());
}
