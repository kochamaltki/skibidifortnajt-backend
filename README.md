# Backend
## Docs
### Acces points
#### /api/get/posts/by-user/{id}
 Get: multiple **Message**
 Note: Only from user {id}
#### /api/get/posts/all
 Get: multiple **Message**
#### /api/post
 Post: 1x **Message**
 Effect: Adds a message to the db
### Types
```
Message {
    user_id: i32
    body: string (max 2048 chars)
}
```
 
