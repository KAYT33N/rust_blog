All routes respond in this `JSON` form :

    {
	    "code":  (Integer) real status code ,
	    "status": (String) respective status text,
	    "response": (JSON Object) real response body
    }
-   `JSON` object is either `errors` or requested record (e.g. posts, tokens)
- stared (*) routes need auth with token
- to authorize set `AUTHORIZATION` header with the token you got from `login` route
# Routes :

## healthcheck
route:

    (GET)		/healthcheck
returns `200` `Ok` to check app availability 

## signup
route :

     (POST)		/users

request :

    {
		"username":"{{username}}",
		"password":"{{password}}"
	}
response (on success):

	...
		"response": {
			"user": {
				"id": {{user_id}},
				"username": "{{username}}"
			}
	...
response (on error) :

	...
		"response": {
			"errors": {
				"username": {{bool}},
				"password": {{bool}},
				"unexpected": {{bool}}
			}
	...
rules :
- username : 
	- ^\w{4,30}$
- password : 
	- should contain a-zA-Z
	- should contain 0-9
	- can have !$#@%
	- should have 8-40 characters 
## *whoami
route :

    (GET) 		/users
   response :
   

    
	...
		"response": {
			"id": {{token_id}},
			"user_id": {{user_id}}
		}
	...
## login
route :

    (POST)		/tokens
request :

    {
		"username":"{{usename}}",
		"password":"{{password}}"
	}
response (on success) :

	...
		"response": {
	   		"token": "{{your token}}"
	   	}
   	...
response (on error) :

    ...
	    "errors": {
			"unauthorized": {{bool}},
			"unexpected": {{bool}}
		}
	...
## *logout

    (DELETE)	/tokens
- response (on success) :
	- 202 with no body
- response (on error) :
	- 401 with no body
##		*posts_store
route :

    (POST)		/posts

request :

    {
		"parent_id": (Optional)(Defaults to 0){{Int}},
		"body":{{String}}
	}

response (on success) :

    ...
	    "response": {
			"post": {
				"id": {{Int}},
				"parent_id": {{Int}},
				"user_id": {{Int}},
				"body": {{String}},
				"created_at": {{Timestamp}}
			}
		}
	...

response (on error) :

	...
	    "errors": {
			"parent_id": {{bool}},
			"authed": false,
			"body": false,
			"unexpected": false
		}
	...
## posts_show_by_thread
route :

    (GET)		/posts/<post_id>

response :

    ...
    	"response": {
		    "posts": [
				{
					"id": {{post id}},
					"parent_id": {{post parent id}},
					"user_id": {{author id}},
					"username": "{{author username}}",
					"body": "{{post body}}",
					"created_at": "{{time}}",
					"replies": {{replies count}}
				},
				...requested post's first degree childs
			]
		}
	...

