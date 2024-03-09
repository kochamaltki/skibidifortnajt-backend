# Backend
## Docs
### Install
 - Install sqlite on your system
### Running
 - Before the first deploy, create a file `SECRET` in the root directory, with its content being a base64 secret
 - Run `./deploy.sh` from the root directory
### Acces points
#### /api/get/posts/by-user/{id}
 - Get: 200 (PostList) / 404 ("User not found")
 - Note: Only from user {id}
#### /api/get/posts/by-tag/{tag}
 - Get: 200 (PostList) / 404 ("Tag not found")
 - Note: Posts with tag {tag}
#### /api/get/posts/by-id/{id}
 - Get: 200 (Post) / 404 ("Post not found")
 - Note: Post with id {id}
#### /api/get/posts/all
 - Get: 200 (PostList)
#### /api/get/tags/from-post/{id}
 - Get: 200 (TagList) / 404 ("Post not found")
 - Note: All tags of post {id}
#### /api/get/user/name/{id}
 - Get: 200 (string) / 404 ("User not found")
 - Note: Get username of user {id} ("" if there is no such user) 
#### /api/get/user/id/{name}
 - Get: 200 (i64) / 404 ("User not found")
 - Note: Get id of user {name} (-1 if there is no such user)
#### /api/get/reactions/from-post/{id}
 - Get: 200 (ReactionCountMap) / 404 ("Post not found")
 - Note: Get (reaction, count) map from post {id}
#### /api/post/add-post
 - Post: PostCreateRequest
 - Effect: Adds a post to the db
 - Return: 201 ("Post created") / 401 ("Wrong token" / "User is banned") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/react
 - Post: ReactRequest
 - Effects: Adds reaction to a post
 - Return: 200 ("Reaction added") / 406 ("Reaction already exists")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/login
 - Post: LoginRequest
 - Effect: Login ig
 - Return: 200 (token) / 401 ("Password incorrect") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/signup
 - Post: SignupRequest
 - Effect: Creates a user with given name and password
 - Return: 201 (token) / 409 ("User already exists")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/delete-user
 - Post: UserDeleteRequest
 - Effect: Deletes a user
 - Return: 200 ("User deleted") / 401 ("Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/admin/post/upgrade-user
 - Post: UserUpgradeRequest
 - Effect: User with given id becomes an admin
 - Note: Token must belong to an admin
 - Return: 200 ("Upgrade succesful") / 401 ("User is not admin" / "Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/admin/post/ban-user
 - Post: UserBanRequest
 - Effect: User with given id is banned and their posts are deleted
 - Note: Token must belong to an admin
 - Return: 200 ("Ban succesful") / 401 ("User is not admin" / "Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
### Types
```
Post {
    post_id: i64
    user_id: i64
    date: i64
    body: string (max 2048 chars)
}
```
```
PostList {
    post_list: Vec<Post>
}
```
```
TagList {
    tag_list: Vec<string (max 64 chars)>
}
```
```
ReactionCountMap {
    reaction_count_map: map(i64, i64)
}
```
```
LoginRequest {
    user_name: string (max 64 chars)
    passwd: string (max 128 chars)
}
```
```
SignupRequest {
    user_name: string (max 64 chars)
    passwd: string (max 128 chars)
}
```
```
PostCreateRequest {
    body: string (max 2048 chars)
    token: string
}
```
```
ReactRequest {
    post_id: i64
    reaction_type: i64
    token: string
}
```
```
UserDeleteRequest {
    token: string
}
```
```
UserUpgradeRequest {
    user_id: i64
    token: string
}
```
```
UserBanRequest {
    user_id: i64
    token: string
}
```
