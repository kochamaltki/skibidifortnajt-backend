# Backend
## Docs
### Install
 - Install sqlite (sqlite3, libsqlite3-dev packages)
 - Install gcc
 - Install rust
 - Install docker, docker compose (optional)
 - If you want to use the test scripts, install curl
 - Create file `secret.sql` in the root directory with the following content `INSERT INTO users VALUES (0, '{admin_user}', '{admin_display_name}', '{admin_desc}', '{admin_passwd_hash}', 1);`
### Running
 - Before the first deploy, create a file `SECRET` in the root directory, with its content being a base64 secret
 - Run `./scripts/deploy.sh` from the root directory
 - Docker: First `docker build -t backend .` and run `docker compose up` from the root directory
### Acces points
#### /api/get/posts/by-user/{id}/{limit}/{offset}
 - Get: 200 (PostList) / 404 ("User not found")
 - Note: Only from user {id}
#### /api/get/posts/by-id/{id}/{limit}/{offset}
 - Get: 200 (Post) / 404 ("Post not found")
 - Note: Post with id {id}
#### /api/get/posts/new/{limit}/{offset}
 - Get: 200 (PostList) sorted by date
#### /api/get/posts/top/{limit}/{offset}/{from_date}
 - Get: 200 (PostList) sorted by likes descending
#### /api/get/posts/bottom/{limit}/{offset}/{from_date}
 - Get: 200 (PostList) sorted by likes ascending
#### /api/get/posts/trending/{limit}/{offset}/{from_date}
 - Get: 200 (PostList) sorted by (likes / age in minutes)
#### /api/get/posts/from-search/{search-phrase}/{limit}/{offset}/{from_date}
 - Get: 200 (PostList)
```
Post {
    post_id: i64
    user_id: i64
    date: i64
    body: string (max 2048 chars)
    likes: i64
    user_name: string
    display_name: string
    pfp_image: string
}
```
```
PostList {
    post_list: Vec<Post>
}
```
#### /api/get/users/from-search/{search-phrase}/{limit}/{offset}
 - Get: 200 (ProfileList)
```
Profile {
    user_id: i64
    user_name: String
    display_name: String
    description: String
    pfp_image: String
}
```
```
ProfileList {
    profile_list: Vec<Profile>
}
```
#### /api/get/tags/from-post/{id}
 - Get: 200 (TagList) / 404 ("Post not found")
 - Note: All tags of post {id}
```
TagList {
    tag_list: Vec<string (max 64 chars)>
}
```
#### /api/get/user/name/{id}
 - Get: 200 (string) / 404 ("User not found")
 - Note: Get username of user {id} 
#### /api/get/user/id/{name}
 - Get: 200 (i64) / 404 ("User not found")
 - Note: Get id of user {name} 
#### /api/get/profile/by-id/{id}
 - Get: 200 (Profile) / 404 ("User not found")
 - Note: Get user profile
```
Profile {
    user_id: i64
    user_name: string (max 64 chars)
    display_name: string (max 64 chars)
    description: string (max 2048 chars)
}
```
 #### /api/get/likes/from-post/{id}
 - Get: 200 (LikeCount) / 404 ("Post not found")
 - Note: Get the number of likes from post {id}
```
LikeCount {
    like_count: i64
}
```
 #### /api/get/images/from-post/{id}
 - Get: 200 (ImageList) / 404 ("Post not found")
 - Note: Get a list of image names used to acces them via the call below
```
ImageList {
    image_list: Vec<string (max 64 chars)>
}
```
 #### /api/get/image/{image-name}
 - Get: Image
 #### /api/get/comments/{id}
 - Get: 200 (CommentList) / 404 ("Post not found")
 - Note: Get a list of comments from post {id}
#### /api/post/add-post
 - Post: 
```
PostCreateRequest {
    body: string (max 2048 chars)
    tags: Vec<string (max 64 chars)>
}
```
 - With cookies
 - Effect: Adds a post to the db
 - Return: 201 ({post_id:i64}) / 401 ("Wrong token" / "User is banned") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/comment
 - Post: 
