Source: https://actix.rs/


Tool for testing: https://insomnia.rest/

# Websocket Test
- WS_URL: ws://localhost:8080/api/ws
> Open Insomnia > Scratch Pad > Press add (+) right beside filter > WebSocket Request

# User Endpoint Test
  - URL (GET/POST): localhost:8080/api/users
> send POST - Send as JSON
```
{
	"username": "Jack",
	"email": "jack@gmail.com"
}
```
> send GET - retrieves data from the database.
```
[
	{
		"id": 1,
		"username": "Jack",
		"email": "jack@gmail.com"
	}
]
```
