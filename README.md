# Backend
## Docs
### Install
 - Install sqlite on your system
### Running
 - Before the first deploy, create a file `SECRET` in the root directory, with its content being a base64 secret
 - Run `./deploy.sh` from the root directory
### Acces points
#### /api/get/posts/by-user/{id}
 - Get: PostList
 - Note: Only from user {id}
#### /api/get/posts/by-tag/{tag}
 - Get: PostList
 - Note: Posts with tag {tag}
#### /api/get/posts/by-id/{id}
 - Get: Post
 - Note: Post with id {id}
#### /api/get/posts/all
 - Get: PostList
#### /api/get/tags/from-post/{id}
 - Get: TagList
 - Note: All tags of post {id}
#### /api/get/user/name/{id}
 - Get: string
 - Note: Get username of user {id} ("" if there is no such user) 
#### /api/get/user/id/{name}
 - Get: i64
 - Note: Get id of user {name} (-1 if there is no such user)
#### /api/post/add-post
 - Post: Post
 - Effect: Adds a post to the db
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/login
 - Post: LoginRequest
 - Effect: Login ig
 - Return: User token
#### /api/post/signup
 - Post: SignupRequest
 - Effect: Creates a user with given name and password
 - Return: User Token
#### /api/post/delete-user
 - Post: UserDeleteRequest
 - Effect: Deletes a user
#### /api/admin/post/upgrade-user
 - Post: UserUpgradeRequest
 - Effect: User with given id becomes an admin
 - Note: Token must belong to an admin
#### /api/admin/post/ban-user
 - Post: UserBanRequest
 - Effect: User with given id is banned and their posts are deleted
 - Note: Token must belong to an admin
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