```
CommentCreateRequest {
    post_id: i64
    body: string (max 2048 chars)
}
```
 - With cookies
 - Effect: Adds a comment to the post
 - Return: 201 ({comment_id:i64}) / 401 ("Wrong token" / "User is banned") / 404 ("User not found" / "Post not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/react
 - Post: 
```
LikeRequest {
    post_id: i64
}
```
 - With cookies
 - Effects: Adds like to a post
 - Return: 200 ("Like added") / 406 ("Like already exists")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/login
 - Post: 
```
LoginRequest {
    user_name: string (max 64 chars)
    passwd: string (max 128 chars)
    remember_password: bool
}
```
 - Effect: Login ig
 - Return: 200 (token) / 401 ("Password incorrect") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/signup
 - Post: 
```
SignupRequest {
    user_name: string (max 64 chars)
    passwd: string (max 128 chars)
    remember_password: bool
}
```
 - Effect: Creates a user with given name and password
 - Return: 201 (token) / 409 ("User already exists")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/delete-user
 - Post: 
```
UserDeleteRequest {
    user_id: i64
}
```
 - With cookies
 - Effect: Deletes a user
 - Return: 200 ("User deleted") / 401 ("Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/admin/post/upgrade-user
 - Post: 
```
UserUpgradeRequest {
    user_id: i64
}
```
 - With cookies
 - Effect: User with given id becomes an admin
 - Note: Token must belong to an admin
 - Return: 200 ("Upgrade succesful") / 401 ("User is not admin" / "Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/admin/post/ban-user
 - Post: 
```
UserBanRequest {
    user_id: i64
    ban_length: i64,
    ban_message: string,
}
```
 - With cookies
 - Effect: User with given id is banned
 - Note: Token must belong to an admin
 - Return: 200 ("Ban succesful") / 401 ("User is not admin" / "Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/admin/post/unban-user
 - Post: 
```
UserUnbanRequest {
    user_id: i64,
}
```
 - With cookies
 - Effect: User with given id is unbanned
 - Note: Token must belong to an admin
 - Return: 200 ("Unban succesful") / 401 ("User is not admin" / "Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/change/display-name
 - Post: 
```
DisplayNameChangeRequest {
    new_display_name: string (max 64 chars)
}
```
 - With cookies
 - Effect: User's display name changes
 - Return: 200 ("Change succesful") / 401 ("Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/change/user-name
 - Post: 
```
UserNameChangeRequest {
    new_user_name: string (max 64 chars)
}
```
 - With cookies
 - Effect: User's display name changes
 - Return: 200 ("Change succesful") / 401 ("Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/change/description
 - Post: 
```
DescriptionChangeRequest {
    new_description: string (max 64 chars)
}
```
 - With cookies
 - Effect: User's description changes
 - Return: 200 ("Change succesful") / 401 ("Wrong token") / 404 ("User not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/upload/image
 - Post: Image (max 25MB)
 - With cookies
 - Return: 200 (image-id) / 400 ("Invalid image format" / "File type error" / ) / 401 ("Wrong token") / 500 ("File read error")
 - Headers: 'Content-Type: multipart/form-data', 'auth: {user_token}'
#### /api/post/add-image-to-post
 - Post:
```
AddImageToPostRequest {
    image_id: i64,
    post_id: i64
}
```
 - With cookies
 - Effect: Image is added to post
 - Return: 200 ("Image added to post") / 400 ("Image already added to this post") / 401 ("Wrong token" / "User not authorized") / 404 ("Image not found" / "Post not found")
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
#### /api/post/set-pfp
 - Post:
```
SetPFPRequest {
    image_id: i64,
    user_id: i64
}
```
 - With cookies
 - Effect: User's PFP is set to the image
#### /api/post/remove-pfp
 - Post:
```
RemovePFPRequest {
    user_id: i64
}
```
 - With cookies
 - Effect: User's PFP is deleted
