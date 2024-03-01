# Backend
## Docs
### Install
 - Install postgresql on your system
```
# passwd postgres
$ su postgres
[postgres]$ initdb -D /var/lib/postgres/data
[postgres]$ exit
$ systemctl enable --now postgresql.service
$ su postgres
[postgres]$ createuser --interactive // important to name the user dr
[postgres]$ exit
$ createdb projekt-db -O dr
```
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
 
