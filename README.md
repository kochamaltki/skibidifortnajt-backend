# Backend
## Docs
### Install
 - Install sqlite on your system
### Acces points
#### /api/get/posts/by-user/{id}
 - Get: list of **Post**
 - Note: Only from user {id}
#### /api/get/posts/all
 - Get: list of **Post**
#### /api/post
 - Post: 1x **Post**
 - Effect: Adds a post to the db
 - Headers: 'Content-Type: application/json' 'Content-Type: text/plain'
### Types
```
Post {
    user_id: i32
    body: string (max 2048 chars)
}
```
 
