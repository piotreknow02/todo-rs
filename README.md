# todo-rs

Simple todo app in rust and rocket 🦀🚀

---

### Endpoints

- `GET /` - list all items

- `POST /` - add new item

- `GET /<id>/done` - mark item as done

- `DELETE /<id>` - delete item

### Config

This project requires correctly setup environment variables simlar to this in `.env.example` file

> You can just copy-paste `.env.example` to `.env` if you're running on docker

### Running on docker

To run this project just type 

```bash
docker-compose up --build
```

### Running locally

To run this project in your local environment you need to launch a mongodb instance. (you can also use `dev-docker-compose.yml` file)

Then just type

```bash
cargo run
```


