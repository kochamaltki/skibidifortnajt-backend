# Backend
## Docs
### Install
 - Install sqlite (sqlite3, libsqlite3-dev packages)
 - Install gcc
 - Install rust
 - Install docker, docker compose (optional)
 - If you want to use the test scripts, install curl
 - Create media/image, media/profile_pictures
### Running
 - Before the first deploy, create a file `SECRET` in the root directory, with its content being a base64 secret
 - Run `./deploy.sh` from the root directory
 - Docker: First `docker build -t backend .` and run `docker compose up` from the root directory
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
 - Note: Get username of user {id} 
#### /api/get/user/id/{name}
 - Get: 200 (i64) / 404 ("User not found")
 - Note: Get id of user {name} 
#### /api/get/profile/by-id/{id}
 - Get: 200 (Profile) / 404 ("User not found")
 - Note: Get user profile
 #### /api/get/likes/from-post/{id}
 - Get: 200 (LikeCount) / 404 ("Post not found")
 - Note: Get (like, count) map from post {id}
 #### /api/get/images/from-post/{id}
 - Get: 200 (ImageList) / 404 ("Post not found")
 - Note: Get a list of image names used to acces them via the call below
 #### /api/get/image/{image-name}
 - Get: Image
#### /api/post/add-post
 - Post: PostCreateRequest
 - Effect: Adds a post to the db
 - Return: 201 ("Post created") / 401 ("Wrong token" / "User is banned") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/react
 - Post: LikeRequest
 - Effects: Adds like to a post
 - Return: 200 ("Like added") / 406 ("Like already exists")
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
#### /api/post/change/display-name
 - Post: DisplayNameChangeRequest
 - Effect: User's display name changes
 - Return: 200 ("Ban succesful") / 401 ("Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/upload/image
 - Post: Image (max 25MB)
 - Return: 200 (image-id) / 400 ("Invalid image format" / "File type error" / ) / 500 ("File read error")
 - Headers: 'Content-Type: multipart/form-data'
#### /api/post/add-image-to-post
 - Post: AddImageToPostRequest
 - Effect: Image is added to post
 - Return: 200 ("Image added to post") / 400 ("Image already added to this post") / 401 ("Wrong token" / "User not authorized") / 404 ("Image not found" / "Post not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
### Types
```
Post {
    post_id: i64
    user_id: i64
    date: i64
    body: string (max 2048 chars)
    likes: i64
}
```
```
Profile {
    user_id: i64
    user_name: string (max 64 chars)
    display_name: string (max 64 chars)
    description: string (max 2048 chars)
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
ImageList {
    image_list: Vec<string (max 64 chars)>
}
```
```
LikeCount {
    like_count: i64
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
    tags: Vec<string (max 64 chars)>
    token: string
}
```
```
LikeRequest {
    post_id: i64
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
```
DisplayNameChangeRequest {
    new_display_name: string (max 64 chars)
    token: string
}
```
```
AddImageToPostRequest {
    token: string,
    image_id: i64,
    post_id: i64
}
```
