#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// =======================
// STRUCT DATA
// =======================

#[contracttype]
#[derive(Clone, Debug)]
pub struct Post {
    pub id: u64,
    pub author: String,
    pub content: String,
    pub likes: u32,
    pub comments: Vec<Comment>,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Comment {
    pub author: String,
    pub content: String,
}

// =======================
// STORAGE KEY
// =======================

const POSTS: Symbol = symbol_short!("POSTS");

// =======================
// CONTRACT
// =======================

#[contract]
pub struct SocialContract;

#[contractimpl]
impl SocialContract {

    // =======================
    // GET ALL POSTS
    // =======================
    pub fn get_posts(env: Env) -> Vec<Post> {
        env.storage().instance().get(&POSTS).unwrap_or(Vec::new(&env))
    }

    // =======================
    // CREATE POST
    // =======================
    pub fn create_post(env: Env, author: String, content: String) -> String {
        let mut posts: Vec<Post> = env.storage().instance().get(&POSTS).unwrap_or(Vec::new(&env));

        let post = Post {
            id: env.prng().gen::<u64>(),
            author,
            content,
            likes: 0,
            comments: Vec::new(&env),
        };

        posts.push_back(post);
        env.storage().instance().set(&POSTS, &posts);

        String::from_str(&env, "Post berhasil dibuat 🚀")
    }

    // =======================
    // LIKE POST
    // =======================
    pub fn like_post(env: Env, post_id: u64) -> String {
        let mut posts: Vec<Post> = env.storage().instance().get(&POSTS).unwrap_or(Vec::new(&env));

        for i in 0..posts.len() {
            let mut post = posts.get(i).unwrap();

            if post.id == post_id {
                post.likes += 1;
                posts.set(i, post);

                env.storage().instance().set(&POSTS, &posts);
                return String::from_str(&env, "Post di-like ❤️");
            }
        }

        String::from_str(&env, "Post tidak ditemukan")
    }

    // =======================
    // COMMENT POST
    // =======================
    pub fn comment_post(env: Env, post_id: u64, author: String, content: String) -> String {
        let mut posts: Vec<Post> = env.storage().instance().get(&POSTS).unwrap_or(Vec::new(&env));

        for i in 0..posts.len() {
            let mut post = posts.get(i).unwrap();

            if post.id == post_id {
                let comment = Comment { author, content };
                post.comments.push_back(comment);

                posts.set(i, post);
                env.storage().instance().set(&POSTS, &posts);

                return String::from_str(&env, "Komentar ditambahkan 💬");
            }
        }

        String::from_str(&env, "Post tidak ditemukan")
    }

    // =======================
    // DELETE POST
    // =======================
    pub fn delete_post(env: Env, post_id: u64) -> String {
        let mut posts: Vec<Post> = env.storage().instance().get(&POSTS).unwrap_or(Vec::new(&env));

        for i in 0..posts.len() {
            if posts.get(i).unwrap().id == post_id {
                posts.remove(i);

                env.storage().instance().set(&POSTS, &posts);
                return String::from_str(&env, "Post dihapus 🗑️");
            }
        }

        String::from_str(&env, "Post tidak ditemukan")
    }
}

mod test;